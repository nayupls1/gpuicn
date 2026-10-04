//! A date field component that allows users to enter and edit date.
//! Includes a tiny, dependency-free [`Date`] type.
//!
//! ```ignore
//! Calendar::new("cal").on_change(|selection, _, _| println!("{selection:?}"))
//! Calendar::new("range").mode(CalendarMode::Range).months(2)
//! ```

use gpui::{
    App, ElementId, FontWeight, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px, relative,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

/// A calendar date (proleptic Gregorian).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: i32,
    /// 1–12
    pub month: u32,
    /// 1–31
    pub day: u32,
}

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September", "October",
    "November", "December",
];

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Today in UTC.
    pub fn today() -> Self {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        Self::from_days(secs.div_euclid(86_400))
    }

    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            _ if Self::is_leap_year(year) => 29,
            _ => 28,
        }
    }

    /// Days since 1970-01-01 (Howard Hinnant's algorithm).
    pub fn to_days(self) -> i64 {
        let y = if self.month <= 2 { self.year - 1 } else { self.year } as i64;
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let m = self.month as i64;
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + self.day as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    pub fn from_days(days: i64) -> Self {
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        let year = (yoe + era * 400 + if month <= 2 { 1 } else { 0 }) as i32;
        Self { year, month, day }
    }

    /// 0 = Sunday … 6 = Saturday.
    pub fn weekday(self) -> u32 {
        (self.to_days() + 4).rem_euclid(7) as u32
    }

    pub fn add_days(self, days: i64) -> Self {
        Self::from_days(self.to_days() + days)
    }

    /// Same day in another month, clamped to that month's length.
    pub fn add_months(self, months: i32) -> Self {
        let total = self.year * 12 + self.month as i32 - 1 + months;
        let (year, month) = (total.div_euclid(12), total.rem_euclid(12) as u32 + 1);
        Self { year, month, day: self.day.min(Self::days_in_month(year, month)) }
    }

    pub fn first_of_month(self) -> Self {
        Self { day: 1, ..self }
    }

    pub fn month_name(self) -> &'static str {
        MONTHS[(self.month - 1) as usize]
    }

    /// "October 4, 2026"
    pub fn format_long(self) -> String {
        format!("{} {}, {}", self.month_name(), self.day, self.year)
    }

    /// "Oct 04, 2026"
    pub fn format_short(self) -> String {
        format!("{} {:02}, {}", &self.month_name()[..3], self.day, self.year)
    }

    /// "2026-10-04"
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    #[default]
    Single,
    Range,
    Multiple,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CalendarSelection {
    Single(Option<Date>),
    Range(Option<Date>, Option<Date>),
    Multiple(Vec<Date>),
}

impl CalendarSelection {
    fn empty(mode: CalendarMode) -> Self {
        match mode {
            CalendarMode::Single => Self::Single(None),
            CalendarMode::Range => Self::Range(None, None),
            CalendarMode::Multiple => Self::Multiple(Vec::new()),
        }
    }

    pub fn single(&self) -> Option<Date> {
        match self {
            Self::Single(date) => *date,
            Self::Range(start, _) => *start,
            Self::Multiple(dates) => dates.first().copied(),
        }
    }

    fn contains(&self, date: Date) -> bool {
        match self {
            Self::Single(d) => *d == Some(date),
            Self::Range(Some(start), Some(end)) => *start <= date && date <= *end,
            Self::Range(start, _) => *start == Some(date),
            Self::Multiple(dates) => dates.contains(&date),
        }
    }

    fn toggle(&self, date: Date) -> Self {
        match self {
            Self::Single(d) if *d == Some(date) => Self::Single(None),
            Self::Single(_) => Self::Single(Some(date)),
            Self::Range(Some(start), None) if date >= *start => Self::Range(Some(*start), Some(date)),
            Self::Range(Some(start), None) => Self::Range(Some(date), Some(*start)),
            Self::Range(_, _) => Self::Range(Some(date), None),
            Self::Multiple(dates) => {
                let mut dates = dates.clone();
                match dates.iter().position(|d| *d == date) {
                    Some(ix) => {
                        dates.remove(ix);
                    }
                    None => dates.push(date),
                }
                Self::Multiple(dates)
            }
        }
    }
}

struct CalendarState {
    month: Date,
    selection: CalendarSelection,
}

#[derive(IntoElement)]
pub struct Calendar {
    id: ElementId,
    mode: CalendarMode,
    months: usize,
    selection: Option<CalendarSelection>,
    default_month: Option<Date>,
    disabled: Option<Rc<dyn Fn(Date) -> bool>>,
    on_change: Option<Rc<dyn Fn(&CalendarSelection, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Calendar);

impl Calendar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            mode: CalendarMode::Single,
            months: 1,
            selection: None,
            default_month: None,
            disabled: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    /// Number of months shown side by side (`numberOfMonths`).
    pub fn months(mut self, months: usize) -> Self {
        self.months = months.max(1);
        self
    }

    /// Controlled selection.
    pub fn selection(mut self, selection: CalendarSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    /// Controlled single date.
    pub fn selected(self, date: Option<Date>) -> Self {
        self.selection(CalendarSelection::Single(date))
    }

    /// Month shown initially (defaults to the selection or today).
    pub fn default_month(mut self, month: Date) -> Self {
        self.default_month = Some(month);
        self
    }

    /// Dates for which this returns true can't be picked.
    pub fn disabled(mut self, predicate: impl Fn(Date) -> bool + 'static) -> Self {
        self.disabled = Some(Rc::new(predicate));
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&CalendarSelection, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Calendar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mode = self.mode;
        let initial_month = self
            .default_month
            .or_else(|| self.selection.as_ref().and_then(|s| s.single()))
            .unwrap_or_else(Date::today)
            .first_of_month();
        let initial_selection = self.selection.clone().unwrap_or_else(|| CalendarSelection::empty(mode));
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| CalendarState {
            month: initial_month,
            selection: initial_selection,
        });
        let (month, selection) = {
            let s = state.read(cx);
            (s.month, self.selection.clone().unwrap_or_else(|| s.selection.clone()))
        };
        let theme = cx.theme();
        let today = Date::today();
        let cell = px(32.);
        let radius = theme.radius_md();

        let nav = |id: &'static str, icon: IconName, delta: i32| {
            let state = state.clone();
            div()
                .id(id)
                .flex()
                .size(cell)
                .items_center()
                .justify_center()
                .rounded(radius)
                .cursor_pointer()
                .hover(|s| s.bg(theme.accent))
                .child(Icon::new(icon))
                .on_click(move |_, _, cx| {
                    state.update(cx, |s, cx| {
                        s.month = s.month.add_months(delta);
                        cx.notify();
                    })
                })
        };

        let months = (0..self.months).map(|offset| {
            let first = month.add_months(offset as i32);
            let start = first.add_days(-(first.weekday() as i64));
            let days_in_month = Date::days_in_month(first.year, first.month);
            let weeks = (first.weekday() + days_in_month).div_ceil(7);

            let header = div()
                .relative()
                .flex()
                .h(cell)
                .items_center()
                .justify_center()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .child(format!("{} {}", first.month_name(), first.year))
                .when(offset == 0, |this| this.child(div().absolute().left_0().child(nav("prev", IconName::ChevronLeft, -1))))
                .when(offset + 1 == self.months, |this| {
                    this.child(div().absolute().right_0().child(nav("next", IconName::ChevronRight, 1)))
                });

            let weekdays = div().flex().children(["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].map(|d| {
                div()
                    .w(cell)
                    .text_center()
                    .text_size(px(12.8))
                    .text_color(theme.muted_foreground)
                    .child(d)
            }));

            let rows = (0..weeks).map(|week| {
                div().flex().mt_2().children((0..7).map(|weekday| {
                    let date = start.add_days((week * 7 + weekday) as i64);
                    let outside = date.month != first.month;
                    let is_disabled = self.disabled.as_ref().is_some_and(|f| f(date));
                    let selected = !outside && selection.contains(date);
                    let (range_start, range_end, in_range) = match &selection {
                        CalendarSelection::Range(Some(s), Some(e)) => (date == *s, date == *e, *s < date && date < *e),
                        CalendarSelection::Range(Some(s), None) => (date == *s, date == *s, false),
                        _ => (false, false, false),
                    };
                    let is_today = date == today;
                    let state = state.clone();
                    let on_change = self.on_change.clone();
                    let current = selection.clone();

                    let mut el = div()
                        .id(("day", date.to_days() as u64))
                        .flex()
                        .size(cell)
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .rounded(radius)
                        .when(outside, |this| this.text_color(theme.muted_foreground))
                        .when(is_today && !selected, |this| this.bg(theme.accent).text_color(theme.accent_foreground));
                    if selected && !outside {
                        el = if mode == CalendarMode::Range && in_range {
                            el.rounded_none().bg(theme.accent).text_color(theme.accent_foreground)
                        } else {
                            el.bg(theme.primary).text_color(theme.primary_foreground)
                        };
                        if mode == CalendarMode::Range && (range_start || range_end) && !(range_start && range_end) {
                            el = if range_start { el.rounded_r(px(0.)) } else { el.rounded_l(px(0.)) };
                        }
                    }
                    el.when(is_disabled, |this| this.opacity(0.5))
                        .when(!is_disabled && !outside, |this| {
                            this.cursor_pointer()
                                .when(!selected, |this| this.hover(|s| s.bg(theme.accent).text_color(theme.accent_foreground)))
                                .on_click(move |_, window, cx| {
                                    let next = current.toggle(date);
                                    state.update(cx, |s, cx| {
                                        s.selection = next.clone();
                                        cx.notify();
                                    });
                                    if let Some(handler) = &on_change {
                                        handler(&next, window, cx);
                                    }
                                })
                        })
                        .child(date.day.to_string())
                }))
            });

            div().flex().flex_col().gap_4().child(header).child(div().flex().flex_col().child(weekdays).children(rows))
        });

        div()
            .id(self.id)
            .flex()
            .gap_4()
            .p_3()
            .line_height(relative(1.))
            .refine_style(&self.style)
            .children(months)
    }
}

/// Convenience to render a short label for a selection.
pub fn selection_label(selection: &CalendarSelection) -> Option<SharedString> {
    match selection {
        CalendarSelection::Single(Some(date)) => Some(date.format_long().into()),
        CalendarSelection::Range(Some(start), Some(end)) => {
            Some(format!("{} - {}", start.format_short(), end.format_short()).into())
        }
        CalendarSelection::Range(Some(start), None) => Some(start.format_short().into()),
        CalendarSelection::Multiple(dates) if !dates.is_empty() => Some(format!("{} dates", dates.len()).into()),
        _ => None,
    }
}

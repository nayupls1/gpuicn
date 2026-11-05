//! A date field component that allows users to enter and edit date.
//! Includes a tiny, dependency-free [`Date`] type.
//!
//! ```ignore
//! <Calendar onChange={move |selection: &CalendarSelection| println!("{selection:?}")} />
//! <Calendar mode={CalendarMode::Range} months={2} />
//! <Calendar selected={day.get()} onChange={move |s: &CalendarSelection| day.set(s.single())} />
//! <Calendar disabled={|d: Date| d.weekday() == 0 || d.weekday() == 6} />
//! ```

use std::rc::Rc;

use rsx::prelude::*;
use rsx::{Div, Style};

use super::icon::{Icon, IconName};
use super::theme::theme;

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

/// Which dates can't be picked: `disabled={|d: Date| d < Date::today()}`.
#[derive(Clone)]
pub struct DateFilter(Rc<dyn Fn(Date) -> bool>);

impl DateFilter {
    pub fn matches(&self, date: Date) -> bool {
        (self.0)(date)
    }
}

impl<F: Fn(Date) -> bool + 'static> From<F> for DateFilter {
    fn from(f: F) -> Self {
        DateFilter(Rc::new(f))
    }
}

/// Rounds only the left and/or right corners (`rounded-l-*` / `rounded-r-*`).
fn round_sides(mut el: Div, left: Option<Length>, right: Option<Length>) -> Div {
    let c = &mut el.style().corner_radii;
    if let Some(l) = left {
        c.top_left = Some(l);
        c.bottom_left = Some(l);
    }
    if let Some(r) = right {
        c.top_right = Some(r);
        c.bottom_right = Some(r);
    }
    el
}

#[component]
pub fn Calendar(
    #[prop(optional)] mode: CalendarMode,
    /// Number of months shown side by side (`numberOfMonths`).
    #[prop(default = 1)]
    months: usize,
    /// Controlled selection.
    selection: Option<Accessor<CalendarSelection>>,
    /// Initial selection of an uncontrolled calendar.
    default_selection: Option<CalendarSelection>,
    /// Controlled single date.
    selected: Option<Accessor<Option<Date>>>,
    /// Month shown initially (defaults to the selection or today).
    default_month: Option<Date>,
    /// Dates for which this returns true can't be picked.
    disabled: Option<DateFilter>,
    on_change: Option<Callback<CalendarSelection>>,
    attrs: Attributes,
) -> impl IntoElement {
    let months = months.max(1);
    let controlled = move || {
        selection.map(|s| s.get()).or_else(|| selected.map(|d| CalendarSelection::Single(d.get())))
    };
    let initial_controlled = untrack(controlled);
    let initial_month = default_month
        .or_else(|| initial_controlled.as_ref().or(default_selection.as_ref()).and_then(|s| s.single()))
        .unwrap_or_else(Date::today)
        .first_of_month();
    let initial_selection = initial_controlled.or(default_selection).unwrap_or_else(|| CalendarSelection::empty(mode));
    let month = signal(initial_month);
    let inner = signal(initial_selection);
    let t = theme();

    move || {
        let theme = t.get();
        let current_month = month.get();
        let selection = controlled().unwrap_or_else(|| inner.get());
        let today = Date::today();
        let radius = theme.radius_md();
        let (accent, accent_fg) = (theme.accent, theme.accent_foreground);

        let nav = |icon: IconName, delta: i32| {
            <div
                class="flex size-8 items-center justify-center cursor-pointer"
                corner_radius={radius}
                hover={move |s: Style| s.bg(accent)}
                onClick={move |_| month.update(|m| *m = m.add_months(delta))}
            >
                <Icon name={icon} />
            </div>
        };

        let grids = (0..months).map(|offset| {
            let first = current_month.add_months(offset as i32);
            let start = first.add_days(-(first.weekday() as i64));
            let days_in_month = Date::days_in_month(first.year, first.month);
            let weeks = (first.weekday() + days_in_month).div_ceil(7);

            let header = <div class="relative flex h-8 items-center justify-center text-sm leading-[1.25rem] font-medium">
                {format!("{} {}", first.month_name(), first.year)}
                {(offset == 0).then(|| <div class="absolute left-0">{nav(IconName::ChevronLeft, -1)}</div>)}
                {(offset + 1 == months).then(|| <div class="absolute right-0">{nav(IconName::ChevronRight, 1)}</div>)}
            </div>;

            let weekdays = <div class="flex">
                {["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].map(|d| {
                    <div class="w-8 text-center text-[12.8px]" text_color={theme.muted_foreground}>{d}</div>
                })}
            </div>;

            let rows = (0..weeks).map(|week| {
                let cells = (0..7).map(|weekday| {
                    let date = start.add_days((week * 7 + weekday) as i64);
                    let outside = date.month != first.month;
                    let is_disabled = disabled.as_ref().is_some_and(|f| f.matches(date));
                    let selected = !outside && selection.contains(date);
                    let (range_start, range_end, in_range) = match &selection {
                        CalendarSelection::Range(Some(s), Some(e)) => (date == *s, date == *e, *s < date && date < *e),
                        CalendarSelection::Range(Some(s), None) => (date == *s, date == *s, false),
                        _ => (false, false, false),
                    };
                    let is_today = date == today;
                    let range = mode == CalendarMode::Range;

                    let mut el = div()
                        .class("flex size-8 items-center justify-center text-sm leading-[1.25rem]")
                        .corner_radius(radius);
                    if outside {
                        el = el.text_color(theme.muted_foreground);
                    }
                    if is_today && !selected {
                        el = el.bg(accent).text_color(accent_fg);
                    }
                    if selected && !outside {
                        el = if range && in_range {
                            // Each week's slice of the band reads as its own pill.
                            let left = (weekday == 0).then_some(radius);
                            let right = (weekday == 6).then_some(radius);
                            round_sides(el.corner_radius(px(0.0)), left, right).bg(accent).text_color(accent_fg)
                        } else {
                            el.bg(theme.primary).text_color(theme.primary_foreground)
                        };
                        if range && (range_start || range_end) && !(range_start && range_end) {
                            el = if range_start {
                                round_sides(el, None, Some(px(0.0)))
                            } else {
                                round_sides(el, Some(px(0.0)), None)
                            };
                        }
                    }
                    // Outside days inside a range continue the band (muted text),
                    // rounded at the row's ends like in-month days.
                    if outside && in_range && range {
                        let left = (weekday == 0).then_some(radius);
                        let right = (weekday == 6).then_some(radius);
                        el = round_sides(el.corner_radius(px(0.0)), left, right).bg(accent);
                    }
                    if is_disabled {
                        el = el.opacity(0.5);
                    }
                    if !is_disabled && !outside {
                        el = el.class("cursor-pointer");
                        if !selected {
                            el = el.hover(|s| s.bg(accent).text_color(accent_fg));
                        }
                        let current = selection.clone();
                        el = el.on_click(move |_| {
                            let next = current.toggle(date);
                            inner.set(next.clone());
                            if let Some(f) = on_change {
                                f.call(&next);
                            }
                        });
                    }
                    el.child(date.day.to_string())
                });
                <div class="flex mt-2">{...cells}</div>
            });

            <div class="flex flex-col gap-4">
                {header}
                <div class="flex flex-col">
                    {weekdays}
                    {...rows}
                </div>
            </div>
        });

        // `bg-background`: opaque, so a caller's shadow doesn't tint it.
        <div class="flex gap-4 p-3 leading-none" bg={theme.background} {...&attrs}>
            {...grids}
        </div>
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

//! A date picker component with range and presets — a Popover holding a Calendar.
//!
//! ```ignore
//! DatePicker::new("dob").placeholder("Pick a date").on_change(|date, _, _| println!("{date:?}"))
//! DatePicker::new("trip").mode(CalendarMode::Range).months(2)
//! ```

use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window, div, prelude::*, px};
use std::rc::Rc;

use super::button::Button;
use super::calendar::{Calendar, CalendarMode, CalendarSelection, selection_label};
use super::icon::IconName;
use super::popover::Popover;
use super::theme::ActiveTheme;
use super::utils::{Align, StyledExt};

#[derive(IntoElement)]
pub struct DatePicker {
    id: ElementId,
    mode: CalendarMode,
    months: usize,
    placeholder: SharedString,
    selection: Option<CalendarSelection>,
    on_change: Option<Rc<dyn Fn(&CalendarSelection, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(DatePicker);

impl DatePicker {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            mode: CalendarMode::Single,
            months: 1,
            placeholder: "Pick a date".into(),
            selection: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn months(mut self, months: usize) -> Self {
        self.months = months;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Controlled selection.
    pub fn selection(mut self, selection: CalendarSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&CalendarSelection, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for DatePicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mode = self.mode;
        let empty = match mode {
            CalendarMode::Single => CalendarSelection::Single(None),
            CalendarMode::Range => CalendarSelection::Range(None, None),
            CalendarMode::Multiple => CalendarSelection::Multiple(Vec::new()),
        };
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| empty);
        let selection = self.selection.clone().unwrap_or_else(|| state.read(cx).clone());
        let label = selection_label(&selection);
        let muted = cx.theme().muted_foreground;

        let trigger = Button::new("date-trigger")
            .outline()
            .w(px(if mode == CalendarMode::Range { 300. } else { 240. }))
            .justify_start()
            .font_weight(gpui::FontWeight::NORMAL)
            .icon(IconName::Calendar)
            .child(match label {
                Some(label) => div().child(label),
                None => div().text_color(muted).child(self.placeholder.clone()),
            })
            .refine_style(&self.style);

        let (months, on_change) = (self.months, self.on_change);
        Popover::new(self.id)
            .align(Align::Start)
            .w_auto()
            .p_0()
            .trigger(trigger)
            .content_with_handle(move |popover, _, _| {
                let popover = popover.clone();
                let state = state.clone();
                let on_change = on_change.clone();
                Calendar::new("date-calendar")
                    .mode(mode)
                    .months(months)
                    .selection(selection.clone())
                    .on_change(move |next, window, cx| {
                        state.update(cx, |s, cx| {
                            *s = next.clone();
                            cx.notify();
                        });
                        let complete = match next {
                            CalendarSelection::Single(Some(_)) => true,
                            CalendarSelection::Range(Some(_), Some(_)) => true,
                            _ => false,
                        };
                        if complete {
                            popover.close(window, cx);
                        }
                        if let Some(handler) = &on_change {
                            handler(next, window, cx);
                        }
                    })
            })
    }
}

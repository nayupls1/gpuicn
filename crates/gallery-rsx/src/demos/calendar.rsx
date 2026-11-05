use rsx::prelude::*;

use crate::ui::calendar::{Calendar, CalendarMode, CalendarSelection, Date};
use crate::ui::theme::theme;

/// `border-border`. rsx#?: style-method attributes (`border_color={..}`)
/// aren't accepted on components, so it goes through `Attributes`.
fn border() -> Attributes {
    let t = theme();
    Attributes::new().with(move |el: rsx::Div| el.border_color(t.get().border))
}

pub fn demo() -> Element {
    // rsx#?: an integer literal passed to a `usize` prop doesn't compile (`From<i32>`), hence `2usize`.
    let today = Date::today();
    // rsx#7: no `shadow-sm` on the calendars.
    <div class="flex items-start gap-6">
        <Calendar
            default_selection={CalendarSelection::Single(Some(today.add_days(8)))}
            class="rounded-lg border"
            {...border()}
        />
        <Calendar
            mode={CalendarMode::Range}
            default_selection={CalendarSelection::Range(Some(today.add_days(2)), Some(today.add_days(16)))}
            months={2usize}
            class="rounded-md border"
            {...border()}
        />
    </div>
}

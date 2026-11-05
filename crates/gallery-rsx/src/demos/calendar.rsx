use rsx::prelude::*;

use crate::ui::calendar::{Calendar, CalendarMode, CalendarSelection, Date};
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    let today = Date::today();
    <div class="flex items-start gap-6">
        <Calendar
            default_selection={CalendarSelection::Single(Some(today.add_days(8)))}
            class="rounded-lg border"
            class:shadow-sm={t.get().shadows}
            border_color={t.get().border}
        />
        <Calendar
            mode={CalendarMode::Range}
            default_selection={CalendarSelection::Range(Some(today.add_days(2)), Some(today.add_days(16)))}
            months={2}
            class="rounded-md border"
            class:shadow-sm={t.get().shadows}
            border_color={t.get().border}
        />
    </div>
}

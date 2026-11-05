use rsx::prelude::*;

use crate::ui::calendar::CalendarMode;
use crate::ui::date_picker::DatePicker;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 w-[280px]">
        <DatePicker class="w-full" />
        <DatePicker mode={CalendarMode::Range} months={2} placeholder="Pick a date range" class="w-full" />
    </div>
}

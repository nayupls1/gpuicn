use rsx::prelude::*;

use crate::ui::calendar::CalendarMode;
use crate::ui::date_picker::DatePicker;

pub fn demo() -> Element {
    // rsx#?: an integer literal passed to a `usize` prop doesn't compile (`From<i32>`), hence `2usize`.
    <div class="flex flex-col gap-4 w-[280px]">
        <DatePicker class="w-full" />
        <DatePicker mode={CalendarMode::Range} months={2usize} placeholder="Pick a date range" class="w-full" />
    </div>
}

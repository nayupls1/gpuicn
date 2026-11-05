use rsx::prelude::*;

use crate::ui::native_select::{disabled_option, option, NativeSelect};

pub fn demo() -> Element {
    <NativeSelect
        class="w-[240px]"
        placeholder="Select status"
        options={vec![
            option("todo", "Todo"),
            option("in-progress", "In Progress"),
            option("done", "Done"),
            disabled_option("cancelled", "Cancelled"),
        ]}
    />
}

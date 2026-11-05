use rsx::prelude::*;

use crate::ui::checkbox::Checkbox;
use crate::ui::label::Label;

pub fn demo() -> Element {
    <div class="flex items-center gap-2">
        <Checkbox />
        <Label text="Accept terms and conditions" />
    </div>
}

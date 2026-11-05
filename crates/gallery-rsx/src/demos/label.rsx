use rsx::prelude::*;

use crate::ui::label::Label;

pub fn demo() -> Element {
    <div class="flex items-center gap-2">
        {/* TODO(checkbox): `<Checkbox />` before the label once `checkbox` is ported. */}
        <input type="checkbox" />
        <Label text="Accept terms and conditions" />
    </div>
}

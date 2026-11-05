use rsx::prelude::*;

use crate::ui::checkbox::Checkbox;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4">
        <Checkbox label="Accept terms and conditions" />
        <Checkbox default_checked label="Enable notifications" />
        <Checkbox indeterminate={true} label="Select all" />
        <Checkbox disabled={true} label="Disabled" />
    </div>
}

use rsx::prelude::*;

use crate::ui::switch::Switch;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4">
        <Switch label="Airplane Mode" />
        <Switch default_checked label="Wi-Fi" />
        <Switch disabled={true} label="Disabled" />
    </div>
}

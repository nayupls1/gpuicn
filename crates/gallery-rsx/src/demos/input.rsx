use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::input::Input;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 w-[320px]">
        <Input placeholder="Email" />
        <Input placeholder="Password" masked={true} />
        <Input placeholder="Disabled" disabled={true} />
        <Input placeholder="Invalid" invalid={true} />
        <div class="flex items-center gap-2 flex-nowrap">
            <Input placeholder="Email" />
            <Button outline label="Subscribe" />
        </div>
    </div>
}

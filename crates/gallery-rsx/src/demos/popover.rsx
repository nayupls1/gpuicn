use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::input::Input;
use crate::ui::label::Label;
use crate::ui::popover::Popover;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <Popover
        trigger={<Button outline label="Open popover" />}
        class="w-[320px]"
        content={move |_| <div class="flex flex-col gap-3">
            <div class="flex flex-col gap-1.5 pb-1">
                <div class="font-medium leading-[16px]">Dimensions</div>
                <div class="text-sm" text_color={t.get().muted_foreground}>Set the dimensions for the layer.</div>
            </div>
            {["Width", "Max. width", "Height", "Max. height"].map(|label| <div class="flex flex-nowrap gap-4 items-center">
                <Label text={label} class="w-[88px] flex-none" />
                <Input default_value="100%" class="h-8 flex-1" />
            </div>)}
        </div>}
    />
}

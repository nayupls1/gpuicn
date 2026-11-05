use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::DialogHandle;
use crate::ui::field::Field;
use crate::ui::input::Input;
use crate::ui::sheet::Sheet;
use crate::ui::utils::Side;

fn sheet(side: Side, label: &'static str) -> Element {
    <Sheet
        side={side}
        trigger={<Button outline label={label} />}
        title="Edit profile"
        description="Make changes to your profile here. Click save when you're done."
        content={|_| <div class="flex flex-col gap-4">
            <Field label="Name"><Input default_value="Pedro Duarte" /></Field>
            <Field label="Username"><Input default_value="@peduarte" /></Field>
        </div>}
        footer={|sheet: DialogHandle| <div class="flex flex-col gap-2">
            <Button label="Save changes" />
            <Button outline label="Close" onClick={move |_| sheet.close()} />
        </div>}
    />
}

pub fn demo() -> Element {
    <div class="flex items-center gap-2">
        {sheet(Side::Top, "Top")}
        {sheet(Side::Right, "Right")}
        {sheet(Side::Bottom, "Bottom")}
        {sheet(Side::Left, "Left")}
    </div>
}

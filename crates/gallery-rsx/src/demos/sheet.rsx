use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::DialogHandle;
use crate::ui::label::Label;
use crate::ui::sheet::Sheet;
use crate::ui::utils::Side;

/// A labelled text field.
fn field(label: &'static str, value: &'static str) -> Element {
    // TODO(field): `<Field label={label}><Input default_value={value} /></Field>` once `field` and `input` are ported.
    <div class="flex flex-col gap-3">
        <Label text={label} />
        <input value={value} class="h-9 w-full" />
    </div>
}

fn sheet(side: Side, label: &'static str) -> Element {
    <Sheet
        side={side}
        trigger={<Button outline label={label} />}
        title="Edit profile"
        description="Make changes to your profile here. Click save when you're done."
        content={|_| <div class="flex flex-col gap-4">
            {field("Name", "Pedro Duarte")}
            {field("Username", "@peduarte")}
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

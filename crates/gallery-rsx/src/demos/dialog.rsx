use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{Dialog, DialogHandle};
use crate::ui::label::Label;
use crate::ui::sonner::{toast, Toast};

/// A labelled text field.
fn field(label: &'static str, value: &'static str) -> Element {
    // TODO(field): `<Field label={label}><Input default_value={value} /></Field>` once `field` and `input` are ported.
    <div class="flex flex-col gap-3">
        <Label text={label} />
        <input value={value} class="h-9 w-full" />
    </div>
}

pub fn demo() -> Element {
    <Dialog
        trigger={<Button outline label="Edit Profile" />}
        title="Edit profile"
        description="Make changes to your profile here. Click save when you're done."
        content={|_| <div class="flex flex-col gap-4">
            {field("Name", "Pedro Duarte")}
            {field("Username", "@peduarte")}
            <div class="flex flex-col gap-3">
                <Label text="Theme" />
                {/* TODO(select): `<Select class="w-full" default_value="system" items=[Light, Dark, System] />` once `select` is ported. */}
                <select value="system" class="w-full">
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                    <option value="system">System</option>
                </select>
            </div>
        </div>}
        footer={|dialog: DialogHandle| <div class="flex items-center gap-2">
            <Button outline label="Cancel" onClick={move |_| dialog.close()} />
            <Button label="Save changes" onClick={move |_| {
                dialog.close();
                toast(Toast::new("Profile updated").success());
            }} />
        </div>}
    />
}

use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{Dialog, DialogHandle};
use crate::ui::field::Field;
use crate::ui::input::Input;
use crate::ui::select::{Select, SelectItem};
use crate::ui::sonner::{toast, Toast};

pub fn demo() -> Element {
    <Dialog
        trigger={<Button outline label="Edit Profile" />}
        title="Edit profile"
        description="Make changes to your profile here. Click save when you're done."
        content={|_| <div class="flex flex-col gap-4">
            <Field label="Name"><Input default_value="Pedro Duarte" /></Field>
            <Field label="Username"><Input default_value="@peduarte" /></Field>
            <Field label="Theme">
                <Select
                    class="w-full"
                    default_value={SharedString::from("system")}
                    items={vec![
                        SelectItem::new("light", "Light"),
                        SelectItem::new("dark", "Dark"),
                        SelectItem::new("system", "System"),
                    ]}
                />
            </Field>
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

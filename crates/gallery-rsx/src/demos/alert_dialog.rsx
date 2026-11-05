use rsx::prelude::*;

use crate::ui::alert_dialog::AlertDialog;
use crate::ui::button::Button;
use crate::ui::sonner::toast;

pub fn demo() -> Element {
    <AlertDialog
        trigger={<Button outline label="Show Dialog" />}
        title="Are you absolutely sure?"
        description="This action cannot be undone. This will permanently delete your account and remove your data from our servers."
        action="Continue"
        onAction={move |_| {
            toast("Account deleted");
        }}
    />
}

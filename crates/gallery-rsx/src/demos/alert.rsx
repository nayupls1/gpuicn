use rsx::prelude::*;

use crate::ui::alert::Alert;
use crate::ui::icon::IconName;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 w-[560px]">
        <Alert
            icon={IconName::CircleCheck}
            title="Success! Your changes have been saved"
            description="This is an alert with icon, title and description."
        />
        <Alert icon={IconName::Info} title="This Alert has a title and an icon. No description." />
        <Alert
            destructive
            icon={IconName::CircleAlert}
            title="Unable to process your payment."
            description="Please verify your billing information and try again."
        />
    </div>
}

use rsx::prelude::*;

use crate::ui::badge::Badge;
use crate::ui::button::{Button, ButtonSize};
use crate::ui::spinner::Spinner;

pub fn demo() -> Element {
    <div class="flex items-center gap-6">
        <Spinner />
        <Spinner class="size-8" />
        <Button secondary size={ButtonSize::Sm} loading={true} label="Loading..." />
        <Badge><Spinner class="size-3" />Syncing</Badge>
    </div>
}

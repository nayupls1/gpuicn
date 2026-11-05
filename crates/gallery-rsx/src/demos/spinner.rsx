use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::spinner::Spinner;

pub fn demo() -> Element {
    <div class="flex items-center gap-6">
        <Spinner />
        <Spinner class="size-8" />
        <Button secondary size={ButtonSize::Sm} loading={true} label="Loading..." />
        {/* TODO(badge): `<Badge><Spinner class="size-3" />Syncing</Badge>` once `badge` is ported. */}
    </div>
}

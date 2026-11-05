use rsx::prelude::*;

use crate::ui::badge::Badge;
use crate::ui::icon::IconName;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <div class="flex items-center gap-2">
        <Badge label="Badge" />
        <Badge secondary label="Secondary" />
        <Badge destructive label="Destructive" />
        <Badge outline label="Outline" />
        <Badge
            label="Verified"
            icon={IconName::CircleCheck}
            class={if t.get().is_dark() { "bg-[#155dfc] text-white" } else { "bg-[#2b7fff] text-white" }}
        />
        <Badge label="8" class="rounded-full h-5 min-w-5 px-1" />
        <Badge destructive label="99" class="rounded-full h-5 min-w-5 px-1" />
        <Badge outline label="20+" class="rounded-full h-5 min-w-5 px-1" />
    </div>
}

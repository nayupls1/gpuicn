use rsx::prelude::*;

use crate::demos::avatar_image;
use crate::ui::avatar::Avatar;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <div class="flex items-center gap-4">
        <Avatar src={avatar_image(0)} fallback="CN" />
        <Avatar src={avatar_image(1)} fallback="ER" class="rounded-lg" />
        <Avatar fallback="LR" />
        {/* Avatar group: overlapping, each ringed with the page background (`ring-2 ring-background`). */}
        <div class="flex">
            {["CN", "LR", "ER"].into_iter().enumerate().map(|(ix, initials)| {
                <div class="p-[2px] rounded-full" ml={px(if ix > 0 { -10.0 } else { 0.0 })} bg={t.get().background}>
                    <Avatar src={avatar_image(ix)} fallback={initials} />
                </div>
            }).collect::<Vec<_>>()}
        </div>
    </div>
}

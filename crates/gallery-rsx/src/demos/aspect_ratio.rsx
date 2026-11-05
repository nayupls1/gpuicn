use rsx::prelude::*;

use crate::ui::aspect_ratio::AspectRatio;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <AspectRatio ratio={16.0 / 9.0} class="w-[450px] rounded-lg" bg={t.get().muted}>
        <div class="size-full flex items-center justify-center">16 / 9</div>
    </AspectRatio>
}

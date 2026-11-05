use rsx::prelude::*;

use crate::ui::aspect_ratio::AspectRatio;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    // rsx#N: a component's `Attributes` can't take style methods like `bg={..}`,
    // so the theme color goes on the content.
    <AspectRatio ratio={16.0f32 / 9.0} class="w-[450px] rounded-lg">
        <div class="size-full flex items-center justify-center rounded-lg" bg={t.get().muted}>16 / 9</div>
    </AspectRatio>
}

use rsx::prelude::*;

use crate::ui::avatar::Avatar;
use crate::ui::theme::theme;

/// Bundled avatar pictures (the gpui gallery's), by file path.
fn avatar_image(n: usize) -> String {
    let file = match n % 3 {
        0 => "avatar-1.png",
        1 => "avatar-2.png",
        _ => "avatar-3.png",
    };
    format!("{}/../gallery/assets/{file}", env!("CARGO_MANIFEST_DIR"))
}

pub fn demo() -> Element {
    let t = theme();
    <div class="flex items-center gap-4">
        <Avatar src={avatar_image(0)} fallback="CN" />
        <Avatar src={avatar_image(1)} fallback="ER" class="rounded-lg" />
        <Avatar fallback="LR" />
        {/* Avatar group: overlapping, each ringed with the page background. */}
        <div class="flex">
            {["CN", "LR", "ER"].into_iter().enumerate().map(|(ix, initials)| {
                <div class="p-[2px] rounded-full" bg={t.get().background} ml={px(if ix > 0 { -10.0 } else { 0.0 })}>
                    <Avatar src={avatar_image(ix)} fallback={initials} />
                </div>
            }).collect::<Vec<_>>()}
        </div>
    </div>
}

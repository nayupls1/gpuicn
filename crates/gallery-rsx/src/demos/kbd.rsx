use rsx::prelude::*;

use crate::ui::kbd::{Kbd, KbdGroup};
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <div class="flex flex-col gap-4 items-center">
        <KbdGroup>
            <Kbd text="⌘" />
            <Kbd text="⇧" />
            <Kbd text="⌥" />
            <Kbd text="⌃" />
        </KbdGroup>
        <KbdGroup>
            <Kbd text="Ctrl" />
            <div text_color={t.get().muted_foreground}>+</div>
            <Kbd text="B" />
        </KbdGroup>
    </div>
}

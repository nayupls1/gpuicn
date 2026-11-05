use rsx::prelude::*;

use crate::ui::separator::Separator;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <div>
        <div class="flex flex-col gap-1">
            <div class="text-sm font-medium">Radix Primitives</div>
            <div class="text-sm" text_color={t.get().muted_foreground}>An open-source UI component library.</div>
        </div>
        <Separator class="my-4" />
        <div class="flex items-center h-5 gap-4 text-sm">
            Blog
            <Separator vertical />
            Docs
            <Separator vertical />
            Source
        </div>
    </div>
}

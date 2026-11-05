use rsx::prelude::*;

use crate::ui::scroll_area::ScrollArea;
use crate::ui::separator::Separator;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <ScrollArea class="h-[288px] w-[192px] rounded-md border" border_color={t.get().border}>
        <div class="p-4">
            <div class="mb-4 text-sm font-medium">Tags</div>
            {(1..=50).rev().map(|n| <div>
                <div class="text-sm">{format!("v1.2.0-beta.{n}")}</div>
                <Separator class="my-2" />
            </div>)}
        </div>
    </ScrollArea>
}

use rsx::prelude::*;

use crate::ui::resizable::{ResizablePanel, ResizablePanelGroup};
use crate::ui::theme::theme;

fn cell(label: &'static str) -> Element {
    <div class="flex size-full items-center justify-center font-semibold">{label}</div>
}

pub fn demo() -> Element {
    let t = theme();
    <div class="w-[480px] h-[240px] rounded-lg border overflow-hidden" border_color={t.get().border}>
        <ResizablePanelGroup with_handle>
            <ResizablePanel default_size={50} min_size={20}>{cell("One")}</ResizablePanel>
            <ResizablePanel default_size={50}>
                <ResizablePanelGroup vertical with_handle>
                    <ResizablePanel default_size={25}>{cell("Two")}</ResizablePanel>
                    <ResizablePanel default_size={75}>{cell("Three")}</ResizablePanel>
                </ResizablePanelGroup>
            </ResizablePanel>
        </ResizablePanelGroup>
    </div>
}

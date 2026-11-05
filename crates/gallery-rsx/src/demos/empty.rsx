use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::empty::Empty;
use crate::ui::icon::IconName;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <Empty
        outline
        class="w-[560px]"
        icon={IconName::Folder}
        title="No Projects Yet"
        description="You haven't created any projects yet. Get started by creating your first project."
    >
        <div class="flex flex-col items-center gap-4">
            <div class="flex flex-nowrap items-center gap-2">
                <Button label="Create Project" />
                <Button outline label="Import Project" />
            </div>
            <Button link size={ButtonSize::Sm} icon_end={IconName::ArrowUpRight} label="Learn More" text_color={t.get().muted_foreground} />
        </div>
    </Empty>
}

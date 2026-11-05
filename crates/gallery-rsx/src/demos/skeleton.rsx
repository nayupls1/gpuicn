use rsx::prelude::*;

use crate::ui::skeleton::Skeleton;

pub fn demo() -> Element {
    <div class="flex items-center gap-4">
        <Skeleton class="size-12 rounded-full" />
        <div class="flex flex-col gap-2">
            <Skeleton class="h-4 w-[250px]" />
            <Skeleton class="h-4 w-[200px]" />
        </div>
    </div>
}

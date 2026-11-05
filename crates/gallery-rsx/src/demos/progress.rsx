use rsx::prelude::*;

use crate::ui::progress::Progress;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 w-[300px]">
        <Progress value={13.0f32} />
        <Progress value={66.0f32} />
        <Progress value={100.0f32} />
    </div>
}

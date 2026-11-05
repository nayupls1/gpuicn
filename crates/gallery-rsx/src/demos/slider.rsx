use rsx::prelude::*;

use crate::ui::slider::Slider;

pub fn demo() -> Element {
    <div class="flex flex-col gap-8 w-[300px]">
        <Slider default_value={vec![50.0]} max={100} step={1} />
        <Slider default_value={vec![25.0, 75.0]} />
        <div class="flex justify-center h-[160px]">
            <Slider vertical default_value={vec![40.0]} />
        </div>
    </div>
}

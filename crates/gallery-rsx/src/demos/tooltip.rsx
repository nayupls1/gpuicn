use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::tooltip::Tooltip;
use crate::ui::utils::Side;

pub fn demo() -> Element {
    <div class="flex items-center gap-4">
        <Tooltip content="Add to library"><Button outline label="Hover" /></Tooltip>
        <Tooltip side={Side::Right} content="Shown on the right"><Button outline label="Right" /></Tooltip>
    </div>
}

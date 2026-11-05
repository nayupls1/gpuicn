use rsx::prelude::*;

use crate::ui::icon::IconName;
use crate::ui::toggle::Toggle;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 items-center">
        {/* Default variant: off, then on. */}
        <div class="flex items-center gap-2">
            <Toggle icon={IconName::Bold} />
            <Toggle default_pressed icon={IconName::Italic} />
            <Toggle icon={IconName::Underline} />
        </div>
        {/* Outline variant: off, then on. */}
        <div class="flex items-center gap-2">
            <Toggle outline icon={IconName::Star} label="Bookmark" />
            <Toggle outline default_pressed icon={IconName::Heart} label="Favorite" />
        </div>
    </div>
}

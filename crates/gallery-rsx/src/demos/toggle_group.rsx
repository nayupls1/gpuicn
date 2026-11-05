use rsx::prelude::*;

use crate::ui::icon::IconName;
use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

pub fn demo() -> Element {
    // `self-start`: shrink to the groups' width, like the gpui gallery's centered preview.
    <div class="flex flex-col gap-4 self-start">
        <ToggleGroup
            multiple
            outline
            items={vec![
                ToggleGroupItem::new("bold").icon(IconName::Bold),
                ToggleGroupItem::new("italic").icon(IconName::Italic),
                ToggleGroupItem::new("underline").icon(IconName::Underline),
            ]}
        />
        <ToggleGroup
            default_value={vec!["left".into()]}
            items={vec![
                ToggleGroupItem::new("left").icon(IconName::AlignLeft),
                ToggleGroupItem::new("center").icon(IconName::AlignCenter),
                ToggleGroupItem::new("right").icon(IconName::AlignRight),
            ]}
        />
    </div>
}

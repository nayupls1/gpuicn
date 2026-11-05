use rsx::prelude::*;

use crate::ui::command::{Command, CommandGroup, CommandItem};
use crate::ui::icon::IconName;
use crate::ui::sonner::toast;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    // Fixed height, anchored to the top: filtering shouldn't move the input.
    <div class="h-[340px]">
        <Command
            class="w-[450px] rounded-lg border"
            bind={move |el: rsx::Div| if t.get().shadows { el.class("shadow-md") } else { el }}
            border_color={t.get().border}
            groups={vec![
                CommandGroup::new("Suggestions")
                    .item(CommandItem::new("Calendar").icon(IconName::Calendar))
                    .item(CommandItem::new("Search Emoji").icon(IconName::Search))
                    .item(CommandItem::new("Calculator").icon(IconName::Terminal).disabled(true)),
                CommandGroup::new("Settings")
                    .item(CommandItem::new("Profile").icon(IconName::User).shortcut("⌘P"))
                    .item(CommandItem::new("Billing").icon(IconName::CreditCard).shortcut("⌘B"))
                    .item(CommandItem::new("Settings").icon(IconName::Settings).shortcut("⌘S")),
            ]}
            onSelect={|value: &SharedString| {
                toast(format!("Selected {value}"));
            }}
        />
    </div>
}

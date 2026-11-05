use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::dropdown_menu::{DropdownMenu, Menu, MenuBuilder, MenuEntry, MenuItem};
use crate::ui::icon::IconName;

pub fn demo() -> Element {
    <DropdownMenu
        trigger={<Button outline label="Open" />}
        class="w-[224px]"
        menu={Menu::new()
            .label("My Account")
            .item(MenuItem::new("Profile").shortcut("⇧⌘P"))
            .item(MenuItem::new("Billing").shortcut("⌘B"))
            .item(MenuItem::new("Settings").shortcut("⌘S"))
            .item(MenuItem::new("Keyboard shortcuts").shortcut("⌘K"))
            .separator()
            .item(MenuItem::new("Team"))
            .submenu(
                "Invite users",
                vec![
                    MenuItem::new("Email").into(),
                    MenuItem::new("Message").into(),
                    MenuEntry::Separator,
                    MenuItem::new("More...").into(),
                ],
            )
            .item(MenuItem::new("New Team").shortcut("⌘+T"))
            .separator()
            .item(MenuItem::new("GitHub"))
            .item(MenuItem::new("Support"))
            .item(MenuItem::new("API").disabled(true))
            .separator()
            .item(MenuItem::new("Log out").icon(IconName::LogOut).destructive().shortcut("⇧⌘Q"))}
    />
}

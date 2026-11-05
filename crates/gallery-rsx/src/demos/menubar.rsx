use rsx::prelude::*;

use crate::ui::dropdown_menu::{MenuBuilder, MenuEntry, MenuItem};
use crate::ui::menubar::{Menubar, MenubarMenu};

pub fn demo() -> Element {
    <Menubar menus={vec![
        MenubarMenu::new("File")
            .item(MenuItem::new("New Tab").shortcut("⌘T"))
            .item(MenuItem::new("New Window").shortcut("⌘N"))
            .item(MenuItem::new("New Incognito Window").disabled(true))
            .separator()
            .submenu("Share", vec![MenuItem::new("Email link").into(), MenuItem::new("Messages").into(), MenuItem::new("Notes").into()])
            .separator()
            .item(MenuItem::new("Print...").shortcut("⌘P")),
        MenubarMenu::new("Edit")
            .item(MenuItem::new("Undo").shortcut("⌘Z"))
            .item(MenuItem::new("Redo").shortcut("⇧⌘Z"))
            .separator()
            .submenu(
                "Find",
                vec![
                    MenuItem::new("Search the web").into(),
                    MenuEntry::Separator,
                    MenuItem::new("Find...").into(),
                    MenuItem::new("Find Next").into(),
                    MenuItem::new("Find Previous").into(),
                ],
            )
            .separator()
            .item(MenuItem::new("Cut"))
            .item(MenuItem::new("Copy"))
            .item(MenuItem::new("Paste")),
        MenubarMenu::new("View")
            .item(MenuItem::checkbox("Always Show Bookmarks Bar", false))
            .item(MenuItem::checkbox("Always Show Full URLs", true))
            .separator()
            .item(MenuItem::new("Reload").inset().shortcut("⌘R"))
            .item(MenuItem::new("Toggle Fullscreen").inset()),
        MenubarMenu::new("Profiles")
            .item(MenuItem::radio("Andy", false))
            .item(MenuItem::radio("Benoit", true))
            .item(MenuItem::radio("Luis", false))
            .separator()
            .item(MenuItem::new("Edit...").inset())
            .item(MenuItem::new("Add Profile...").inset()),
    ]} />
}

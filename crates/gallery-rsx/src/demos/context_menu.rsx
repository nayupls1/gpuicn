use rsx::prelude::*;

use crate::ui::context_menu::ContextMenu;
use crate::ui::dropdown_menu::{Menu, MenuBuilder, MenuItem};
use crate::ui::theme::{theme, ColorExt};

pub fn demo() -> Element {
    let t = theme();
    <ContextMenu
        trigger={<div
            class="flex h-[150px] w-[300px] items-center justify-center rounded-md text-sm"
            border_width={px(1.5)}
            border_color={t.get().muted_foreground.opacity(0.45)}
        >
            {/* rsx: no dashed borders (gpuicn draws this one `border_dashed()`). */}
            Right click here
        </div>}
        class="min-w-[208px]"
        menu={Menu::new()
            .item(MenuItem::new("Back").inset().shortcut("⌘["))
            .item(MenuItem::new("Forward").inset().shortcut("⌘]").disabled(true))
            .item(MenuItem::new("Reload").inset().shortcut("⌘R"))
            .submenu(
                "More Tools",
                vec![
                    MenuItem::new("Save Page...").into(),
                    MenuItem::new("Create Shortcut...").into(),
                    MenuItem::new("Developer Tools").into(),
                ],
            )
            .separator()
            .item(MenuItem::checkbox("Show Bookmarks", true))
            .item(MenuItem::checkbox("Show Full URLs", false))
            .separator()
            .label("People")
            .item(MenuItem::radio("Pedro Duarte", true))
            .item(MenuItem::radio("Colm Tuite", false))}
    />
}

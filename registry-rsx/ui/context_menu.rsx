//! Displays a menu to the user — such as a set of actions or functions —
//! triggered by a right click.
//!
//! ```ignore
//! <ContextMenu
//!     trigger={<div class="size-[300px] border">Right click here</div>}
//!     menu={Menu::new()
//!         .item(MenuItem::new("Back").shortcut("⌘["))
//!         .item(MenuItem::new("Forward").shortcut("⌘]").disabled(true))
//!         .separator()
//!         .item(MenuItem::checkbox("Show Bookmarks", true))}
//! />
//! ```

use rsx::prelude::*;
use rsx::Point;

use super::dropdown_menu::{Menu, MenuList};
use super::popover::{Popover, PopoverHandle, PopoverTrigger};

#[component]
pub fn ContextMenu(
    /// The area that responds to right clicks.
    trigger: Option<Children>,
    /// The entries (see [`Menu`]); a closure keeps them following signals.
    #[prop(optional)]
    menu: Accessor<Menu>,
    /// Styles for the menu panel (`class="min-w-[208px]"`).
    attrs: Attributes,
) -> impl IntoElement {
    let popover = PopoverHandle::new();
    // Where the right click happened, in window coordinates.
    let position = signal(None::<Point>);
    let attrs = StoredValue::new(attrs);
    <Popover
        unstyled
        handle={popover}
        mode={PopoverTrigger::Manual}
        anchor_position={position}
        trigger={move || <div
            class="flex"
            onContextMenu={move |e: &MouseEvent| {
                e.stop_propagation();
                position.set(Some(e.position));
                popover.open();
            }}
        >
            {trigger}
        </div>}
        content={move |p: PopoverHandle| <MenuList entries={menu} popover={p} {...attrs.with_value(|a| a.clone())} />}
    />
}

//! A visually persistent menu common in desktop applications that provides
//! quick access to a consistent set of commands.
//!
//! ```ignore
//! <Menubar menus={vec![
//!     MenubarMenu::new("File")
//!         .item(MenuItem::new("New Tab").shortcut("⌘T"))
//!         .item(MenuItem::new("New Window").shortcut("⌘N"))
//!         .separator()
//!         .submenu("Share", vec![MenuItem::new("Email link").into()]),
//!     MenubarMenu::new("Edit").item(MenuItem::new("Undo").shortcut("⌘Z")),
//! ]} />
//!
//! // Menus whose items read signals: pass a closure.
//! <Menubar menus={move || vec![
//!     MenubarMenu::new("View").item(MenuItem::checkbox("Show Bookmarks", bookmarks.get())),
//! ]} />
//! ```
//!
//! gpuicn's `Menubar::new(id).menu(..).menu(..)` becomes the `menus` prop.

use rsx::prelude::*;

use super::dropdown_menu::{bounds_contain, Menu, MenuBuilder, MenuEntry, MenuList};
use super::popover::{Popover, PopoverHandle, PopoverTrigger};
use super::theme::theme;
use super::utils::Align;

#[derive(Clone)]
pub struct MenubarMenu {
    label: SharedString,
    entries: Vec<MenuEntry>,
}

impl MenubarMenu {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), entries: Vec::new() }
    }
}

impl MenuBuilder for MenubarMenu {
    fn entries_mut(&mut self) -> &mut Vec<MenuEntry> {
        &mut self.entries
    }
}

#[component]
pub fn Menubar(
    /// The menus, left to right (gpuicn's `.menu(..)`). A closure keeps
    /// their items following the signals it reads.
    #[prop(optional)]
    menus: Accessor<Vec<MenubarMenu>>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    // Which menu is open.
    let open = signal(None::<usize>);
    let labels: Vec<SharedString> = menus.with_untracked(|m| m.iter().map(|m| m.label.clone()).collect());
    // Where each trigger is, to switch menus as the pointer crosses them.
    let triggers: Vec<NodeRef> = labels.iter().map(|_| NodeRef::new()).collect();
    let triggers = StoredValue::new(triggers);

    let menu = move |(ix, label): (usize, SharedString)| {
        let hovered = signal(false);
        let node = triggers.with_value(|t| t[ix]);
        let entries = move || menus.with(|m| m.get(ix).map(|m| Menu { entries: m.entries.clone() }).unwrap_or_default());
        let trigger = move || {
            let theme = t.get();
            let lit = open.get() == Some(ix) || hovered.get();
            <div
                ref={node}
                class="flex items-center px-2 py-1 text-sm leading-[1.25rem] font-medium cursor-pointer"
                corner_radius={theme.radius_sm()}
                bg={if lit { theme.accent } else { Color::TRANSPARENT }}
                text_color={if lit { theme.accent_foreground } else { theme.foreground }}
                onHover={move |h: &bool| hovered.set(*h)}
                onClick={move |_| open.update(|o| *o = if *o == Some(ix) { None } else { Some(ix) })}
            >
                {label.clone()}
            </div>
        };
        <Popover
            unstyled
            mode={PopoverTrigger::Manual}
            open={open.get() == Some(ix)}
            align={Align::Start}
            side_offset={8}
            on_open_change={move |is_open: &bool| {
                if !*is_open && open.get_untracked() == Some(ix) {
                    open.set(None);
                }
            }}
            // While a menu is open, pointing at another trigger opens that menu.
            on_backdrop_move={move |position: &rsx::Point| {
                let hovered = triggers.with_value(|t| t.iter().position(|n| n.bounds().is_some_and(|b| bounds_contain(&b, *position))));
                if let Some(hovered) = hovered {
                    if open.get_untracked().is_some() && open.get_untracked() != Some(hovered) {
                        open.set(Some(hovered));
                    }
                }
            }}
            trigger={trigger}
            content={move |popover: PopoverHandle| <MenuList entries={entries} popover={popover} class="min-w-48" />}
        />
    };

    let attrs = StoredValue::new(attrs);
    move || {
        let theme = t.get();
        <div
            class="flex items-center gap-1 h-9 p-1 border"
            class:shadow-xs={theme.shadows}
            corner_radius={theme.radius_md()}
            border_color={theme.border}
            bg={theme.background}
            {...attrs.with_value(|a| a.clone())}
        >
            {labels.iter().cloned().enumerate().map(menu).collect::<Vec<_>>()}
        </div>
    }
}

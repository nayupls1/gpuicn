//! A visually persistent menu common in desktop applications that provides
//! quick access to a consistent set of commands.
//!
//! ```ignore
//! Menubar::new("app-menu")
//!     .menu(
//!         MenubarMenu::new("File")
//!             .item(MenuItem::new("New Tab").shortcut("⌘T"))
//!             .item(MenuItem::new("New Window").shortcut("⌘N"))
//!             .separator()
//!             .submenu("Share", vec![MenuItem::new("Email link").into()]),
//!     )
//!     .menu(MenubarMenu::new("Edit").item(MenuItem::new("Undo").shortcut("⌘Z")))
//! ```

use gpui::{
    App, Bounds, ElementId, FontWeight, IntoElement, Pixels, RenderOnce, SharedString, StyleRefinement, Window, canvas,
    div, prelude::*,
};
use std::rc::Rc;

use super::dropdown_menu::{MenuBuilder, MenuEntry, MenuList};
use super::popover::{Popover, PopoverTrigger};
use super::theme::ActiveTheme;
use super::utils::{Align, StyledExt};

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

#[derive(Default)]
struct MenubarState {
    open: Option<usize>,
    /// Window bounds of each trigger, as of the last frame.
    triggers: Vec<Bounds<Pixels>>,
}

#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    menus: Vec<MenubarMenu>,
    style: StyleRefinement,
}

crate::impl_styled!(Menubar);

impl Menubar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), menus: Vec::new(), style: StyleRefinement::default() }
    }

    pub fn menu(mut self, menu: MenubarMenu) -> Self {
        self.menus.push(menu);
        self
    }
}

impl RenderOnce for Menubar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| MenubarState::default());
        let open_menu = state.read(cx).open;
        let theme = cx.theme();
        let (accent, accent_fg) = (theme.accent, theme.accent_foreground);
        let radius = theme.radius_sm();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_1()
            .h_9()
            .p_1()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.border)
            .bg(theme.background)
            .when(theme.shadows, |this| this.shadow_xs())
            .refine_style(&self.style)
            .children(self.menus.into_iter().enumerate().map(|(ix, menu)| {
                let is_open = open_menu == Some(ix);
                let entries = Rc::new(menu.entries);
                let toggle_state = state.clone();
                let change_state = state.clone();
                let trigger = div()
                    .id(("menubar-trigger", ix))
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .rounded(radius)
                    .text_sm().line_height(gpui::rems(1.25))
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .when(is_open, |this| this.bg(accent).text_color(accent_fg))
                    .hover(move |s| s.bg(accent).text_color(accent_fg))
                    .child(menu.label)
                    .child({
                        let state = state.clone();
                        canvas(
                            move |bounds, _, cx| {
                                state.update(cx, |s, _| {
                                    if s.triggers.len() <= ix {
                                        s.triggers.resize(ix + 1, Bounds::default());
                                    }
                                    s.triggers[ix] = bounds;
                                })
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                    })
                    .on_click(move |_, _, cx| {
                        toggle_state.update(cx, |s, cx| {
                            s.open = if s.open == Some(ix) { None } else { Some(ix) };
                            cx.notify();
                        })
                    });
                let switch_state = state.clone();

                Popover::new(("menubar-menu", ix))
                    .unstyled()
                    .mode(PopoverTrigger::Manual)
                    .open(is_open)
                    .align(Align::Start)
                    .side_offset(gpui::px(8.))
                    .on_open_change(move |open, _, cx| {
                        if !open {
                            change_state.update(cx, |s, cx| {
                                if s.open == Some(ix) {
                                    s.open = None;
                                    cx.notify();
                                }
                            })
                        }
                    })
                    // While a menu is open, pointing at another trigger opens that menu.
                    .on_backdrop_move(move |position, _, cx| {
                        switch_state.update(cx, |s, cx| {
                            if let Some(hovered) = s.triggers.iter().position(|b| b.contains(&position))
                                && s.open.is_some()
                                && s.open != Some(hovered)
                            {
                                s.open = Some(hovered);
                                cx.notify();
                            }
                        })
                    })
                    .trigger(trigger)
                    .content_with_handle(move |popover, _, _| {
                        MenuList::for_popover("menubar-list", entries.clone(), popover).min_w_48()
                    })
            }))
    }
}

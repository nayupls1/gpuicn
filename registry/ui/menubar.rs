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
    App, ElementId, FontWeight, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
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

#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    menus: Vec<MenubarMenu>,
    on_value_change: Option<ValueHandler>,
    style: StyleRefinement,
}

type ValueHandler = Rc<dyn Fn(&Option<usize>, &mut Window, &mut App)>;

/// Set the open menu and tell `on_value_change` if it changed.
fn set_open_menu(
    state: &gpui::Entity<Option<usize>>,
    value: Option<usize>,
    handler: &Option<ValueHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    let changed = state.update(cx, |s, cx| {
        let changed = *s != value;
        *s = value;
        cx.notify();
        changed
    });
    if changed && let Some(handler) = handler {
        handler(&value, window, cx);
    }
}

crate::impl_styled!(Menubar);

impl Menubar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), menus: Vec::new(), on_value_change: None, style: StyleRefinement::default() }
    }

    /// Called with the index of the open menu (`None` when all are closed).
    pub fn on_value_change(mut self, handler: impl Fn(&Option<usize>, &mut Window, &mut App) + 'static) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    pub fn menu(mut self, menu: MenubarMenu) -> Self {
        self.menus.push(menu);
        self
    }
}

impl RenderOnce for Menubar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| None::<usize>);
        let open_menu = *state.read(cx);
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
                let hover_state = state.clone();
                let change_state = state.clone();
                let (toggle_handler, hover_handler, change_handler) =
                    (self.on_value_change.clone(), self.on_value_change.clone(), self.on_value_change.clone());
                let trigger = div()
                    .id(("menubar-trigger", ix))
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .rounded(radius)
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .when(is_open, |this| this.bg(accent).text_color(accent_fg))
                    .hover(move |s| s.bg(accent).text_color(accent_fg))
                    .child(menu.label)
                    .on_click(move |_, window, cx| {
                        let next = if *toggle_state.read(cx) == Some(ix) { None } else { Some(ix) };
                        set_open_menu(&toggle_state, next, &toggle_handler, window, cx);
                    })
                    // While any menu is open, hovering another one switches to it.
                    .on_mouse_move(move |_, window, cx| {
                        if hover_state.read(cx).is_some_and(|open| open != ix) {
                            set_open_menu(&hover_state, Some(ix), &hover_handler, window, cx);
                        }
                    });

                Popover::new(("menubar-menu", ix))
                    .unstyled()
                    .mode(PopoverTrigger::Manual)
                    .modal(false)
                    .open(is_open)
                    .align(Align::Start)
                    .side_offset(gpui::px(8.))
                    .on_open_change(move |open, window, cx| {
                        if !open && *change_state.read(cx) == Some(ix) {
                            set_open_menu(&change_state, None, &change_handler, window, cx);
                        }
                    })
                    .trigger(trigger)
                    .content_with_handle(move |popover, _, _| {
                        MenuList::for_popover("menubar-list", entries.clone(), popover).min_w_48()
                    })
            }))
    }
}

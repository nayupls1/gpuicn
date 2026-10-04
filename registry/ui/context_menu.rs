//! Displays a menu to the user — such as a set of actions or functions —
//! triggered by a right click.
//!
//! ```ignore
//! ContextMenu::new("canvas")
//!     .trigger(div().size(px(300.)).border_1().border_dashed().child("Right click here"))
//!     .item(MenuItem::new("Back").shortcut("⌘["))
//!     .item(MenuItem::new("Forward").shortcut("⌘]").disabled(true))
//!     .separator()
//!     .item(MenuItem::checkbox("Show Bookmarks", true))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, MouseButton, Pixels, Point, RenderOnce,
    StyleRefinement, Window, div, prelude::*,
};
use std::rc::Rc;

use super::dropdown_menu::{MenuBuilder, MenuEntry, MenuList};
use super::popover::{Popover, PopoverTrigger};
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct ContextMenu {
    id: ElementId,
    trigger: Option<AnyElement>,
    entries: Vec<MenuEntry>,
    style: StyleRefinement,
}

crate::impl_styled!(ContextMenu);

impl MenuBuilder for ContextMenu {
    fn entries_mut(&mut self) -> &mut Vec<MenuEntry> {
        &mut self.entries
    }
}

impl ContextMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), trigger: None, entries: Vec::new(), style: StyleRefinement::default() }
    }

    /// The area that responds to right clicks.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
}

#[derive(Default)]
struct ContextMenuState {
    open: bool,
    position: Point<Pixels>,
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| ContextMenuState::default());
        let (open, position) = {
            let s = state.read(cx);
            (s.open, s.position)
        };
        let entries = Rc::new(self.entries);
        let style = self.style;
        let close_state = state.clone();
        let open_state = state.clone();

        let trigger = div()
            .id("context-trigger")
            .children(self.trigger)
            .on_mouse_down(MouseButton::Right, move |event, _, cx| {
                cx.stop_propagation();
                open_state.update(cx, |s, cx| {
                    s.open = true;
                    s.position = event.position;
                    cx.notify();
                })
            });

        Popover::new(self.id)
            .unstyled()
            .mode(PopoverTrigger::Manual)
            .open(open)
            .anchor_position(position)
            .on_open_change(move |open, _, cx| {
                close_state.update(cx, |s, cx| {
                    s.open = *open;
                    cx.notify();
                })
            })
            .trigger(trigger)
            .content_with_handle(move |popover, _, _| {
                MenuList::for_popover("context-menu", entries.clone(), popover).refine_style(&style)
            })
    }
}

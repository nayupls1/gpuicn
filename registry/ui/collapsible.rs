//! An interactive component which expands/collapses a panel.
//!
//! ```ignore
//! Collapsible::new("repos")
//!     .trigger(Button::new("toggle").ghost().icon(IconName::ChevronsUpDown))
//!     .content(div().child("@radix-ui/colors"))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Window,
    div, prelude::*,
};
use std::rc::Rc;

use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    open: Option<bool>,
    default_open: bool,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    children: Vec<AnyElement>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Collapsible);

impl Collapsible {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: None,
            default_open: false,
            trigger: None,
            content: None,
            children: Vec::new(),
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Clicking this toggles the content.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// Only rendered while open.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn on_open_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Collapsible {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_open;
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let open = self.open.unwrap_or(*state.read(cx));
        let on_open_change = self.on_open_change;

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .refine_style(&self.style)
            .children(self.trigger.map(|trigger| {
                div().id("collapsible-trigger").child(trigger).on_click(move |_, window, cx| {
                    let next = !open;
                    state.update(cx, |s, cx| {
                        *s = next;
                        cx.notify();
                    });
                    if let Some(handler) = &on_open_change {
                        handler(&next, window, cx);
                    }
                })
            }))
            // Children are always visible (e.g. the first item of a list);
            // `content` only when open.
            .children(self.children)
            .when(open, |this| this.children(self.content))
    }
}

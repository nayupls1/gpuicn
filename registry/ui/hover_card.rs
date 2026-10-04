//! For sighted users to preview content available behind a link.
//!
//! ```ignore
//! HoverCard::new("nextjs")
//!     .trigger(Button::new("t").link().label("@nextjs"))
//!     .content(|_, _| profile_preview())
//! ```

use gpui::{AnyElement, App, ElementId, IntoElement, RenderOnce, StyleRefinement, Window, prelude::*};
use std::rc::Rc;

use super::popover::{Popover, PopoverTrigger};
use super::utils::{Align, Side, StyledExt};

#[derive(IntoElement)]
pub struct HoverCard {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>>,
    side: Side,
    align: Align,
    open_delay_ms: u64,
    close_delay_ms: u64,
    style: StyleRefinement,
}

crate::impl_styled!(HoverCard);

impl HoverCard {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            side: Side::Bottom,
            align: Align::Center,
            open_delay_ms: 700,
            close_delay_ms: 300,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content<E: IntoElement>(mut self, content: impl Fn(&mut Window, &mut App) -> E + 'static) -> Self {
        self.content = Some(Rc::new(move |window, cx| content(window, cx).into_any_element()));
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn open_delay(mut self, ms: u64) -> Self {
        self.open_delay_ms = ms;
        self
    }

    pub fn close_delay(mut self, ms: u64) -> Self {
        self.close_delay_ms = ms;
        self
    }
}

impl RenderOnce for HoverCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let content = self.content;
        let mut popover = Popover::new(self.id)
            .mode(PopoverTrigger::Hover { open_delay_ms: self.open_delay_ms, close_delay_ms: self.close_delay_ms })
            .side(self.side)
            .align(self.align)
            .w_64()
            .refine_style(&self.style)
            .content(move |window, cx| match &content {
                Some(content) => content(window, cx),
                None => gpui::Empty.into_any_element(),
            });
        if let Some(trigger) = self.trigger {
            popover = popover.trigger(trigger);
        }
        popover
    }
}

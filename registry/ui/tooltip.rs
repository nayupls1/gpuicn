//! A popup that displays information related to an element when the element
//! receives hover.
//!
//! ```ignore
//! Tooltip::new("add")
//!     .trigger(Button::new("add").outline().label("Hover"))
//!     .content("Add to library")
//! ```
//!
//! For gpui's built-in `.tooltip(..)` on any stateful element use
//! [`tooltip`]: `div().id("x").tooltip(tooltip("Add to library"))`.

use gpui::{
    AnyElement, AnyView, App, Context, ElementId, IntoElement, Render, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};

use super::popover::{Popover, PopoverTrigger};
use super::theme::ActiveTheme;
use super::utils::{Align, Side, StyledExt};

#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: SharedString,
    side: Side,
    align: Align,
    delay_ms: u64,
    style: StyleRefinement,
}

crate::impl_styled!(Tooltip);

impl Tooltip {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: SharedString::default(),
            side: Side::Top,
            align: Align::Center,
            delay_ms: 300,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.content = content.into();
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

    /// Hover time before showing, in milliseconds (default 300).
    pub fn delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }
}

fn bubble(text: SharedString, style: &StyleRefinement, cx: &App) -> gpui::Div {
    let theme = cx.theme();
    div()
        .px_3()
        .py_1p5()
        .rounded(theme.radius_md())
        .bg(theme.foreground)
        .text_color(theme.background)
        .text_xs()
        .refine_style(style)
        .child(text)
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (content, style) = (self.content, self.style);
        let mut popover = Popover::new(self.id)
            .unstyled()
            .mode(PopoverTrigger::Hover { open_delay_ms: self.delay_ms, close_delay_ms: 0 })
            .side(self.side)
            .align(self.align)
            .content(move |_, cx| bubble(content.clone(), &style, cx));
        if let Some(trigger) = self.trigger {
            popover = popover.trigger(trigger);
        }
        popover
    }
}

/// A tooltip view for gpui's native `.tooltip(..)` API.
pub struct TooltipView(SharedString);

impl Render for TooltipView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        bubble(self.0.clone(), &StyleRefinement::default(), cx).mt_1()
    }
}

/// `div().id("x").tooltip(tooltip("Add to library"))`
pub fn tooltip(text: impl Into<SharedString>) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    let text = text.into();
    move |_, cx| cx.new(|_| TooltipView(text.clone())).into()
}

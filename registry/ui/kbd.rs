//! Used to display textual user input from keyboard.
//!
//! ```ignore
//! KbdGroup::new().child(Kbd::new("⌘")).child(Kbd::new("K"))
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};

use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Kbd {
    text: SharedString,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Kbd, KbdGroup);

impl Kbd {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap_1()
            .h_5()
            .min_w_5()
            .px_1()
            .rounded(theme.radius_sm())
            .bg(theme.muted)
            .text_color(theme.muted_foreground)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .refine_style(&self.style)
            .child(self.text)
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct KbdGroup {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl KbdGroup {
    pub fn new() -> Self {
        Self { children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for KbdGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().items_center().gap_1().refine_style(&self.style).children(self.children)
    }
}

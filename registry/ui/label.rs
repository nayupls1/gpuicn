//! Renders an accessible label associated with controls.

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, relative,
};

use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    disabled: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Label);

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), disabled: false, children: Vec::new(), style: StyleRefinement::default() }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .text_sm()
            .line_height(relative(1.))
            .font_weight(FontWeight::MEDIUM)
            .when(self.disabled, |this| this.opacity(0.5))
            .refine_style(&self.style)
            .child(self.text)
            .children(self.children)
    }
}

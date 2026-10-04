//! Visually or semantically separates content.

use gpui::{App, IntoElement, RenderOnce, StyleRefinement, Window, div, prelude::*};

use super::theme::ActiveTheme;
use super::utils::{Orientation, StyledExt};

#[derive(IntoElement)]
pub struct Separator {
    orientation: Orientation,
    style: StyleRefinement,
}

crate::impl_styled!(Separator);

impl Separator {
    pub fn horizontal() -> Self {
        Self { orientation: Orientation::Horizontal, style: StyleRefinement::default() }
    }

    pub fn vertical() -> Self {
        Self { orientation: Orientation::Vertical, style: StyleRefinement::default() }
    }

    pub fn new(orientation: Orientation) -> Self {
        Self { orientation, style: StyleRefinement::default() }
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex_none()
            .bg(cx.theme().border)
            .map(|this| match self.orientation {
                Orientation::Horizontal => this.h_px().w_full(),
                Orientation::Vertical => this.w_px().h_full(),
            })
            .refine_style(&self.style)
    }
}

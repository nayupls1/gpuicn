//! Displays content within a desired ratio.
//!
//! ```ignore
//! AspectRatio::new(16.0 / 9.0).child(img("photo.jpg").size_full())
//! ```

use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, StyleRefinement, Window, div, prelude::*};

use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct AspectRatio {
    ratio: f32,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(AspectRatio);

impl AspectRatio {
    /// `ratio` is width / height.
    pub fn new(ratio: f32) -> Self {
        Self { ratio, children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for AspectRatio {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().relative().w_full().overflow_hidden();
        el.style().aspect_ratio = Some(self.ratio);
        el.refine_style(&self.style).children(self.children)
    }
}

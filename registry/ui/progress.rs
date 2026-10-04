//! Displays an indicator showing the completion progress of a task.
//!
//! ```ignore
//! Progress::new(66.0).w(px(240.))
//! ```

use gpui::{App, IntoElement, RenderOnce, StyleRefinement, Window, div, prelude::*, relative};

use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Progress {
    /// 0–100
    value: f32,
    style: StyleRefinement,
}

crate::impl_styled!(Progress);

impl Progress {
    pub fn new(value: f32) -> Self {
        Self { value, style: StyleRefinement::default() }
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .relative()
            .h_2()
            .w_full()
            .overflow_hidden()
            .rounded_full()
            .bg(theme.primary.opacity(0.2))
            .refine_style(&self.style)
            .child(
                div()
                    .h_full()
                    .w(relative(self.value.clamp(0.0, 100.0) / 100.0))
                    .bg(theme.primary),
            )
    }
}

//! Use to show a placeholder while content is loading.
//!
//! ```ignore
//! Skeleton::new().h_4().w(px(250.))
//! Skeleton::new().size_12().rounded_full()
//! ```

use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, RenderOnce, StyleRefinement, Window,
    div, prelude::*,
};
use std::time::Duration;

use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Skeleton {
    id: ElementId,
    style: StyleRefinement,
}

crate::impl_styled!(Skeleton);

impl Skeleton {
    pub fn new() -> Self {
        Self { id: "skeleton".into(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .rounded(theme.radius_md())
            .bg(theme.accent)
            .refine_style(&self.style)
            .with_animation(
                self.id,
                Animation::new(Duration::from_millis(2000))
                    .repeat()
                    .with_easing(gpui::pulsating_between(0.5, 1.0)),
                |this, delta| this.opacity(delta),
            )
    }
}

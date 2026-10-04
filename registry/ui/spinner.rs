//! An indicator that can be used to show a loading state.

use gpui::{
    Animation, AnimationExt, App, IntoElement, RenderOnce, StyleRefinement, Styled, Window,
};
use std::time::Duration;

use super::icon::{Icon, IconName};
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Spinner {
    id: gpui::ElementId,
    style: StyleRefinement,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            id: "spinner".into(),
            style: StyleRefinement::default(),
        }
    }

    /// Needed only when several spinners must animate independently.
    pub fn id(mut self, id: impl Into<gpui::ElementId>) -> Self {
        self.id = id.into();
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        Icon::new(IconName::LoaderCircle)
            .refine_style(&self.style)
            .with_animation(
                self.id,
                Animation::new(Duration::from_millis(1000)).repeat(),
                |icon, delta| icon.rotate(delta * std::f32::consts::TAU),
            )
    }
}

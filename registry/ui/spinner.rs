//! An indicator that can be used to show a loading state.

use gpui::{
    Animation, AnimationExt, App, Bounds, IntoElement, PathBuilder, Pixels, RenderOnce, StyleRefinement, Styled,
    Window, canvas, div, fill, point, prelude::*,
};
use std::f32::consts::TAU;
use std::time::Duration;

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
        // Drawn as a vector arc rather than a rotated `loader-circle` icon:
        // rotating the rasterized SVG leaves stray pixels at small sizes.
        div().flex_none().size_4().refine_style(&self.style).with_animation(
            self.id,
            Animation::new(Duration::from_millis(1000)).repeat(),
            |this, delta| {
                this.child(
                    canvas(|_, _, _| {}, move |bounds, _, window, _| paint_arc(bounds, delta, window))
                        .size_full(),
                )
            },
        )
    }
}

/// A 270° arc in the current text color, proportioned like lucide's
/// `loader-circle` (radius 9 and stroke 2 in a 24px box).
fn paint_arc(bounds: Bounds<Pixels>, turn: f32, window: &mut Window) {
    let color = window.text_style().color;
    let side = bounds.size.width.min(bounds.size.height);
    let stroke = side * (2. / 24.);
    let radius = side * (9. / 24.);
    let center = bounds.center();
    let start = turn * TAU - TAU / 4.;
    let sweep = TAU * 0.75;
    let at = |angle: f32| point(center.x + radius * angle.cos(), center.y + radius * angle.sin());

    let segments = 48;
    let mut builder = PathBuilder::stroke(stroke);
    builder.move_to(at(start));
    for i in 1..=segments {
        builder.line_to(at(start + sweep * i as f32 / segments as f32));
    }
    if let Ok(path) = builder.build() {
        window.paint_path(path, color);
    }
    // Round caps.
    for end in [start, start + sweep] {
        let p = at(end);
        let r = stroke / 2.;
        window.paint_quad(fill(Bounds::from_corners(p - point(r, r), p + point(r, r)), color).corner_radii(r));
    }
}

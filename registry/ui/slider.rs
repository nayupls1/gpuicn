//! An input where the user selects a value from within a given range.
//!
//! ```ignore
//! Slider::new("volume").default_value([50.]).max(100.).step(1.).w(px(240.))
//! Slider::new("price").default_value([25., 75.]) // two thumbs = range
//! ```

use gpui::{
    App, Bounds, ElementId, IntoElement, MouseButton, Pixels, RenderOnce, StyleRefinement, Window,
    canvas, div, prelude::*, px, relative,
};
use std::rc::Rc;

use super::theme::ActiveTheme;
use super::utils::{InvisibleDrag, Orientation, StyledExt, ring_shadow};

struct SliderState {
    values: Vec<f32>,
    track: Bounds<Pixels>,
    active_thumb: usize,
}

/// Drag payload identifying which slider is being dragged.
struct SliderDrag(gpui::EntityId);

#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    min: f32,
    max: f32,
    step: f32,
    value: Option<Vec<f32>>,
    default_value: Vec<f32>,
    orientation: Orientation,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&Vec<f32>, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Slider);

impl Slider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            min: 0.,
            max: 100.,
            step: 1.,
            value: None,
            default_value: vec![0.],
            orientation: Orientation::Horizontal,
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Controlled values (one per thumb).
    pub fn value(mut self, value: impl Into<Vec<f32>>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<Vec<f32>>) -> Self {
        self.default_value = value.into();
        self
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&Vec<f32>, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_value.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| SliderState {
            values: default,
            track: Bounds::default(),
            active_thumb: 0,
        });
        if let Some(value) = &self.value {
            state.update(cx, |s, _| s.values = value.clone());
        }
        let values = state.read(cx).values.clone();
        let theme = cx.theme();
        let (min, max, step) = (self.min, self.max, self.step);
        let vertical = self.orientation == Orientation::Vertical;
        let span = (max - min).max(f32::EPSILON);
        let fraction = |v: f32| ((v - min) / span).clamp(0., 1.);
        let (lo, hi) = match values.len() {
            0 => (0., 0.),
            1 => (0., fraction(values[0])),
            _ => (
                fraction(values.iter().cloned().fold(f32::MAX, f32::min)),
                fraction(values.iter().cloned().fold(f32::MIN, f32::max)),
            ),
        };

        // Convert a pointer position into a value and update the closest thumb.
        let set_from_position = {
            let state = state.clone();
            let on_change = self.on_change.clone();
            move |position: gpui::Point<Pixels>, pick_thumb: bool, window: &mut Window, cx: &mut App| {
                let next = state.update(cx, |s, cx| {
                    let track = s.track;
                    let t = if vertical {
                        1. - ((position.y - track.top()) / track.size.height.max(px(1.)))
                    } else {
                        (position.x - track.left()) / track.size.width.max(px(1.))
                    };
                    let raw = min + t.clamp(0., 1.) * span;
                    let value = if step > 0. { ((raw - min) / step).round() * step + min } else { raw }.clamp(min, max);
                    if pick_thumb {
                        s.active_thumb = s
                            .values
                            .iter()
                            .enumerate()
                            .min_by(|a, b| (a.1 - value).abs().total_cmp(&(b.1 - value).abs()))
                            .map(|(ix, _)| ix)
                            .unwrap_or(0);
                    }
                    let ix = s.active_thumb.min(s.values.len().saturating_sub(1));
                    if s.values.get(ix) == Some(&value) {
                        return None;
                    }
                    if let Some(v) = s.values.get_mut(ix) {
                        *v = value;
                    }
                    cx.notify();
                    Some(s.values.clone())
                });
                if let (Some(next), Some(handler)) = (next, &on_change) {
                    handler(&next, window, cx);
                }
            }
        };

        let track = div()
            .relative()
            .flex_1()
            .rounded_full()
            .bg(theme.muted)
            .map(|this| if vertical { this.w_1p5().h_full() } else { this.h_1p5().w_full() })
            .child(
                div()
                    .absolute()
                    .rounded_full()
                    .bg(theme.primary)
                    .map(|this| {
                        if vertical {
                            this.w_full().bottom(relative(lo)).h(relative(hi - lo))
                        } else {
                            this.h_full().left(relative(lo)).w(relative(hi - lo))
                        }
                    }),
            )
            .child({
                let state = state.clone();
                canvas(move |bounds, _, cx| state.update(cx, |s, _| s.track = bounds), |_, _, _, _| {})
                    .absolute()
                    .size_full()
            });

        let thumbs = values.iter().enumerate().map(|(ix, value)| {
            let f = fraction(*value);
            let entity_id = state.entity_id();
            let pick = state.clone();
            div()
                .id(("thumb", ix))
                .absolute()
                .size_4()
                .rounded_full()
                .border_1()
                .border_color(theme.primary)
                .bg(gpui::white())
                .when(theme.shadows, |this| this.shadow_sm())
                .map(|this| {
                    if vertical {
                        this.bottom(relative(f)).mb(px(-8.)).left(px(-5.))
                    } else {
                        this.left(relative(f)).ml(px(-8.)).top(px(-5.))
                    }
                })
                .when(!self.disabled, |this| {
                    this.cursor_grab()
                        .hover(|s| s.shadow(vec![ring_shadow(theme.ring.opacity(0.5), px(4.))]))
                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                            cx.stop_propagation();
                            pick.update(cx, |s, _| s.active_thumb = ix);
                        })
                        .on_drag(SliderDrag(entity_id), |_, _, _, cx| cx.new(|_| InvisibleDrag))
                })
        });

        let drag_set = set_from_position.clone();
        let click_set = set_from_position;
        let entity_id = state.entity_id();

        div()
            .id(self.id)
            .relative()
            .flex()
            .items_center()
            .map(|this| if vertical { this.flex_col().h_full().min_h(px(176.)).w_4() } else { this.w_full().h_4() })
            .when(self.disabled, |this| this.opacity(0.5))
            .refine_style(&self.style)
            .child(track.children(thumbs))
            .when(!self.disabled, |this| {
                this.on_mouse_down(MouseButton::Left, move |event: &gpui::MouseDownEvent, window: &mut Window, cx: &mut App| {
                    click_set(event.position, true, window, cx)
                })
                    .on_drag_move(move |event: &gpui::DragMoveEvent<SliderDrag>, window: &mut Window, cx: &mut App| {
                        if event.drag(cx).0 == entity_id {
                            drag_set(event.event.position, false, window, cx);
                        }
                    })
            })
    }
}

//! An input where the user selects a value from within a given range.
//!
//! ```ignore
//! <Slider default_value={vec![50.0]} max={100.0f32} step={1.0f32} class="w-[240px]" />
//! <Slider default_value={vec![25.0, 75.0]} />          // two thumbs = range
//! <Slider value={volume} onChange={move |v: &Vec<f32>| volume.set(v.clone())} />
//! ```
//!
//! Press anywhere on the track to move the closest thumb there, and drag to
//! keep moving it. A focused thumb also follows the arrow keys, Home and End.

use rsx::prelude::*;
use rsx::{Div, MouseMoveEvent, Point};

use super::theme::{theme, ColorExt};
use super::utils::{is_focus_visible, keyboard_focus, on_tab_key, pointer_pressed, ring};

/// `value` snapped to `step` and clamped to `min..=max`.
pub fn snap(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let v = if step > 0.0 { ((value - min) / step).round() * step + min } else { value };
    v.clamp(min, max)
}

#[component]
pub fn Slider(
    #[prop(default = 0.0)] min: f32,
    #[prop(default = 100.0)] max: f32,
    #[prop(default = 1.0)] step: f32,
    /// Controlled values (one per thumb).
    value: Option<Accessor<Vec<f32>>>,
    /// One value per thumb; two values make a range slider.
    #[prop(default = vec![0.0])] default_value: Vec<f32>,
    vertical: bool,
    disabled: Option<Accessor<bool>>,
    on_change: Option<Callback<Vec<f32>>>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let state = signal(default_value);
    let values = memo(move || value.map(|v| v.get()).unwrap_or_else(|| state.get()));
    let disabled = memo(move || disabled.map(|d| d.get()).unwrap_or(false));
    let track = NodeRef::new();
    let active_thumb = signal(0usize);
    // A thumb is held down (pressed or being dragged).
    let dragging = signal(false);
    let hovered = signal(false);
    let thumbs: StoredValue<Vec<NodeRef>> = StoredValue::new(Vec::new());

    let span = (max - min).max(f32::EPSILON);
    let fraction = move |v: f32| ((v - min) / span).clamp(0.0, 1.0);

    // Set the active thumb to `value`, and report the change.
    let set_value = move |ix: usize, value: f32| {
        let mut next = values.get_untracked();
        let value = snap(value, min, max, step);
        match next.get_mut(ix) {
            Some(v) if *v != value => *v = value,
            _ => return,
        }
        state.set(next.clone());
        if let Some(f) = on_change {
            f.call(&next);
        }
    };
    // Convert a pointer position into a value for the active (or closest) thumb.
    let set_from_position = move |position: Point, pick_thumb: bool| {
        let Some(b) = untrack(|| track.bounds()) else { return };
        let t = if vertical {
            1.0 - (position.y - b.origin.y) / b.size.height.max(1.0)
        } else {
            (position.x - b.origin.x) / b.size.width.max(1.0)
        };
        let value = snap(min + t.clamp(0.0, 1.0) * span, min, max, step);
        if pick_thumb {
            let closest = values.with_untracked(|vs| {
                vs.iter()
                    .enumerate()
                    .min_by(|a, b| (a.1 - value).abs().total_cmp(&(b.1 - value).abs()))
                    .map(|(ix, _)| ix)
                    .unwrap_or(0)
            });
            active_thumb.set(closest);
        }
        set_value(active_thumb.get_untracked(), value);
    };
    let thumb_count = memo(move || values.with(|v| v.len()));

    <div
        class="relative flex items-center"
        class:flex-col={vertical}
        class:h-full={vertical}
        class:min-h-[176px]={vertical}
        class:w-4={vertical}
        class:w-full={!vertical}
        class:h-4={!vertical}
        opacity={if disabled.get() { 0.5 } else { 1.0 }}
        onHover={move |h: &bool| hovered.set(*h)}
        onMouseDown={move |e: &MouseEvent| {
            if disabled.get_untracked() || e.button != MouseButton::Left {
                return;
            }
            pointer_pressed();
            set_from_position(e.position, true);
            dragging.set(true);
            // Like Radix, the moved thumb takes focus (so arrow keys follow).
            let ix = active_thumb.get_untracked();
            thumbs.with_value(|t| t.get(ix).map(|n| n.focus()));
        }}
        {...attrs}
    >
        <div
            ref={track}
            class="relative flex-1 rounded-full"
            class:w-1.5={vertical}
            class:h-full={vertical}
            class:h-1.5={!vertical}
            class:w-full={!vertical}
            bg={t.get().muted}
        >
            {move || {
                let (lo, hi) = values.with(|vs| match vs.len() {
                    0 => (0.0, 0.0),
                    1 => (0.0, fraction(vs[0])),
                    _ => (
                        fraction(vs.iter().cloned().fold(f32::MAX, f32::min)),
                        fraction(vs.iter().cloned().fold(f32::MIN, f32::max)),
                    ),
                });
                if vertical {
                    <div class="absolute w-full rounded-full" bg={t.get().primary} bottom={relative(lo)} h={relative(hi - lo)} />
                } else {
                    <div class="absolute h-full rounded-full" bg={t.get().primary} left={relative(lo)} w={relative(hi - lo)} />
                }
            }}
            {move || {
                // Rebuilt only when the number of thumbs changes, so a focused
                // thumb keeps focus while it moves.
                let n = thumb_count.get();
                let refs: Vec<NodeRef> = (0..n).map(|_| NodeRef::new()).collect();
                thumbs.set_value(refs.clone());
                refs.into_iter().enumerate().map(|(ix, node)| {
                    <div
                        ref={node}
                        class="absolute size-4 rounded-full border"
                        class:cursor-grab={!disabled.get()}
                        border_color={t.get().primary}
                        bg={t.get().background}
                        bind={move |el: Div| {
                            let f = values.with(|vs| vs.get(ix).copied().map(fraction).unwrap_or(0.0));
                            if vertical { el.bottom(relative(f)).mb(px(-8.0)).left(px(-5.0)) } else { el.left(relative(f)).ml(px(-8.0)).top(px(-5.0)) }
                        }}
                        onMouseDown={move |e: &MouseEvent| {
                            if disabled.get_untracked() || e.button != MouseButton::Left {
                                return;
                            }
                            e.stop_propagation();
                            pointer_pressed();
                            active_thumb.set(ix);
                            dragging.set(true);
                            node.focus();
                        }}
                        onKeyDown={move |e: &KeyboardEvent| {
                            on_tab_key(e);
                            if disabled.get_untracked() {
                                return;
                            }
                            let current = values.with_untracked(|vs| vs.get(ix).copied().unwrap_or(min));
                            let big = (span / 10.0).max(step);
                            let next = match e.key.as_ref() {
                                "ArrowRight" | "ArrowUp" => current + step,
                                "ArrowLeft" | "ArrowDown" => current - step,
                                "PageUp" => current + big,
                                "PageDown" => current - big,
                                "Home" => min,
                                "End" => max,
                                _ => return,
                            };
                            e.prevent_default();
                            keyboard_focus();
                            active_thumb.set(ix);
                            set_value(ix, next);
                        }}
                    >
                        {move || {
                            // `hover:ring-4` / `focus-visible:ring-4`: the active thumb is ringed
                            // while the pointer is on the slider, and while it's held or dragged.
                            let active = active_thumb.get() == ix && (dragging.get() || hovered.get());
                            let show = !disabled.get() && (active || is_focus_visible(node));
                            show.then(|| ring(t.get().ring.opacity(0.5), 4.0, 8.0, 1.0))
                        }}
                    </div>
                }).collect::<Vec<_>>()
            }}
        </div>
        {/* rsx-gpui only sends `onMouseMove` / `onMouseUp` to the element under the
            pointer, so while dragging a transparent layer over the whole window
            receives them, wherever the pointer goes. */}
        {move || dragging.get().then(|| <Portal>
            <div
                class="absolute inset-0 cursor-grabbing"
                onMouseMove={move |e: &MouseMoveEvent| {
                    if e.pressed_button == Some(MouseButton::Left) {
                        set_from_position(e.position, false);
                    } else {
                        // Released outside the window.
                        dragging.set(false);
                    }
                }}
                onMouseUp={move |_| {
                    dragging.set(false);
                    // The press's click focuses the track (it has a `ref`, so rsx-gpui
                    // makes it focusable): hand focus back to the thumb.
                    let ix = active_thumb.get_untracked();
                    thumbs.with_value(|t| t.get(ix).map(|n| n.focus()));
                }}
            />
        </Portal>)}
    </div>
}

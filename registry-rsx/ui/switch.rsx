//! A control that allows the user to toggle between checked and not checked.
//!
//! ```ignore
//! <Switch label="Airplane Mode" />
//! <Switch checked={wifi} onChange={move |on: &bool| wifi.set(*on)} label="Wi-Fi" />
//! ```
//!
//! Click it, or focus it and press Space / Enter.

use rsx::prelude::*;

use super::label::Label;
use super::theme::{theme, ColorExt};
use super::utils::{ease_in_out, is_focus_visible, on_tab_key, pointer_pressed, ring, use_animation_clock};

/// How far the 16px thumb travels in the 32×18.4 track (1px border).
const TRAVEL: f32 = 14.0;
/// Thumb slide duration, in seconds.
const DURATION: f32 = 0.15;

#[component]
pub fn Switch(
    /// Controlled checked state.
    checked: Option<Accessor<bool>>,
    default_checked: bool,
    disabled: Option<Accessor<bool>>,
    label: Option<Accessor<String>>,
    on_change: Option<Callback<bool>>,
    /// Style overrides, applied to the track (like gpuicn's `Styled`).
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let node = NodeRef::new();
    let state = signal(default_checked);
    let is_checked = memo(move || checked.map(|c| c.get()).unwrap_or_else(|| state.get()));
    let disabled = memo(move || disabled.map(|d| d.get()).unwrap_or(false));

    // The thumb slides over 150ms: remember where it started and when.
    let clock = use_animation_clock();
    let offset_of = |on: bool| if on { TRAVEL } else { 0.0 };
    let slide = signal((f32::NEG_INFINITY, offset_of(is_checked.get_untracked())));
    let thumb_offset = move || {
        let to = offset_of(is_checked.get());
        let (start, from) = slide.get();
        // Only subscribe to the clock while sliding, so an idle switch never redraws.
        if from == to || clock.get_untracked() - start >= DURATION {
            return to;
        }
        from + (to - from) * ease_in_out((clock.get() - start) / DURATION)
    };
    let last = StoredValue::new(is_checked.get_untracked());
    effect(move || {
        let on = is_checked.get();
        if last.get_value() != on {
            last.set_value(on);
            // Start from wherever the thumb is now (it may be mid-slide).
            let from = untrack(|| {
                let (start, from) = slide.get();
                let t = ((clock.get() - start) / DURATION).clamp(0.0, 1.0);
                let prev_to = offset_of(!on);
                if t >= 1.0 { prev_to } else { from + (prev_to - from) * ease_in_out(t) }
            });
            slide.set((clock.get_untracked(), from));
        }
    });

    <div
        ref={node}
        class="flex items-center gap-2"
        class:cursor-pointer={!disabled.get()}
        opacity={if disabled.get() { 0.5 } else { 1.0 }}
        onMouseDown={move |_| pointer_pressed()}
        onKeyDown={on_tab_key}
        onClick={move |e: &ClickEvent| {
            if disabled.get_untracked() {
                return;
            }
            e.stop_propagation();
            let next = !is_checked.get_untracked();
            state.set(next);
            if let Some(f) = on_change {
                f.call(&next);
            }
        }}
    >
        {move || {
            let theme = t.get();
            let on = is_checked.get();
            let (track, thumb) = match (on, theme.is_dark()) {
                (true, false) => (theme.primary, theme.background),
                (true, true) => (theme.primary, theme.primary_foreground),
                (false, false) => (theme.input, theme.background),
                (false, true) => (theme.input.opacity(0.8), theme.foreground),
            };
            // rsx#7: no `shadow-xs` on the light track.
            <div
                class="relative flex flex-none items-center w-8 h-[18.4px] rounded-full border border-transparent"
                bg={track}
                {...&attrs}
            >
                <div class="size-4 rounded-full" bg={thumb} ml={px(thumb_offset())} />
                {(is_focus_visible(node) && !disabled.get()).then(|| ring(theme.ring.opacity(0.5), 3.0, 9.2, 1.0))}
            </div>
        }}
        {label.map(|l| <Label text={l} />)}
    </div>
}

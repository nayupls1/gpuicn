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
use super::utils::{is_focus_visible, on_tab_key, pointer_pressed, reduced_motion, ring};

/// How far the 16px thumb travels in the 32×18.4 track (1px border).
const TRAVEL: f32 = 14.0;

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
    // `(track, thumb)` colors.
    let colors = memo(move || {
        let theme = t.get();
        match (is_checked.get(), theme.is_dark()) {
            (true, false) => (theme.primary, theme.background),
            (true, true) => (theme.primary, theme.primary_foreground),
            (false, false) => (theme.input, theme.background),
            (false, true) => (theme.input.opacity(0.8), theme.foreground),
        }
    });
    // The thumb slides over 150ms (gpuicn's `with_animation`).
    let slide = if reduced_motion() { "" } else { "transition-transform duration-150 ease-in-out" };

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
        <div
            class="relative flex flex-none items-center w-8 h-[18.4px] rounded-full border border-transparent"
            class:shadow-xs={move || t.with(|theme| theme.shadows && !theme.is_dark())}
            bg={move || colors.get().0}
            {...attrs}
        >
            <div
                class={format!("size-4 rounded-full {slide}")}
                bg={move || colors.get().1}
                translate_x={move || px(if is_checked.get() { TRAVEL } else { 0.0 })}
            />
            {move || {
                let theme = t.get();
                (is_focus_visible(node) && !disabled.get()).then(|| ring(theme.ring.opacity(0.5), 3.0, 9.2, 1.0))
            }}
        </div>
        {label.map(|l| <Label text={l} />)}
    </div>
}

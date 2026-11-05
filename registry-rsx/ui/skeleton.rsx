//! Use to show a placeholder while content is loading.
//!
//! ```ignore
//! <Skeleton class="h-4 w-[250px]" />
//! <Skeleton class="size-12 rounded-full" />
//! ```

use rsx::prelude::*;

use super::theme::theme;
use super::utils::use_animation_clock;

/// gpui's `pulsating_between(min, max)` at `delta` (0..1 over one period).
pub fn pulsating_between(min: f32, max: f32, delta: f32) -> f32 {
    let t = (delta * 2.0 * std::f32::consts::PI).sin();
    let breath = (t * t * t + t) / 2.0;
    min + (breath + 1.0) / 2.0 * (max - min)
}

/// Pulse period, in seconds.
const PERIOD: f32 = 2.0;

#[component]
pub fn Skeleton(attrs: Attributes) -> impl IntoElement {
    let t = theme();
    // rsx#16: no animations, so the pulse is driven by the animation clock.
    let clock = use_animation_clock();
    move || {
        let theme = t.get();
        <div
            corner_radius={theme.radius_md()}
            bg={theme.accent}
            {...&attrs}
            opacity={pulsating_between(0.5, 1.0, (clock.get() / PERIOD).fract())}
        />
    }
}

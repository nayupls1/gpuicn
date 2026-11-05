//! Displays an indicator showing the completion progress of a task.
//!
//! ```ignore
//! <Progress value={66.0f32} class="w-[240px]" />
//! <Progress value={progress} />   // a signal: the bar follows it
//! ```

use rsx::prelude::*;

use super::theme::{theme, ColorExt};

#[component]
pub fn Progress(
    /// 0–100
    value: Accessor<f32>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();
        <div class="relative h-2 w-full overflow-hidden rounded-full" bg={theme.primary.opacity(0.2)} {...&attrs}>
            {/* GPUI clips overflow to a rectangle, not the rounded track,
                so the indicator carries its own rounding. */}
            <div class="h-full rounded-full" bg={theme.primary} w={relative(value.get().clamp(0.0, 100.0) / 100.0)} />
        </div>
    }
}

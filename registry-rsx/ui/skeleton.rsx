//! Use to show a placeholder while content is loading.
//!
//! ```ignore
//! <Skeleton class="h-4 w-[250px]" />
//! <Skeleton class="size-12 rounded-full" />
//! ```

use rsx::prelude::*;

use super::theme::theme;
use super::utils::reduced_motion;

/// A pulsing block: `animate-pulse` (opacity 1 → 0.5 → 1 every 2s, like
/// gpuicn's `pulsating_between(0.5, 1.0)`), held still with reduced motion.
#[component]
pub fn Skeleton(attrs: Attributes) -> impl IntoElement {
    let t = theme();
    <div
        class:animate-pulse={!reduced_motion()}
        corner_radius={t.get().radius_md()}
        bg={t.get().accent}
        {...attrs}
    />
}

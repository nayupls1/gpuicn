//! An indicator that can be used to show a loading state.
//!
//! ```ignore
//! <Spinner />
//! <Spinner class="size-6 text-red-500" />
//! ```

use rsx::prelude::*;

use super::utils::reduced_motion;

/// A 270° arc in the current text color, proportioned like lucide's
/// `loader-circle` (radius 9 and stroke 2 in a 24px box), turning once a
/// second (`animate-spin`; still with reduced motion).
#[component]
pub fn Spinner(attrs: Attributes) -> impl IntoElement {
    <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        class="size-4 flex-none"
        class:animate-spin={!reduced_motion()}
        {...attrs}
    >
        <path d="M21 12a9 9 0 1 1-6.219-8.56" />
    </svg>
}

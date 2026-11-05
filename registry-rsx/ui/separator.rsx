//! Visually or semantically separates content.
//!
//! ```ignore
//! <Separator />
//! <Separator vertical />
//! <Separator class="my-4" />
//! ```

use rsx::prelude::*;

use super::theme::theme;
use super::utils::Orientation;

#[component]
pub fn Separator(
    #[prop(optional)] orientation: Orientation,
    /// Shorthand for `orientation={Orientation::Vertical}`.
    vertical: bool,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let vertical = vertical || orientation == Orientation::Vertical;
    <div
        class="flex-none"
        class:h-px={!vertical}
        class:w-full={!vertical}
        class:w-px={vertical}
        class:h-full={vertical}
        bg={t.get().border}
        {...attrs}
    />
}

//! Displays content within a desired ratio.
//!
//! ```ignore
//! <AspectRatio ratio={16.0f32 / 9.0} class="w-[450px]">
//!     <img src="photo.jpg" class="size-full" />
//! </AspectRatio>
//! ```

use rsx::prelude::*;

#[component]
pub fn AspectRatio(
    /// width / height
    ratio: f32,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let ratio = if ratio > 0.0 { ratio } else { 1.0 };
    // rsx#N: the style model has no `aspect-ratio`. A percentage top padding
    // is a fraction of the parent's width (in CSS and in GPUI's layout), so
    // a spacer sets the height and the children fill the box on top of it.
    <div class="relative w-full overflow-hidden" {...attrs}>
        <div class="w-full" pt={relative(1.0 / ratio)} />
        <div class="absolute inset-0">{children}</div>
    </div>
}

//! Displays content within a desired ratio.
//!
//! ```ignore
//! <AspectRatio ratio={16.0 / 9.0} class="w-[450px]">
//!     <img src="photo.jpg" class="size-full object-cover" />
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
    <div class="relative w-full overflow-hidden" aspect_ratio={ratio} {...attrs}>
        {children}
    </div>
}

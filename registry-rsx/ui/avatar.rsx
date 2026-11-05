//! An image element with a fallback for representing the user.
//!
//! ```ignore
//! <Avatar src="https://github.com/shadcn.png" fallback="CN" />
//! <Avatar src={concat!(env!("CARGO_MANIFEST_DIR"), "/assets/me.png")} fallback="ME" class="rounded-lg" />
//! ```

use rsx::prelude::*;

use super::theme::theme;

#[component]
pub fn Avatar(
    /// URL or file path, as `<img src>` takes it.
    src: Option<Accessor<String>>,
    /// Shown while loading or when there is no image (usually initials).
    #[prop(optional)]
    fallback: Accessor<String>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    // GPUI clips overflow to a rectangle, so the image and fallback carry the
    // avatar's corner radius themselves (`rounded-full` unless overridden).
    let radius = div().spread(&attrs).computed_style().corner_radii.top_left.unwrap_or(px(9999.0));
    // rsx has no image load/error events, so the fallback is drawn under the
    // image: it shows while the image loads, and stays if it fails.
    <div class="relative flex flex-none size-8 overflow-hidden rounded-full" {...attrs}>
        <div
            class="absolute inset-0 flex items-center justify-center text-sm leading-[1.25rem] font-medium"
            corner_radius={radius}
            bg={t.get().muted}
        >
            {fallback}
        </div>
        {src.map(|s| <img src={move || s.get()} alt="" class="absolute inset-0 size-full object-cover" corner_radius={radius} />)}
    </div>
}

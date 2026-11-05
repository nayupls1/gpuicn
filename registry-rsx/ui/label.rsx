//! Renders an accessible label associated with controls.
//!
//! ```ignore
//! <Label text="Email" />
//! <Label text="Accept terms" disabled />
//! ```

use rsx::prelude::*;

#[component]
pub fn Label(
    text: Option<Accessor<String>>,
    disabled: Option<Accessor<bool>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    <div
        class="flex items-center gap-2 text-sm leading-none font-medium"
        opacity={if disabled.map(|d| d.get()).unwrap_or(false) { 0.5 } else { 1.0 }}
        {...attrs}
    >
        {text}
        {children}
    </div>
}

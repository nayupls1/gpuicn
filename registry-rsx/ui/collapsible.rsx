//! An interactive component which expands/collapses a panel.
//!
//! ```ignore
//! <Collapsible
//!     trigger={<Button ghost icon={IconName::ChevronsUpDown} />}
//!     content={<div>"@radix-ui/colors"</div>}
//! >
//!     <div>"@radix-ui/primitives"</div>
//! </Collapsible>
//! ```

use rsx::prelude::*;

#[component]
pub fn Collapsible(
    /// Whether the content is shown (controlled).
    open: Option<Accessor<bool>>,
    /// Whether the content starts shown (uncontrolled).
    default_open: bool,
    /// Clicking this toggles the content.
    trigger: Option<Children>,
    /// Only rendered while open.
    content: Option<Children>,
    on_open_change: Option<Callback<bool>>,
    /// Always visible (e.g. the first item of a list), between the trigger and `content`.
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let state = signal(default_open);
    let is_open = move || open.map(|o| o.get()).unwrap_or_else(|| state.get());
    let toggle = move || {
        let next = !untrack(is_open);
        state.set(next);
        if let Some(f) = on_open_change {
            f.call(&next);
        }
    };
    <div class="flex flex-col" {...attrs}>
        {trigger.map(|trigger| <div onClick={move |_| toggle()}>{trigger}</div>)}
        {children}
        {move || is_open().then(|| content)}
    </div>
}

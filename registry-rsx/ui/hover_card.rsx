//! For sighted users to preview content available behind a link.
//!
//! ```ignore
//! <HoverCard
//!     trigger={<Button link label="@nextjs" />}
//!     content={|| profile_preview()}
//! />
//! ```

use rsx::prelude::*;

use super::popover::{Popover, PopoverTrigger};
use super::utils::{Align, Side};

#[component]
pub fn HoverCard(
    /// The element that shows the card when hovered.
    trigger: Option<Children>,
    /// The card's content, built each time it opens.
    content: Option<ChildrenFn>,
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    /// Hover time before opening, in milliseconds (default 700).
    #[prop(default = 700)]
    open_delay: u64,
    /// Time before closing once the pointer leaves, in milliseconds (default 300).
    #[prop(default = 300)]
    close_delay: u64,
    /// Styles for the card (default `w-64`).
    attrs: Attributes,
) -> impl IntoElement {
    <Popover
        mode={PopoverTrigger::Hover { open_delay_ms: open_delay, close_delay_ms: close_delay }}
        side={side}
        align={align}
        trigger={move || trigger.map(|t| t.render())}
        class="w-64"
        {...attrs}
        content={move |_| content.map(|c| c.render())}
    />
}

//! Draws floating UI (popovers, menus, dialogs, sheets, tooltips, toasts) on
//! top of the rest of the window: the equivalent of a React portal.
//!
//! rsx has portals built in (`<Portal>`), so unlike gpuicn's gpui version
//! there is no layer store here. [`PortalHost`] is kept so apps written for
//! gpuicn keep their shape; it renders nothing.
//!
//! ```ignore
//! <div class="size-full">
//!     <MainContent />
//!     <PortalHost />
//! </div>
//! ```

use rsx::prelude::*;

/// Kept for parity with gpuicn: rsx targets draw `<Portal>` content above
/// the window by themselves, so this renders nothing.
#[component]
pub fn PortalHost() -> impl IntoElement {}

/// A layer covering the whole window, drawn above everything else.
/// `viewport` is the layer's `NodeRef` (its bounds are the window's; for just
/// the size, `rsx::window_size()` is simpler).
#[component]
pub fn FullWindow(viewport: Option<NodeRef>, children: Children, attrs: Attributes) -> impl IntoElement {
    let viewport = viewport.unwrap_or_default();
    <Portal>
        <div class="absolute inset-0" ref={viewport} {...attrs}>{children}</div>
    </Portal>
}

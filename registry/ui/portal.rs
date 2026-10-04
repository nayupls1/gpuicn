//! Draws floating UI (popovers, menus, dialogs, sheets, tooltips, toasts) on
//! top of the rest of the window — the equivalent of a React portal.
//!
//! gpui can't nest `deferred()` elements, so instead every overlay registers a
//! *layer* while rendering and a single [`PortalHost`] draws them all, in
//! order, after the rest of the window. Layers opened from inside other layers
//! (a select inside a dialog, a submenu) just work.
//!
//! **Add the host once, as the last child of each window's root view:**
//!
//! ```ignore
//! impl Render for MyApp {
//!     fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         div()
//!             .size_full()
//!             .child(self.main_content())
//!             .child(PortalHost::new()) // ← last child
//!     }
//! }
//! ```

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, Global, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Position, Style, Window, WindowId,
};
use std::collections::HashMap;

type LayerBuilder = Box<dyn FnOnce(&mut Window, &mut App) -> AnyElement>;

#[derive(Default)]
struct PortalStore {
    pending: HashMap<WindowId, Vec<(ElementId, LayerBuilder)>>,
    warned: bool,
}

impl Global for PortalStore {}

/// Render `builder`'s element above everything else for this frame.
///
/// Call it from a component's `render` every frame the layer should be
/// visible. `key` must be stable and unique per layer; it namespaces the
/// layer's element ids so keyed state inside it survives across frames.
pub fn portal(
    key: impl Into<ElementId>,
    window: &mut Window,
    cx: &mut App,
    builder: impl FnOnce(&mut Window, &mut App) -> AnyElement + 'static,
) {
    let window_id = window.window_handle().window_id();
    let key = key.into();
    let store = cx.default_global::<PortalStore>();
    let pending = store.pending.entry(window_id).or_default();
    // Still queued from a previous frame → nobody is draining this window.
    if pending.iter().any(|(existing, _)| *existing == key) {
        pending.retain(|(existing, _)| *existing != key);
        if !store.warned {
            store.warned = true;
            eprintln!(
                "gpuicn: an overlay was opened but no `PortalHost` is rendered in this window. \
                 Add `.child(PortalHost::new())` as the last child of your root view."
            );
        }
    }
    store.pending.entry(window_id).or_default().push((key, Box::new(builder)));
}

/// Paints every registered layer. Place it last in your root view.
pub struct PortalHost;

impl PortalHost {
    pub fn new() -> Self {
        PortalHost
    }
}

impl Default for PortalHost {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for PortalHost {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct PortalLayers(Vec<(ElementId, AnyElement)>);

impl Element for PortalHost {
    type RequestLayoutState = PortalLayers;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let window_id = window.window_handle().window_id();

        let mut layers = Vec::new();
        let mut layout_ids = Vec::new();
        // Building a layer can register more layers (nested overlays), so keep
        // draining until nothing new shows up.
        loop {
            let pending = cx
                .default_global::<PortalStore>()
                .pending
                .remove(&window_id)
                .unwrap_or_default();
            if pending.is_empty() {
                break;
            }
            for (key, builder) in pending {
                let (element, layout_id) = window.with_element_namespace(key.clone(), |window| {
                    let mut element = builder(window, cx);
                    let layout_id = element.request_layout(window, cx);
                    (element, layout_id)
                });
                layout_ids.push(layout_id);
                layers.push((key, element));
            }
        }

        let mut style = Style::default();
        style.position = Position::Absolute;
        (window.request_layout(style, layout_ids, cx), PortalLayers(layers))
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        layers: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for (key, element) in &mut layers.0 {
            window.with_element_namespace(key.clone(), |window| {
                element.prepaint(window, cx);
            });
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        layers: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for (key, element) in &mut layers.0 {
            window.with_element_namespace(key.clone(), |window| {
                element.paint(window, cx);
            });
        }
    }
}

/// Wrap `content` in a full-window box (window origin, viewport size), used
/// for modal backdrops.
pub fn full_window(content: gpui::Div, window: &Window) -> impl IntoElement {
    use gpui::{ParentElement, Styled, anchored, point, px};
    let size = window.viewport_size();
    anchored()
        .position(point(px(0.), px(0.)))
        .child(content.w(size.width).h(size.height))
}

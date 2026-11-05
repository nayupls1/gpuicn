//! Resizable panel groups and layouts.
//!
//! ```ignore
//! <ResizablePanelGroup with_handle>
//!     <ResizablePanel default_size={50.0f32}><div>One</div></ResizablePanel>
//!     <ResizablePanel default_size={50.0f32}>
//!         <ResizablePanelGroup vertical>
//!             <ResizablePanel default_size={25.0f32}><div>Two</div></ResizablePanel>
//!             <ResizablePanel default_size={75.0f32}><div>Three</div></ResizablePanel>
//!         </ResizablePanelGroup>
//!     </ResizablePanel>
//! </ResizablePanelGroup>
//! ```
//!
//! A handle sits between neighbouring panels; drag it to resize them.

use rsx::prelude::*;
use rsx::{Div, MouseMoveEvent};

use super::icon::{Icon, IconName};
use super::theme::theme;
use super::utils::{Orientation, Siblings};

#[derive(Clone, Copy)]
struct PanelInfo {
    id: u64,
    default_size: Option<f32>,
    min_size: f32,
    max_size: f32,
}

#[derive(Clone, Copy)]
struct GroupContext {
    vertical: bool,
    with_handle: bool,
    panels: StoredValue<Vec<PanelInfo>>,
    order: Siblings,
    /// Sizes after a drag, in percent; `None` until the first one.
    sizes: Signal<Option<Vec<f32>>>,
    /// The handle being dragged (the index of the panel after it).
    dragging: Signal<Option<usize>>,
    bounds: NodeRef,
    on_resize: Option<Callback<Vec<f32>>>,
}

impl GroupContext {
    /// The panels in order (tracked).
    fn ordered(&self) -> Vec<PanelInfo> {
        let len = self.order.len();
        let mut panels = self.panels.get_value();
        panels.sort_by_key(|p| self.order.index(p.id).unwrap_or(len));
        panels
    }

    /// The panel sizes in percent (tracked): the dragged ones, or the
    /// defaults, splitting what's left among panels without one.
    fn sizes(&self) -> Vec<f32> {
        let panels = self.ordered();
        if let Some(sizes) = self.sizes.get().filter(|s| s.len() == panels.len()) {
            return sizes;
        }
        let given: f32 = panels.iter().filter_map(|p| p.default_size).sum();
        let unset = panels.iter().filter(|p| p.default_size.is_none()).count();
        let fill = if unset > 0 { ((100.0 - given) / unset as f32).max(0.0) } else { 0.0 };
        panels.iter().map(|p| p.default_size.unwrap_or(fill)).collect()
    }

    /// Moves the handle before panel `ix` to the pointer.
    fn drag_to(&self, ix: usize, e: &MouseMoveEvent) {
        let Some(bounds) = self.bounds.bounds() else { return };
        let (pos, origin, extent) = if self.vertical {
            (e.position.y, bounds.origin.y, bounds.size.height)
        } else {
            (e.position.x, bounds.origin.x, bounds.size.width)
        };
        if extent <= 0.0 || ix == 0 {
            return;
        }
        let (panels, mut sizes) = untrack(|| (self.ordered(), self.sizes()));
        if ix >= sizes.len() {
            return;
        }
        let pointer = ((pos - origin) / extent * 100.0).clamp(0.0, 100.0);
        let a_ix = ix - 1;
        let before: f32 = sizes[..a_ix].iter().sum();
        let pair = sizes[a_ix] + sizes[ix];
        let (a, b) = (panels[a_ix], panels[ix]);
        let lo = a.min_size.max(pair - b.max_size);
        let hi = a.max_size.min(pair - b.min_size);
        let size = (pointer - before).clamp(lo, hi.max(lo));
        sizes[a_ix] = size;
        sizes[ix] = pair - size;
        self.sizes.set(Some(sizes.clone()));
        if let Some(f) = self.on_resize {
            f.call(&sizes);
        }
    }
}

#[component]
pub fn ResizablePanelGroup(
    #[prop(optional)] orientation: Orientation,
    /// Stack the panels top to bottom.
    vertical: bool,
    /// Show a grip on the handles (`<ResizableHandle withHandle />`).
    with_handle: bool,
    /// Called with the panel sizes (percent) while dragging.
    on_resize: Option<Callback<Vec<f32>>>,
    /// [`ResizablePanel`]s.
    children: Option<ChildrenFn>,
    attrs: Attributes,
) -> impl IntoElement {
    let vertical = vertical || orientation == Orientation::Vertical;
    let context = GroupContext {
        vertical,
        with_handle,
        panels: StoredValue::new(Vec::new()),
        order: Siblings::new(),
        sizes: signal(None),
        dragging: signal(None),
        bounds: NodeRef::new(),
        on_resize,
    };
    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div ref={context.bounds} class="relative flex size-full" class:flex-col={vertical} {...attrs}>
        <Provider value={context}>{children}</Provider>
        {/* While dragging a handle, a transparent full-window layer follows the
            pointer: rsx's `onMouseMove` only fires over the element under it. */}
        {move || context.dragging.get().map(|ix| <Portal>
            <div
                class="absolute inset-0"
                onMouseMove={move |e: &MouseMoveEvent| {
                    if e.pressed_button.is_none() {
                        context.dragging.set(None);
                    } else {
                        context.drag_to(ix, e);
                    }
                }}
                onMouseUp={move |_| context.dragging.set(None)}
            />
        </Portal>)}
    </div>
}

#[component]
pub fn ResizablePanel(
    /// Initial size in percent of the group.
    default_size: Option<f32>,
    #[prop(default = 0.0)] min_size: f32,
    #[prop(default = 100.0)] max_size: f32,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let context = expect_context::<GroupContext>();
    let id = batch(|| {
        let id = context.order.register();
        context.panels.update_value(|p| p.push(PanelInfo { id, default_size, min_size, max_size }));
        id
    });
    on_cleanup(move || {
        if context.panels.is_alive() {
            context.panels.update_value(|p| p.retain(|p| p.id != id));
        }
    });
    let vertical = context.vertical;
    let t = theme();
    let index = move || context.order.index(id).unwrap_or(0);
    let size = move || context.sizes().get(index()).copied().unwrap_or(0.0) / 100.0;

    // The handle between this panel and the one before it.
    let handle = move || {
        let ix = index();
        let start_drag = move |e: &MouseEvent| {
            e.stop_propagation();
            context.dragging.set(Some(ix));
        };
        (ix > 0).then(|| <div
            class="relative flex flex-none items-center justify-center"
            class:h-px={vertical}
            class:w-full={vertical}
            class:w-px={!vertical}
            class:h-full={!vertical}
            bg={t.get().border}
            onMouseDown={start_drag}
        >
            {/* Wider invisible hit area. rsx gap: no `col-resize` / `row-resize` cursors. */}
            <div
                class="absolute"
                bind={move |el: Div| if vertical { el.w_full().h(px(8.0)).top(px(-4.0)) } else { el.h_full().w(px(8.0)).left(px(-4.0)) }}
                onMouseDown={start_drag}
            />
            {context.with_handle.then(|| <div
                class="flex flex-none items-center justify-center rounded-[2px] border"
                class:h-3={vertical}
                class:w-4={vertical}
                class:h-4={!vertical}
                class:w-3={!vertical}
                border_color={t.get().border}
                bg={t.get().border}
            >
                <Icon name={if vertical { IconName::GripHorizontal } else { IconName::GripVertical }} class="size-2.5" />
            </div>)}
        </div>)
    };

    <>
        {handle}
        <div
            class="flex flex-col overflow-hidden"
            class:w-full={vertical}
            class:h-full={!vertical}
            bind={move |el: Div| if vertical { el.h(relative(size())) } else { el.w(relative(size())) }}
            {...attrs}
        >
            {children}
        </div>
    </>
}

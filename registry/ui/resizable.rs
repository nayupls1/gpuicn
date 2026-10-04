//! Accessible resizable panel groups and layouts with keyboard support.
//!
//! ```ignore
//! ResizablePanelGroup::new("layout")
//!     .panel(ResizablePanel::new(div().child("One")).default_size(50.))
//!     .panel(ResizablePanel::new(
//!         ResizablePanelGroup::new("nested").vertical()
//!             .panel(ResizablePanel::new(div().child("Two")).default_size(25.))
//!             .panel(ResizablePanel::new(div().child("Three")).default_size(75.)),
//!     ).default_size(50.))
//!     .with_handle()
//! ```

use gpui::{
    AnyElement, App, Bounds, ElementId, EntityId, IntoElement, Pixels, RenderOnce, StyleRefinement,
    Window, canvas, div, prelude::*, px, relative,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::theme::ActiveTheme;
use super::utils::{InvisibleDrag, Orientation, StyledExt};

pub struct ResizablePanel {
    content: AnyElement,
    default_size: Option<f32>,
    min_size: f32,
    max_size: f32,
}

impl ResizablePanel {
    pub fn new(content: impl IntoElement) -> Self {
        Self { content: content.into_any_element(), default_size: None, min_size: 0., max_size: 100. }
    }

    /// Initial size in percent of the group.
    pub fn default_size(mut self, percent: f32) -> Self {
        self.default_size = Some(percent);
        self
    }

    pub fn min_size(mut self, percent: f32) -> Self {
        self.min_size = percent;
        self
    }

    pub fn max_size(mut self, percent: f32) -> Self {
        self.max_size = percent;
        self
    }
}

struct GroupState {
    sizes: Vec<f32>,
    bounds: Bounds<Pixels>,
}

#[derive(Clone, Copy)]
struct ResizeDrag {
    group: EntityId,
    handle: usize,
}

#[derive(IntoElement)]
pub struct ResizablePanelGroup {
    id: ElementId,
    orientation: Orientation,
    panels: Vec<ResizablePanel>,
    with_handle: bool,
    on_resize: Option<Rc<dyn Fn(&Vec<f32>, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(ResizablePanelGroup);

impl ResizablePanelGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: Orientation::Horizontal,
            panels: Vec::new(),
            with_handle: false,
            on_resize: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    pub fn panel(mut self, panel: ResizablePanel) -> Self {
        self.panels.push(panel);
        self
    }

    /// Show a grip on the handles (`<ResizableHandle withHandle />`).
    pub fn with_handle(mut self) -> Self {
        self.with_handle = true;
        self
    }

    /// Called with the panel sizes (percent) while dragging.
    pub fn on_resize(mut self, handler: impl Fn(&Vec<f32>, &mut Window, &mut App) + 'static) -> Self {
        self.on_resize = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ResizablePanelGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let count = self.panels.len();
        let defaults: Vec<f32> = {
            let given: f32 = self.panels.iter().filter_map(|p| p.default_size).sum();
            let unset = self.panels.iter().filter(|p| p.default_size.is_none()).count();
            let fill = if unset > 0 { ((100. - given) / unset as f32).max(0.) } else { 0. };
            self.panels.iter().map(|p| p.default_size.unwrap_or(fill)).collect()
        };
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| GroupState { sizes: defaults, bounds: Bounds::default() });
        if state.read(cx).sizes.len() != count {
            state.update(cx, |s, _| s.sizes.resize(count, 0.));
        }
        let sizes = state.read(cx).sizes.clone();
        let limits: Vec<(f32, f32)> = self.panels.iter().map(|p| (p.min_size, p.max_size)).collect();
        let theme = cx.theme();
        let vertical = self.orientation == Orientation::Vertical;
        let group_id = state.entity_id();

        let mut children = Vec::new();
        for (ix, panel) in self.panels.into_iter().enumerate() {
            children.push(
                div()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .map(|this| {
                        let size = relative(sizes.get(ix).copied().unwrap_or(0.) / 100.);
                        if vertical { this.w_full().h(size) } else { this.h_full().w(size) }
                    })
                    .child(panel.content)
                    .into_any_element(),
            );
            if ix + 1 < count {
                children.push(
                    div()
                        .id(("resize-handle", ix))
                        .relative()
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .bg(theme.border)
                        .map(|this| {
                            if vertical { this.h_px().w_full().cursor_row_resize() } else { this.w_px().h_full().cursor_col_resize() }
                        })
                        // Wider invisible hit area.
                        .child(div().absolute().map(|this| {
                            if vertical { this.w_full().h(px(8.)).top(px(-4.)) } else { this.h_full().w(px(8.)).left(px(-4.)) }
                        }))
                        .when(self.with_handle, |this| {
                            this.child(
                                div()
                                    .flex()
                                    .flex_none()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(2.))
                                    .border_1()
                                    .border_color(theme.border)
                                    .bg(theme.border)
                                    .map(|this| if vertical { this.h_3().w_4() } else { this.h_4().w_3() })
                                    .child(Icon::new(IconName::GripVertical).size_2p5()),
                            )
                        })
                        .on_drag(ResizeDrag { group: group_id, handle: ix }, |_, _, _, cx| cx.new(|_| InvisibleDrag))
                        .into_any_element(),
                );
            }
        }

        let drag_state = state.clone();
        let on_resize = self.on_resize;
        div()
            .id(self.id)
            .relative()
            .flex()
            .size_full()
            .when(vertical, |this| this.flex_col())
            .refine_style(&self.style)
            .children(children)
            .child({
                let state = state.clone();
                canvas(move |bounds, _, cx| state.update(cx, |s, _| s.bounds = bounds), |_, _, _, _| {})
                    .absolute()
                    .size_full()
            })
            .on_drag_move(move |event: &gpui::DragMoveEvent<ResizeDrag>, window, cx| {
                let drag = *event.drag(cx);
                if drag.group != group_id {
                    return;
                }
                let next = drag_state.update(cx, |s, cx| {
                    let bounds = s.bounds;
                    let (pos, origin, extent) = if vertical {
                        (event.event.position.y, bounds.top(), bounds.size.height)
                    } else {
                        (event.event.position.x, bounds.left(), bounds.size.width)
                    };
                    if extent <= px(0.) {
                        return None;
                    }
                    let pointer = ((pos - origin) / extent * 100.).clamp(0., 100.);
                    let ix = drag.handle;
                    let before: f32 = s.sizes[..ix].iter().sum();
                    let pair = s.sizes[ix] + s.sizes[ix + 1];
                    let (min_a, max_a) = limits[ix];
                    let (min_b, max_b) = limits[ix + 1];
                    let a = (pointer - before).clamp(min_a.max(pair - max_b), max_a.min(pair - min_b));
                    s.sizes[ix] = a;
                    s.sizes[ix + 1] = pair - a;
                    cx.notify();
                    Some(s.sizes.clone())
                });
                if let (Some(sizes), Some(handler)) = (next, &on_resize) {
                    handler(&sizes, window, cx);
                }
            })
    }
}

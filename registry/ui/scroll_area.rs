//! Augments native scroll functionality for custom, cross-browser styling:
//! a scroll container with a thin, draggable overlay scrollbar.
//!
//! ```ignore
//! ScrollArea::new("tags").h(px(288.)).w(px(192.)).child(tag_list)
//! ScrollArea::new("gallery").horizontal().child(images)
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, MouseButton, ParentElement, Pixels, Point,
    RenderOnce, ScrollHandle, StyleRefinement, Window, div, point, prelude::*, px,
};

use super::theme::ActiveTheme;
use super::utils::{InvisibleDrag, Orientation, StyledExt};

struct ScrollState {
    handle: ScrollHandle,
    /// Pointer position and scroll offset when a thumb drag started.
    drag_start: Option<(Point<Pixels>, Point<Pixels>)>,
}

struct ThumbDrag(gpui::EntityId);

#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    orientation: Orientation,
    handle: Option<ScrollHandle>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(ScrollArea);

impl ScrollArea {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), orientation: Orientation::Vertical, handle: None, children: Vec::new(), style: StyleRefinement::default() }
    }

    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }

    /// Use your own handle, e.g. to scroll programmatically.
    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.handle = Some(handle.clone());
        self
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let given = self.handle.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| ScrollState {
            handle: given.unwrap_or_default(),
            drag_start: None,
        });
        let handle = state.read(cx).handle.clone();
        let theme = cx.theme();
        let vertical = self.orientation == Orientation::Vertical;

        // Thumb geometry from the last layout.
        let viewport = handle.bounds().size;
        let max = handle.max_offset();
        let offset = handle.offset();
        let (view_len, max_len, off) = if vertical {
            (viewport.height, max.height, -offset.y)
        } else {
            (viewport.width, max.width, -offset.x)
        };
        let content_len = view_len + max_len;
        let show = max_len > px(0.5) && view_len > px(0.);
        let thumb_len = if show { (view_len * (view_len / content_len)).max(px(24.)) } else { px(0.) };
        let thumb_pos = if show { (view_len - thumb_len) * (off / max_len).clamp(0., 1.) } else { px(0.) };

        let entity_id = state.entity_id();
        let down_state = state.clone();
        let drag_state = state.clone();
        let content = div()
            .id("scroll-viewport")
            .size_full()
            .map(|this| if vertical { this.overflow_y_scroll() } else { this.overflow_x_scroll().flex() })
            .track_scroll(&handle)
            .on_scroll_wheel(|_, window, _| window.refresh())
            .children(self.children);

        div()
            .id(self.id)
            .relative()
            .overflow_hidden()
            .refine_style(&self.style)
            .child(content)
            .when(show, |this| {
                this.child(
                    div()
                        .absolute()
                        .p_px()
                        .map(|this| {
                            if vertical { this.top_0().right_0().h_full().w_2p5() } else { this.left_0().bottom_0().w_full().h_2p5() }
                        })
                        .child(
                            div()
                                .id("scroll-thumb")
                                .absolute()
                                .rounded_full()
                                .bg(theme.border)
                                .map(|this| {
                                    if vertical {
                                        this.top(thumb_pos).h(thumb_len).w(px(8.))
                                    } else {
                                        this.left(thumb_pos).w(thumb_len).h(px(8.))
                                    }
                                })
                                .on_mouse_down(MouseButton::Left, move |event, _, cx| {
                                    cx.stop_propagation();
                                    down_state.update(cx, |s, _| s.drag_start = Some((event.position, s.handle.offset())));
                                })
                                .on_drag(ThumbDrag(entity_id), |_, _, _, cx| cx.new(|_| InvisibleDrag)),
                        ),
                )
            })
            .on_drag_move(move |event: &gpui::DragMoveEvent<ThumbDrag>, window, cx| {
                if event.drag(cx).0 != entity_id {
                    return;
                }
                let s = drag_state.read(cx);
                let Some((start, start_offset)) = s.drag_start else { return };
                let handle = s.handle.clone();
                let delta = event.event.position - start;
                let track = (view_len - thumb_len).max(px(1.));
                let ratio = max_len / track;
                let next = if vertical {
                    point(start_offset.x, (start_offset.y - delta.y * ratio).clamp(-max_len, px(0.)))
                } else {
                    point((start_offset.x - delta.x * ratio).clamp(-max_len, px(0.)), start_offset.y)
                };
                handle.set_offset(next);
                window.refresh();
            })
    }
}

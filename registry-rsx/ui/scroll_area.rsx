//! Augments native scroll functionality for custom, cross-browser styling:
//! a scroll container with a thin, draggable overlay scrollbar.
//!
//! ```ignore
//! <ScrollArea class="h-[288px] w-[192px]">{tag_list}</ScrollArea>
//! <ScrollArea horizontal>{images}</ScrollArea>
//! ```
//!
//! The viewport is a native scroll container (`overflow-*-scroll`); the
//! thumb follows its `NodeRef` scroll offset and drags it with `scroll_to`.

use rsx::prelude::*;
use rsx::{MouseMoveEvent, Point, Size};

use super::theme::theme;
use super::utils::Orientation;

/// A `Copy` handle to a scroll area's position: read or set it, e.g. to
/// scroll programmatically. Pass it as `track_scroll={handle}`.
#[derive(Clone, Copy, PartialEq)]
pub struct ScrollHandle {
    viewport: NodeRef,
}

impl Default for ScrollHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollHandle {
    pub fn new() -> Self {
        ScrollHandle { viewport: NodeRef::new() }
    }

    /// The scrolling element, e.g. to `scroll_into_view` something in it.
    pub fn node(&self) -> NodeRef {
        self.viewport
    }

    /// The content's offset, like gpui's `ScrollHandle::offset`: zero at the
    /// top-left, negative as it scrolls (tracked).
    pub fn offset(&self) -> Point {
        let s = self.viewport.scroll_offset();
        Point { x: -s.x, y: -s.y }
    }

    /// Scrolls to `offset` (see [`offset`](Self::offset)), clamped to the
    /// content. Takes effect on the next frame.
    pub fn set_offset(&self, offset: Point) {
        let max = untrack(|| self.max_offset());
        self.viewport.scroll_to(Point { x: (-offset.x).clamp(0.0, max.width), y: (-offset.y).clamp(0.0, max.height) });
    }

    /// How far the content can scroll on each axis (tracked).
    pub fn max_offset(&self) -> Size<f32> {
        let max = self.viewport.max_scroll_offset();
        Size { width: max.x, height: max.y }
    }

    /// The viewport's size (tracked).
    pub fn viewport_size(&self) -> Size<f32> {
        self.viewport.bounds().map(|b| b.size).unwrap_or_default()
    }
}

#[component]
pub fn ScrollArea(
    /// Scroll sideways instead of up and down.
    horizontal: bool,
    /// `Orientation::Horizontal` is the same as `horizontal`.
    orientation: Option<Orientation>,
    /// Use your own handle, e.g. to scroll programmatically.
    track_scroll: Option<ScrollHandle>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let vertical = !horizontal && orientation != Some(Orientation::Horizontal);
    let handle = track_scroll.unwrap_or_default();
    let t = theme();
    // Pointer position and scroll position when a thumb drag started.
    let drag: Signal<Option<(f32, Point)>> = signal(None);

    // Thumb geometry from the last layout: `(length, position, track length, max)`.
    let thumb = move || {
        let view = handle.viewport_size();
        let max = handle.max_offset();
        let off = handle.viewport.scroll_offset();
        let (view_len, max_len, off) = if vertical { (view.height, max.height, off.y) } else { (view.width, max.width, off.x) };
        let content_len = view_len + max_len;
        if max_len <= 0.5 || view_len <= 0.0 {
            return None;
        }
        let len = (view_len * (view_len / content_len)).max(24.0);
        let pos = (view_len - len) * (off / max_len).clamp(0.0, 1.0);
        Some((len, pos, view_len, max_len))
    };

    let on_drag_move = move |e: &MouseMoveEvent| {
        let Some((start, start_scrolled)) = drag.get_untracked() else { return };
        if e.pressed_button.is_none() {
            drag.set(None);
            return;
        }
        let Some((len, _, view_len, max_len)) = untrack(thumb) else { return };
        let ratio = max_len / (view_len - len).max(1.0);
        let pointer = if vertical { e.position.y } else { e.position.x };
        let next = if vertical { start_scrolled.y } else { start_scrolled.x } + (pointer - start) * ratio;
        let s = untrack(|| handle.viewport.scroll_offset());
        let next = if vertical { Point { x: s.x, y: next } } else { Point { x: next, y: s.y } };
        handle.set_offset(Point { x: -next.x, y: -next.y });
    };

    <div class="relative overflow-hidden" {...attrs}>
        <div
            ref={handle.viewport}
            class="size-full"
            class:overflow-y-scroll={vertical}
            class:overflow-x-scroll={!vertical}
            class:flex={!vertical}
        >
            {children}
        </div>
        {move || thumb().map(|(len, pos, _, _)| {
            // `p-px`: keep the thumb inside the container's border and corners.
            let track = if vertical { "absolute p-px top-[4px] bottom-[4px] right-[1px] w-2.5" } else { "absolute p-px left-[4px] right-[4px] bottom-[1px] h-2.5" };
            <div class={track}>
                <div
                    class="absolute rounded-full"
                    bg={t.get().border}
                    top={px(if vertical { pos } else { 0.0 })}
                    left={px(if vertical { 0.0 } else { pos })}
                    w={px(if vertical { 8.0 } else { len })}
                    h={px(if vertical { len } else { 8.0 })}
                    onMouseDown={move |e: &MouseEvent| {
                        e.stop_propagation();
                        let pointer = if vertical { e.position.y } else { e.position.x };
                        drag.set(Some((pointer, untrack(|| handle.viewport.scroll_offset()))));
                    }}
                />
            </div>
        })}
        {/* While dragging the thumb, a transparent full-window layer follows the pointer:
            rsx's `onMouseMove` only fires over the element under the pointer. */}
        <Show when={drag.get().is_some()}>
            <Portal>
                <div
                    class="absolute inset-0"
                    onMouseMove={on_drag_move}
                    onMouseUp={move |_| drag.set(None)}
                />
            </Portal>
        </Show>
    </div>
}

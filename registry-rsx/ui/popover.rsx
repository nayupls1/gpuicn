//! Displays rich content in a portal, triggered by a button.
//!
//! This is also the floating-content primitive behind dropdown menus,
//! selects, hover cards, comboboxes and date pickers.
//!
//! ```ignore
//! <Popover
//!     trigger={<Button outline label="Open popover" />}
//!     content={|_| <div>Place content for the popover here.</div>}
//! />
//!
//! // Close from inside the content:
//! <Popover
//!     trigger={<Button label="Confirm" />}
//!     content={|popover: PopoverHandle| <Button label="OK" onClick={move |_| popover.close()} />}
//! />
//! ```

use std::time::Duration;

use rsx::prelude::*;
use rsx::{Bounds, Point, Size};

use super::theme::theme;
use super::utils::{pointer_pressed, Align, Side};

/// How a popover opens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverTrigger {
    /// Toggle on click (Popover, DropdownMenu, Select…).
    #[default]
    Click,
    /// Open while hovering trigger or content (HoverCard, Tooltip).
    Hover { open_delay_ms: u64, close_delay_ms: u64 },
    /// Never opens by itself; drive it with `open`.
    Manual,
}

/// A `Copy` handle to a popover's state: open it, close it, read where its
/// trigger is. Create one with [`PopoverHandle::new`] and pass it as
/// `handle={..}` to control a popover from outside; content render
/// functions receive it too.
#[derive(Clone, Copy)]
pub struct PopoverHandle {
    open: Signal<bool>,
    on_open_change: StoredValue<Option<Callback<bool>>>,
    trigger: NodeRef,
    content: NodeRef,
    viewport: NodeRef,
    hovered_trigger: Signal<bool>,
    hovered_content: Signal<bool>,
    hover_generation: Signal<u64>,
    /// Vertical room on the chosen side (Radix's `--radix-popper-available-height`).
    available_height: Signal<Option<f32>>,
}

impl Default for PopoverHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl PopoverHandle {
    pub fn new() -> Self {
        PopoverHandle {
            open: signal(false),
            on_open_change: StoredValue::new(None),
            trigger: NodeRef::new(),
            content: NodeRef::new(),
            viewport: NodeRef::new(),
            hovered_trigger: signal(false),
            hovered_content: signal(false),
            hover_generation: signal(0),
            available_height: signal(None),
        }
    }

    pub fn set_open(&self, open: bool) {
        if self.open.get_untracked() == open {
            return;
        }
        self.open.set(open);
        if let Some(f) = self.on_open_change.get_value() {
            f.call(&open);
        }
        if open {
            let content = self.content;
            // Take focus once the content exists, so Escape and arrow keys reach it.
            spawn_local(async move { content.focus() });
        } else {
            // Return focus to the trigger.
            let trigger = self.trigger;
            spawn_local(async move { trigger.focus() });
        }
    }

    pub fn open(&self) {
        self.set_open(true)
    }

    pub fn close(&self) {
        self.set_open(false)
    }

    pub fn toggle(&self) {
        self.set_open(!self.open.get_untracked())
    }

    /// Whether the popover is open (tracked).
    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    /// Whether the popover is open, without subscribing.
    pub fn is_open_untracked(&self) -> bool {
        self.open.get_untracked()
    }

    /// Vertical room for the content on the side it opened on (tracked).
    pub fn available_height(&self) -> Option<f32> {
        self.available_height.get()
    }

    /// Where the trigger is, in window coordinates (tracked).
    pub fn trigger_bounds(&self) -> Bounds {
        self.trigger.bounds().unwrap_or_default()
    }

    /// The content panel, e.g. to focus it.
    pub fn content_ref(&self) -> NodeRef {
        self.content
    }

    /// The trigger wrapper.
    pub fn trigger_ref(&self) -> NodeRef {
        self.trigger
    }

    fn schedule_hover(self, delays: (u64, u64)) {
        let hovered = self.hovered_trigger.get_untracked() || self.hovered_content.get_untracked();
        let generation = self.hover_generation.get_untracked() + 1;
        self.hover_generation.set(generation);
        if hovered == self.open.get_untracked() {
            return;
        }
        let delay = if hovered { delays.0 } else { delays.1 };
        spawn_local(async move {
            if delay > 0 {
                sleep(Duration::from_millis(delay)).await;
            }
            if self.hover_generation.get_untracked() == generation {
                let hovered = self.hovered_trigger.get_untracked() || self.hovered_content.get_untracked();
                self.set_open(hovered);
            }
        });
    }
}

/// Which corner of the content sits at a floating position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Where the floating content goes, and which of its corners sits there.
pub fn floating_position(trigger: Bounds, content: Size<f32>, side: Side, align: Align, offset: f32) -> (Point, Corner) {
    let (left, top) = (trigger.origin.x, trigger.origin.y);
    let (right, bottom) = (left + trigger.size.width, top + trigger.size.height);
    match side {
        Side::Bottom | Side::Top => {
            let y = if side == Side::Bottom { bottom + offset } else { top - offset };
            let (x, end) = match align {
                Align::Start => (left, false),
                Align::End => (right, true),
                Align::Center => ((left + right) / 2.0 - content.width / 2.0, false),
            };
            let corner = match (side == Side::Bottom, end) {
                (true, false) => Corner::TopLeft,
                (true, true) => Corner::TopRight,
                (false, false) => Corner::BottomLeft,
                (false, true) => Corner::BottomRight,
            };
            (Point { x, y }, corner)
        }
        Side::Left | Side::Right => {
            let x = if side == Side::Right { right + offset } else { left - offset };
            let (y, end) = match align {
                Align::Start => (top, false),
                Align::End => (bottom, true),
                Align::Center => ((top + bottom) / 2.0 - content.height / 2.0, false),
            };
            let corner = match (side == Side::Right, end) {
                (true, false) => Corner::TopLeft,
                (true, true) => Corner::BottomLeft,
                (false, false) => Corner::TopRight,
                (false, true) => Corner::BottomRight,
            };
            (Point { x, y }, corner)
        }
    }
}

/// Radix's `avoidCollisions`: flip to the opposite side when the content
/// doesn't fit on the preferred one but does on the other. When it fits on
/// neither, take the roomier side. Returns the side and the vertical room on
/// it, which caps the content's height so it never slides over its trigger.
pub fn avoid_collisions(side: Side, trigger: Bounds, content: Size<f32>, offset: f32, viewport: Size<f32>) -> (Side, f32) {
    let margin = 8.0;
    let (left, top) = (trigger.origin.x, trigger.origin.y);
    let (right, bottom) = (left + trigger.size.width, top + trigger.size.height);
    let room = |side: Side| match side {
        Side::Bottom => viewport.height - bottom - offset - margin,
        Side::Top => top - offset - margin,
        Side::Right => viewport.width - right - offset - margin,
        Side::Left => left - offset - margin,
    };
    let height_room = |side: Side| match side {
        Side::Top | Side::Bottom => room(side),
        Side::Left | Side::Right => viewport.height - margin * 2.0,
    };
    if content.width <= 0.0 || content.height <= 0.0 {
        return (side, height_room(side));
    }
    let need = match side {
        Side::Top | Side::Bottom => content.height,
        Side::Left | Side::Right => content.width,
    };
    let opposite = match side {
        Side::Bottom => Side::Top,
        Side::Top => Side::Bottom,
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    };
    // Content measured at a previous cap on this side hides its natural
    // height; treat it as not fitting so the roomier side can win.
    let vertical = matches!(side, Side::Top | Side::Bottom);
    let capped = vertical && content.height + 1.0 >= room(side);
    let chosen = if room(side) >= need && !capped {
        side
    } else if room(opposite) >= need || room(opposite) > room(side) {
        opposite
    } else {
        side
    };
    (chosen, height_room(chosen).max(0.0))
}

/// The top-left of the content, kept 8px inside the window
/// (gpui's `anchored().snap_to_window_with_margin(8)`).
fn place(position: Point, corner: Corner, content: Size<f32>, viewport: Size<f32>) -> Point {
    let mut x = match corner {
        Corner::TopLeft | Corner::BottomLeft => position.x,
        Corner::TopRight | Corner::BottomRight => position.x - content.width,
    };
    let mut y = match corner {
        Corner::TopLeft | Corner::TopRight => position.y,
        Corner::BottomLeft | Corner::BottomRight => position.y - content.height,
    };
    if viewport.width > 0.0 {
        x = x.min(viewport.width - 8.0 - content.width).max(8.0);
        y = y.min(viewport.height - 8.0 - content.height).max(8.0);
    }
    Point { x, y }
}

#[component]
pub fn Popover(
    /// The element that opens the popover.
    trigger: Option<Children>,
    /// The floating content; receives the popover's handle.
    content: Render<PopoverHandle>,
    /// Control the open state from outside.
    open: Option<Accessor<bool>>,
    on_open_change: Option<Callback<bool>>,
    /// Share state with the caller: `handle={popover}` with `let popover = PopoverHandle::new();`.
    handle: Option<PopoverHandle>,
    #[prop(optional)] mode: PopoverTrigger,
    #[prop(optional)] side: Side,
    #[prop(optional)] align: Align,
    #[prop(default = 4.0)] side_offset: f32,
    /// Open at a point (in window coordinates) instead of next to the trigger (context menus).
    anchor_position: Option<Accessor<Option<Point>>>,
    /// Pointer moves over the click-away backdrop of an open popover, in
    /// window coordinates (a menubar switches menus as the pointer crosses its triggers).
    on_backdrop_move: Option<Callback<Point>>,
    /// Don't draw the default panel (background, border, padding, width).
    unstyled: bool,
    /// Classes for the wrapper around the trigger (`w-full` to stretch it).
    wrapper_class: Option<String>,
    /// Styles for the content panel, layered over the defaults.
    attrs: Attributes,
) -> impl IntoElement {
    let handle = handle.unwrap_or_default();
    handle.on_open_change.set_value(on_open_change);
    if let Some(open) = open {
        effect(move || {
            let open = open.get();
            if handle.open.get_untracked() != open {
                handle.open.set(open);
            }
        });
    }
    let appearance = !unstyled;
    let t = theme();
    let content_size = move || handle.content.bounds().map(|b| b.size).unwrap_or(Size { width: 0.0, height: 0.0 });
    let viewport = move || handle.viewport.bounds().map(|b| b.size).unwrap_or(Size { width: 0.0, height: 0.0 });
    let hover_delays = match mode {
        PopoverTrigger::Hover { open_delay_ms, close_delay_ms } => Some((open_delay_ms, close_delay_ms)),
        _ => None,
    };
    let attrs = StoredValue::new(attrs);

    // Built once per opening; only its position and size follow the signals.
    let layer = move || {
        let place_panel = move |el: rsx::Div| {
            let theme = t.get();
            let size = content_size();
            let vp = viewport();
            let trigger_bounds = handle.trigger_bounds();
            let (position, corner, max_height) = match anchor_position.and_then(|a| a.get()) {
                Some(p) => (p, Corner::TopLeft, None),
                None => {
                    let (side, room) = avoid_collisions(side, trigger_bounds, size, side_offset, vp);
                    let (p, c) = floating_position(trigger_bounds, size, side, align, side_offset);
                    (p, c, Some(room))
                }
            };
            handle.available_height.set(max_height);
            let at = place(position, corner, size, vp);
            // Scroll only once the content has hit the cap, so small popovers
            // (tooltips with arrows) are never clipped.
            let scroll = appearance && max_height.is_some_and(|max| size.height > 0.0 && size.height + 1.0 >= max);
            // Hidden for the frame where we don't know our size (or the window's) yet.
            let measured = size.width > 0.0 && vp.width > 0.0;
            let mut el = el.left(px(at.x)).top(px(at.y)).opacity(if measured { 1.0 } else { 0.0 });
            if let Some(max) = max_height.filter(|_| appearance) {
                el = el.max_h(px(max));
            }
            if scroll {
                el = el.class("overflow-y-scroll");
            }
            if appearance {
                el = el
                    .class("w-72 p-4 border")
                    .corner_radius(theme.radius_md())
                    .border_color(theme.border)
                    .bg(theme.popover)
                    .text_color(theme.popover_foreground);
            }
            el.spread(attrs.with_value(|a| a.clone()))
        };

        <div
            ref={handle.content}
            class="absolute flex flex-col"
            bind={place_panel}
            onKeyDown={move |e: &KeyboardEvent| {
                if e.key == "Escape" {
                    e.stop_propagation();
                    handle.close();
                }
            }}
            onHover={move |h: &bool| {
                if let Some(delays) = hover_delays {
                    handle.hovered_content.set(*h);
                    handle.schedule_hover(delays);
                }
            }}
            {...attrs.with_value(|a| a.clone())}
        >
            {content.render(handle)}
        </div>
    };

    let backdrop = move || match hover_delays {
        // Hover popovers only need the window measured.
        Some(_) => <div class="absolute inset-0" ref={handle.viewport} />,
        // Click-away backdrop: closes the popover and swallows the click, so
        // clicking the trigger again doesn't immediately reopen it.
        None => <div
            class="absolute inset-0"
            ref={handle.viewport}
            onMouseDown={move |e: &MouseEvent| {
                e.stop_propagation();
                handle.close();
            }}
            onMouseMove={move |e: &rsx::MouseMoveEvent| {
                if let Some(f) = on_backdrop_move {
                    f.call(&e.position)
                }
            }}
        />,
    };

    let fill = wrapper_class.as_deref().is_some_and(|c| c.contains("w-"));
    <div class={format!("flex {}", wrapper_class.unwrap_or_default())}>
        <div
            ref={handle.trigger}
            class="relative flex"
            class:w-full={fill}
            onClick={move |_| if mode == PopoverTrigger::Click { pointer_pressed(); handle.toggle() }}
            onKeyDown={move |e: &KeyboardEvent| {
                // Keyboard: Enter / Space / ↓ on a focused trigger (key events
                // bubble up from the focused button inside).
                let modified = e.modifiers.control || e.modifiers.alt || e.modifiers.platform;
                if mode == PopoverTrigger::Click
                    && matches!(e.key.as_str(), "Enter" | " " | "ArrowDown")
                    && !modified
                    && !handle.open.get_untracked()
                {
                    e.stop_propagation();
                    e.prevent_default();
                    handle.open();
                }
            }}
            onHover={move |h: &bool| {
                if let Some(delays) = hover_delays {
                    handle.hovered_trigger.set(*h);
                    handle.schedule_hover(delays);
                }
            }}
        >
            {trigger}
        </div>
        <Show when={handle.open.get()}>
            <Portal>
                {backdrop}
                {layer()}
            </Portal>
        </Show>
    </div>
}

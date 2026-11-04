//! Displays rich content in a portal, triggered by a button.
//!
//! This is also the floating-content primitive behind dropdown menus,
//! selects, hover cards, comboboxes and date pickers.
//!
//! ```ignore
//! Popover::new("dimensions")
//!     .trigger(Button::new("open").outline().label("Open popover"))
//!     .content(|_, _| div().child("Place content for the popover here."))
//!
//! // Close from inside the content:
//! Popover::new("confirm")
//!     .trigger(Button::new("t").label("Confirm"))
//!     .content_with_handle(|popover, _, _| {
//!         let popover = popover.clone();
//!         Button::new("ok").label("OK").on_click(move |_, window, cx| popover.close(window, cx))
//!     })
//! ```

use gpui::{
    AnyElement, App, Bounds, Context, Corner, ElementId, Entity, FocusHandle, IntoElement,
    ParentElement, Pixels, Point, RenderOnce, Size, StyleRefinement, Task, Window, anchored,
    canvas, div, point, prelude::*, px,
};
use std::rc::Rc;
use std::time::Duration;

use super::portal::{full_window, portal};
use super::theme::ActiveTheme;
use super::utils::{Align, Side, StyledExt};

/// How a popover opens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverTrigger {
    /// Toggle on click (Popover, DropdownMenu, Select…).
    #[default]
    Click,
    /// Open while hovering trigger or content (HoverCard, Tooltip).
    Hover { open_delay_ms: u64, close_delay_ms: u64 },
    /// Never opens by itself; drive it with `.open(..)`.
    Manual,
}

/// Per-popover state, kept alive across frames by `window.use_keyed_state`.
pub struct PopoverState {
    open: bool,
    pub trigger_bounds: Bounds<Pixels>,
    pub content_size: Size<Pixels>,
    pub focus_handle: FocusHandle,
    needs_focus: bool,
    hovered_trigger: bool,
    hovered_content: bool,
    hover_task: Option<Task<()>>,
    /// What had focus before the popover took it; focused again on close.
    return_focus: Option<FocusHandle>,
    /// Vertical room on the chosen side (Radix's `--radix-popper-available-height`).
    available_height: Option<Pixels>,
}

impl PopoverState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            open: false,
            trigger_bounds: Bounds::default(),
            content_size: Size::default(),
            focus_handle: cx.focus_handle(),
            needs_focus: false,
            hovered_trigger: false,
            hovered_content: false,
            hover_task: None,
            return_focus: None,
            available_height: None,
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

/// A cheap, clonable handle for opening/closing a popover from callbacks.
#[derive(Clone)]
pub struct PopoverHandle {
    state: Entity<PopoverState>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
}

impl PopoverHandle {
    pub fn set_open(&self, open: bool, window: &mut Window, cx: &mut App) {
        let (changed, return_focus) = self.state.update(cx, |state, cx| {
            let changed = state.open != open;
            state.open = open;
            state.needs_focus = open;
            let mut return_focus = None;
            if !open {
                state.content_size = Size::default();
                return_focus = state.return_focus.take();
            }
            cx.notify();
            (changed, return_focus)
        });
        // Like Radix: closing hands focus back to the trigger, but only if
        // focus is still inside the popover (not moved elsewhere by a click).
        if let Some(handle) = return_focus
            && changed
            && self.state.read(cx).focus_handle.contains_focused(window, cx)
        {
            window.focus(&handle);
        }
        if changed && let Some(handler) = &self.on_open_change {
            handler(&open, window, cx);
        }
    }

    pub fn open(&self, window: &mut Window, cx: &mut App) {
        self.set_open(true, window, cx);
    }

    pub fn close(&self, window: &mut Window, cx: &mut App) {
        self.set_open(false, window, cx);
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut App) {
        let open = self.state.read(cx).open;
        self.set_open(!open, window, cx);
    }

    pub fn is_open(&self, cx: &App) -> bool {
        self.state.read(cx).open
    }

    /// Vertical room for the content on the side it opened on. Unstyled
    /// content should cap its height to this and scroll, like shadcn's
    /// `max-h-(--radix-…-content-available-height) overflow-y-auto`.
    pub fn available_height(&self, cx: &App) -> Option<Pixels> {
        self.state.read(cx).available_height
    }

    /// Window-space bounds of the trigger, as of the last frame.
    pub fn trigger_bounds(&self, cx: &App) -> Bounds<Pixels> {
        self.state.read(cx).trigger_bounds
    }

    pub fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.read(cx).focus_handle.clone()
    }

    pub fn entity(&self) -> &Entity<PopoverState> {
        &self.state
    }
}

type ContentBuilder = Rc<dyn Fn(&PopoverHandle, &mut Window, &mut App) -> AnyElement>;

#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<ContentBuilder>,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    mode: PopoverTrigger,
    side: Side,
    align: Align,
    side_offset: Pixels,
    anchor_position: Option<Point<Pixels>>,
    appearance: bool,
    wrapper_style: StyleRefinement,
    on_backdrop_move: Option<BackdropMoveHandler>,
    style: StyleRefinement,
}

type BackdropMoveHandler = Rc<dyn Fn(Point<Pixels>, &mut Window, &mut App)>;

crate::impl_styled!(Popover);

/// The outer-layout part of `style` (width and flex sizing). Components that
/// wrap their trigger in a [`Popover`] forward it with
/// [`Popover::wrapper_style`] so that e.g. `Select::new(..).w_full()` sizes
/// the whole control, not just the trigger inside a content-sized wrapper.
pub fn outer_layout(style: &StyleRefinement) -> StyleRefinement {
    let mut out = StyleRefinement::default();
    out.size.width = style.size.width;
    out.min_size.width = style.min_size.width;
    out.max_size.width = style.max_size.width;
    out.flex_grow = style.flex_grow;
    out.flex_shrink = style.flex_shrink;
    out.flex_basis = style.flex_basis;
    out.align_self = style.align_self;
    out
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: None,
            on_open_change: None,
            mode: PopoverTrigger::Click,
            side: Side::Bottom,
            align: Align::Center,
            side_offset: px(4.),
            anchor_position: None,
            appearance: true,
            wrapper_style: StyleRefinement::default(),
            on_backdrop_move: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content<E: IntoElement>(mut self, content: impl Fn(&mut Window, &mut App) -> E + 'static) -> Self {
        self.content = Some(Rc::new(move |_, window, cx| content(window, cx).into_any_element()));
        self
    }

    /// Like [`Popover::content`], but the builder receives a [`PopoverHandle`]
    /// that can close the popover.
    pub fn content_with_handle<E: IntoElement>(
        mut self,
        content: impl Fn(&PopoverHandle, &mut Window, &mut App) -> E + 'static,
    ) -> Self {
        self.content = Some(Rc::new(move |handle, window, cx| content(handle, window, cx).into_any_element()));
        self
    }

    /// Control the open state yourself (pair with [`Popover::on_open_change`]).
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    pub fn mode(mut self, mode: PopoverTrigger) -> Self {
        self.mode = mode;
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn side_offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.side_offset = offset.into();
        self
    }

    /// Open at a fixed window position instead of next to the trigger
    /// (used by context menus).
    pub fn anchor_position(mut self, position: Point<Pixels>) -> Self {
        self.anchor_position = Some(position);
        self
    }

    /// Style for the element wrapping the trigger (see [`outer_layout`]).
    /// When it sets a width, the trigger stretches to fill it.
    pub fn wrapper_style(mut self, style: StyleRefinement) -> Self {
        self.wrapper_style = style;
        self
    }

    /// Called when the pointer moves over the click-away backdrop of an open
    /// (click/manual) popover. The backdrop blocks hover on everything below
    /// it, so this is how e.g. a menubar switches menus as the pointer moves
    /// across its triggers.
    pub fn on_backdrop_move(mut self, handler: impl Fn(Point<Pixels>, &mut Window, &mut App) + 'static) -> Self {
        self.on_backdrop_move = Some(Rc::new(handler));
        self
    }

    /// Don't draw the default panel (background, border, padding, width).
    pub fn unstyled(mut self) -> Self {
        self.appearance = false;
        self
    }
}

/// Where the floating content goes, and which of its corners sits there.
pub fn floating_position(
    trigger: Bounds<Pixels>,
    content: Size<Pixels>,
    side: Side,
    align: Align,
    offset: Pixels,
) -> (Point<Pixels>, Corner) {
    match side {
        Side::Bottom | Side::Top => {
            let y = if side == Side::Bottom { trigger.bottom() + offset } else { trigger.top() - offset };
            let (x, right) = match align {
                Align::Start => (trigger.left(), false),
                Align::End => (trigger.right(), true),
                Align::Center => (trigger.center().x - content.width / 2., false),
            };
            let corner = match (side == Side::Bottom, right) {
                (true, false) => Corner::TopLeft,
                (true, true) => Corner::TopRight,
                (false, false) => Corner::BottomLeft,
                (false, true) => Corner::BottomRight,
            };
            (point(x, y), corner)
        }
        Side::Left | Side::Right => {
            let x = if side == Side::Right { trigger.right() + offset } else { trigger.left() - offset };
            let (y, bottom) = match align {
                Align::Start => (trigger.top(), false),
                Align::End => (trigger.bottom(), true),
                Align::Center => (trigger.center().y - content.height / 2., false),
            };
            let corner = match (side == Side::Right, bottom) {
                (true, false) => Corner::TopLeft,
                (true, true) => Corner::BottomLeft,
                (false, false) => Corner::TopRight,
                (false, true) => Corner::BottomRight,
            };
            (point(x, y), corner)
        }
    }
}

/// Radix's `avoidCollisions`: flip to the opposite side when the content
/// doesn't fit on the preferred one but does on the other. When it fits on
/// neither, take the roomier side. Returns the side and the room on it, which
/// caps the content's height (Radix's `--radix-popper-available-height`) so
/// window snapping never slides the content over its trigger.
fn avoid_collisions(side: Side, trigger: Bounds<Pixels>, content: Size<Pixels>, offset: Pixels, window: &Window) -> (Side, Pixels) {
    let viewport = window.viewport_size();
    let margin = px(8.);
    let room = |side: Side| match side {
        Side::Bottom => viewport.height - trigger.bottom() - offset - margin,
        Side::Top => trigger.top() - offset - margin,
        Side::Right => viewport.width - trigger.right() - offset - margin,
        Side::Left => trigger.left() - offset - margin,
    };
    // Vertical room for the content when placed on `side`.
    let height_room = |side: Side| match side {
        Side::Top | Side::Bottom => room(side),
        Side::Left | Side::Right => viewport.height - margin * 2.,
    };
    if content.width <= px(0.) || content.height <= px(0.) {
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
    let capped = vertical && content.height + px(1.) >= room(side);
    let chosen = if room(side) >= need && !capped {
        side
    } else if room(opposite) >= need || room(opposite) > room(side) {
        opposite
    } else {
        side
    };
    (chosen, height_room(chosen).max(px(0.)))
}

fn schedule_hover(state: &Entity<PopoverState>, handle: PopoverHandle, delays: (u64, u64), window: &mut Window, cx: &mut App) {
    let (hovered, open) = {
        let s = state.read(cx);
        (s.hovered_trigger || s.hovered_content, s.open)
    };
    if hovered == open {
        state.update(cx, |s, _| s.hover_task = None);
        return;
    }
    let delay = if hovered { delays.0 } else { delays.1 };
    let task = window.spawn(cx, async move |cx| {
        cx.background_executor().timer(Duration::from_millis(delay)).await;
        cx.update(|window, cx| {
            let s = handle.state.read(cx);
            let hovered = s.hovered_trigger || s.hovered_content;
            handle.set_open(hovered, window, cx);
        })
        .ok();
    });
    state.update(cx, |s, _| s.hover_task = Some(task));
}

impl RenderOnce for Popover {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| PopoverState::new(cx));
        let handle = PopoverHandle { state: state.clone(), on_open_change: self.on_open_change.clone() };

        if let Some(open) = self.open {
            state.update(cx, |s, _| {
                if s.open != open {
                    s.open = open;
                    s.needs_focus = open;
                }
            });
        }

        let (open, trigger_bounds, content_size, focus_handle, needs_focus) = {
            let s = state.read(cx);
            (s.open, s.trigger_bounds, s.content_size, s.focus_handle.clone(), s.needs_focus)
        };
        if open && needs_focus && !matches!(self.mode, PopoverTrigger::Hover { .. }) {
            let previous = window.focused(cx);
            window.focus(&focus_handle);
            state.update(cx, |s, _| {
                s.needs_focus = false;
                s.return_focus = previous.filter(|p| *p != s.focus_handle);
            });
        }

        let mode = self.mode;

        let trigger = div()
            .id("popover-trigger")
            .relative()
            .flex()
            .children(self.trigger)
            .child({
                let state = state.clone();
                canvas(
                    move |bounds, window, cx| {
                        state.update(cx, |s, _| {
                            if s.trigger_bounds != bounds {
                                s.trigger_bounds = bounds;
                                // An open popover follows its trigger (scrolling,
                                // resizing, first frame) on the next frame.
                                if s.open {
                                    window.request_animation_frame();
                                }
                            }
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .map(|this| match mode {
                PopoverTrigger::Click => {
                    let (click, keys) = (handle.clone(), handle.clone());
                    this.on_click(move |_, window, cx| click.toggle(window, cx))
                        // Keyboard: Enter / Space / ↓ on a focused trigger (key
                        // events bubble up from the focused button inside).
                        .on_key_down(move |event, window, cx| {
                            if matches!(event.keystroke.key.as_str(), "enter" | "space" | "down")
                                && !event.keystroke.modifiers.modified()
                                && !keys.is_open(cx)
                            {
                                cx.stop_propagation();
                                window.prevent_default();
                                keys.open(window, cx);
                            }
                        })
                }
                PopoverTrigger::Hover { open_delay_ms, close_delay_ms } => {
                    let handle = handle.clone();
                    let state = state.clone();
                    this.on_hover(move |hovered, window, cx| {
                        state.update(cx, |s, _| s.hovered_trigger = *hovered);
                        schedule_hover(&state, handle.clone(), (open_delay_ms, close_delay_ms), window, cx);
                    })
                }
                PopoverTrigger::Manual => this,
            });

        if open && let Some(content) = self.content.clone() {
            let (position, corner, max_height) = match self.anchor_position {
                Some(position) => (position, Corner::TopLeft, None),
                None => {
                    let (side, room) = avoid_collisions(self.side, trigger_bounds, content_size, self.side_offset, window);
                    let (position, corner) = floating_position(trigger_bounds, content_size, side, self.align, self.side_offset);
                    (position, corner, Some(room))
                }
            };
            state.update(cx, |s, _| s.available_height = max_height);
            let layer = FloatingLayer {
                content,
                handle: handle.clone(),
                focus_handle,
                position,
                corner,
                // Unstyled content caps itself via `PopoverHandle::available_height`.
                max_height: max_height.filter(|_| self.appearance),
                // Scroll only once the content has hit the cap, so small
                // popovers (tooltips with arrows) are never clipped.
                scroll: self.appearance && max_height.is_some_and(|max| content_size.height > px(0.) && content_size.height + px(1.) >= max),
                hide_until_measured: content_size.width <= px(0.) && self.align == Align::Center,
                mode,
                appearance: self.appearance,
                on_backdrop_move: self.on_backdrop_move.clone(),
                style: self.style,
            };
            portal(
                ElementId::NamedInteger("popover".into(), state.entity_id().as_u64()),
                window,
                cx,
                move |window, cx| layer.build(window, cx),
            );
        }

        let fill = self.wrapper_style.size.width.is_some();
        div().id(self.id).flex().refine_style(&self.wrapper_style).child(trigger.when(fill, |this| this.w_full()))
    }
}

/// Everything needed to draw an open popover's content inside the portal.
struct FloatingLayer {
    content: ContentBuilder,
    handle: PopoverHandle,
    focus_handle: FocusHandle,
    position: Point<Pixels>,
    corner: Corner,
    max_height: Option<Pixels>,
    scroll: bool,
    hide_until_measured: bool,
    mode: PopoverTrigger,
    appearance: bool,
    on_backdrop_move: Option<BackdropMoveHandler>,
    style: StyleRefinement,
}

impl FloatingLayer {
    fn build(self, window: &mut Window, cx: &mut App) -> AnyElement {
        let handle = self.handle;
        let state = handle.state.clone();
        let content = (self.content)(&handle, window, cx);
        let theme = cx.theme();

        let panel = div()
            .id("popover-content")
            .track_focus(&self.focus_handle)
            .occlude()
            .relative()
            .when(self.appearance, |this| {
                this.w_72()
                    .p_4()
                    .rounded(theme.radius_md())
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.popover)
                    .text_color(theme.popover_foreground)
                    .when(theme.shadows, |this| this.shadow_md())
            })
            .refine_style(&self.style)
            .when_some(self.max_height, |this, max| this.max_h(max))
            .when(self.scroll, |this| this.overflow_y_scroll())
            // Hidden for the single frame where we don't know our size yet.
            .when(self.hide_until_measured, |this| this.opacity(0.))
            .child(content)
            .child({
                let state = state.clone();
                canvas(
                    move |bounds, window, cx| {
                        state.update(cx, |s, _| {
                            if s.content_size != bounds.size {
                                s.content_size = bounds.size;
                                // `cx.notify()` during prepaint is dropped by
                                // gpui; re-render on the next frame instead so
                                // the popover gets positioned and shown.
                                window.request_animation_frame();
                            }
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full()
            })
            .on_key_down({
                let handle = handle.clone();
                move |event, window, cx| {
                    if event.keystroke.key == "escape" {
                        cx.stop_propagation();
                        handle.close(window, cx);
                    }
                }
            })
            .when_some(
                match self.mode {
                    PopoverTrigger::Hover { open_delay_ms, close_delay_ms } => Some((open_delay_ms, close_delay_ms)),
                    _ => None,
                },
                |this, delays| {
                    let handle = handle.clone();
                    let state = state.clone();
                    this.on_hover(move |hovered, window, cx| {
                        state.update(cx, |s, _| s.hovered_content = *hovered);
                        schedule_hover(&state, handle.clone(), delays, window, cx);
                    })
                },
            );

        let floating = anchored()
            .position(self.position)
            .anchor(self.corner)
            .snap_to_window_with_margin(px(8.))
            .child(panel);

        match self.mode {
            PopoverTrigger::Hover { .. } => floating.into_any_element(),
            // Click-away backdrop: closes the popover and swallows the click,
            // so clicking the trigger again doesn't immediately reopen it.
            _ => div()
                .child(full_window(
                    div()
                        .occlude()
                        .on_any_mouse_down(move |_, window, cx| {
                            cx.stop_propagation();
                            handle.close(window, cx);
                        })
                        .when_some(self.on_backdrop_move, |this, handler| {
                            this.on_mouse_move(move |event, window, cx| handler(event.position, window, cx))
                        }),
                    window,
                ))
                .child(floating)
                .into_any_element(),
        }
    }
}

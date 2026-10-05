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
use super::utils::{Align, Side, StyledExt, on_mouse_leave_window};

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
    /// Clicked while hovering: stay closed until the pointer leaves.
    hover_suppressed: bool,
    hover_task: Option<Task<()>>,
    /// Bumped on every open so the content gets fresh hover tracking.
    generation: usize,
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
            hover_suppressed: false,
            hover_task: None,
            generation: 0,
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
        let changed = self.state.update(cx, |state, cx| {
            let changed = state.open != open;
            state.open = open;
            state.needs_focus = open;
            if open && changed {
                state.generation += 1;
            }
            if !open {
                state.content_size = Size::default();
                // The content is going away without telling us it was left.
                state.hovered_content = false;
            }
            cx.notify();
            changed
        });
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
    modal: bool,
    style: StyleRefinement,
}

crate::impl_styled!(Popover);

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
            modal: true,
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

    /// By default (click and manual modes) the rest of the window is inert
    /// while open: clicking outside only closes the popover and nothing
    /// underneath reacts to hover. With `modal(false)` the outside stays
    /// hoverable (clicks still just close), e.g. for menubars that switch
    /// menus on hover.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
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

fn schedule_hover(state: &Entity<PopoverState>, handle: PopoverHandle, delays: (u64, u64), window: &mut Window, cx: &mut App) {
    let (hovered, open) = {
        let s = state.read(cx);
        (!s.hover_suppressed && (s.hovered_trigger || s.hovered_content), s.open)
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
            let hovered = !s.hover_suppressed && (s.hovered_trigger || s.hovered_content);
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

        let (open, trigger_bounds, content_size, focus_handle, needs_focus, generation) = {
            let s = state.read(cx);
            (s.open, s.trigger_bounds, s.content_size, s.focus_handle.clone(), s.needs_focus, s.generation)
        };
        if open && needs_focus && !matches!(self.mode, PopoverTrigger::Hover { .. }) {
            window.focus(&focus_handle);
            state.update(cx, |s, _| s.needs_focus = false);
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
                    move |bounds, _, cx| state.update(cx, |s, _| s.trigger_bounds = bounds),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .map(|this| match mode {
                PopoverTrigger::Click => {
                    let handle = handle.clone();
                    this.on_click(move |_, window, cx| handle.toggle(window, cx))
                }
                PopoverTrigger::Hover { open_delay_ms, close_delay_ms } => {
                    let delays = (open_delay_ms, close_delay_ms);
                    let (hover_handle, hover_state) = (handle.clone(), state.clone());
                    let (down_handle, down_state) = (handle.clone(), state.clone());
                    let (leave_handle, leave_state) = (handle.clone(), state.clone());
                    this.on_hover(move |hovered, window, cx| {
                        hover_state.update(cx, |s, _| {
                            s.hovered_trigger = *hovered;
                            if !*hovered {
                                s.hover_suppressed = false;
                            }
                        });
                        schedule_hover(&hover_state, hover_handle.clone(), delays, window, cx);
                    })
                    // Like Radix tooltips: clicking the trigger dismisses.
                    .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx| {
                        down_state.update(cx, |s, _| s.hover_suppressed = true);
                        schedule_hover(&down_state, down_handle.clone(), delays, window, cx);
                        down_handle.close(window, cx);
                    })
                    .child(on_mouse_leave_window(move |window, cx| {
                        leave_state.update(cx, |s, _| {
                            s.hovered_trigger = false;
                            s.hovered_content = false;
                            s.hover_task = None;
                        });
                        leave_handle.close(window, cx);
                    }))
                }
                PopoverTrigger::Manual => this,
            });

        if open && let Some(content) = self.content.clone() {
            let (position, corner) = match self.anchor_position {
                Some(position) => (position, Corner::TopLeft),
                None => floating_position(trigger_bounds, content_size, self.side, self.align, self.side_offset),
            };
            let layer = FloatingLayer {
                content,
                handle: handle.clone(),
                focus_handle,
                position,
                corner,
                hide_until_measured: content_size.width <= px(0.) && self.align == Align::Center,
                mode,
                appearance: self.appearance,
                modal: self.modal,
                generation,
                style: self.style,
            };
            portal(
                ElementId::NamedInteger("popover".into(), state.entity_id().as_u64()),
                window,
                cx,
                move |window, cx| layer.build(window, cx),
            );
        }

        div().id(self.id).flex().child(trigger)
    }
}

/// Everything needed to draw an open popover's content inside the portal.
struct FloatingLayer {
    content: ContentBuilder,
    handle: PopoverHandle,
    focus_handle: FocusHandle,
    position: Point<Pixels>,
    corner: Corner,
    hide_until_measured: bool,
    mode: PopoverTrigger,
    appearance: bool,
    modal: bool,
    generation: usize,
    style: StyleRefinement,
}

impl FloatingLayer {
    fn build(self, window: &mut Window, cx: &mut App) -> AnyElement {
        let handle = self.handle;
        let state = handle.state.clone();
        let content = (self.content)(&handle, window, cx);
        let theme = cx.theme();

        let panel = div()
            .id(("popover-content", self.generation))
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
            // Hidden for the single frame where we don't know our size yet.
            .when(self.hide_until_measured, |this| this.opacity(0.))
            .child(content)
            .child({
                let state = state.clone();
                canvas(
                    move |bounds, _, cx| {
                        state.update(cx, |s, cx| {
                            if s.content_size != bounds.size {
                                s.content_size = bounds.size;
                                cx.notify();
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
                    div().when(self.modal, |this| this.occlude()).on_any_mouse_down(move |_, window, cx| {
                        cx.stop_propagation();
                        handle.close(window, cx);
                    }),
                    window,
                ))
                .child(floating)
                .into_any_element(),
        }
    }
}

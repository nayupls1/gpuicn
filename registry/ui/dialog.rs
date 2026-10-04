//! A window overlaid on either the primary window or another dialog window,
//! rendering the content underneath inert.
//!
//! Also provides the modal machinery used by `alert_dialog`, `sheet` and
//! `drawer`.
//!
//! ```ignore
//! Dialog::new("edit-profile")
//!     .trigger(Button::new("open").outline().label("Edit Profile"))
//!     .title("Edit profile")
//!     .description("Make changes to your profile here. Click save when you're done.")
//!     .content(|_, _, _| div().flex().flex_col().gap_4().child(Input::new("name")))
//!     .footer(|dialog, _, _| {
//!         let dialog = dialog.clone();
//!         Button::new("save").label("Save changes").on_click(move |_, window, cx| dialog.close(window, cx))
//!     })
//! ```

use gpui::{
    Animation, AnimationExt, AnyElement, App, Context, ElementId, Entity, FocusHandle,
    FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px, relative,
};
use std::rc::Rc;
use std::time::Duration;

use super::icon::{Icon, IconName};
use super::portal::{full_window, portal};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

pub struct DialogState {
    open: bool,
    generation: u64,
    needs_focus: bool,
    focus_handle: FocusHandle,
}

impl DialogState {
    fn new(cx: &mut Context<Self>) -> Self {
        Self { open: false, generation: 0, needs_focus: false, focus_handle: cx.focus_handle() }
    }
}

/// Open/close a dialog (or sheet, drawer…) from callbacks.
#[derive(Clone)]
pub struct DialogHandle {
    state: Entity<DialogState>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
}

impl DialogHandle {
    pub fn set_open(&self, open: bool, window: &mut Window, cx: &mut App) {
        let changed = self.state.update(cx, |s, cx| {
            let changed = s.open != open;
            if changed {
                s.open = open;
                if open {
                    s.generation += 1;
                    s.needs_focus = true;
                }
                cx.notify();
            }
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

    pub fn is_open(&self, cx: &App) -> bool {
        self.state.read(cx).open
    }
}

/// Resolved per-frame modal state.
pub struct Modal {
    pub handle: DialogHandle,
    pub open: bool,
    pub generation: u64,
    pub focus_handle: FocusHandle,
}

/// Shared open-state bookkeeping for every modal component.
pub fn use_modal(
    id: &ElementId,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    window: &mut Window,
    cx: &mut App,
) -> Modal {
    let state = window.use_keyed_state(id.clone(), cx, |_, cx| DialogState::new(cx));
    if let Some(open) = open {
        state.update(cx, |s, _| {
            if s.open != open {
                s.open = open;
                if open {
                    s.generation += 1;
                    s.needs_focus = true;
                }
            }
        });
    }
    let (open, generation, needs_focus, focus_handle) = {
        let s = state.read(cx);
        (s.open, s.generation, s.needs_focus, s.focus_handle.clone())
    };
    if open && needs_focus {
        window.focus(&focus_handle);
        state.update(cx, |s, _| s.needs_focus = false);
    }
    Modal { handle: DialogHandle { state, on_open_change }, open, generation, focus_handle }
}

/// Where the modal panel sits inside the backdrop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalPlacement {
    Center,
    Top,
    Right,
    Bottom,
    Left,
}

/// Register the backdrop + panel layer for an open modal.
///
/// `dismissible`: whether clicking the backdrop closes it (false for alert
/// dialogs).
pub fn render_modal(
    modal: &Modal,
    placement: ModalPlacement,
    dismissible: bool,
    window: &mut Window,
    cx: &mut App,
    panel: impl FnOnce(&DialogHandle, &mut Window, &mut App) -> AnyElement + 'static,
) {
    if !modal.open {
        return;
    }
    let handle = modal.handle.clone();
    let focus_handle = modal.focus_handle.clone();
    let generation = modal.generation;
    portal(
        ElementId::NamedInteger("modal".into(), modal.handle.state.entity_id().as_u64()),
        window,
        cx,
        move |window, cx| {
            let overlay = cx.theme().overlay;
            let panel = panel(&handle, window, cx);
            let backdrop_handle = handle.clone();
            let key_handle = handle.clone();

            let panel = div()
                .id("modal-panel")
                .occlude()
                .flex()
                .map(|this| match placement {
                    ModalPlacement::Center => this,
                    ModalPlacement::Top | ModalPlacement::Bottom => this.w_full(),
                    ModalPlacement::Left | ModalPlacement::Right => this.h_full(),
                })
                .child(panel)
                .with_animation(
                    ElementId::NamedInteger("modal-slide".into(), generation),
                    Animation::new(Duration::from_millis(200)).with_easing(gpui::ease_in_out),
                    move |this, delta| {
                        let offset = px(24.) * (1. - delta);
                        match placement {
                            ModalPlacement::Center => this.mt(offset / 2.),
                            ModalPlacement::Top => this.mt(-offset),
                            ModalPlacement::Bottom => this.mb(-offset),
                            ModalPlacement::Left => this.ml(-offset),
                            ModalPlacement::Right => this.mr(-offset),
                        }
                    },
                );

            let layer = div()
                .id("modal-backdrop")
                .track_focus(&focus_handle)
                .occlude()
                .flex()
                .bg(overlay)
                .map(|this| match placement {
                    ModalPlacement::Center => this.items_center().justify_center(),
                    ModalPlacement::Top => this.flex_col().justify_start(),
                    ModalPlacement::Bottom => this.flex_col().justify_end(),
                    ModalPlacement::Left => this.justify_start(),
                    ModalPlacement::Right => this.justify_end(),
                })
                .on_key_down(move |event, window, cx| {
                    if event.keystroke.key == "escape" {
                        cx.stop_propagation();
                        key_handle.close(window, cx);
                    }
                })
                .when(dismissible, |this| {
                    this.on_any_mouse_down(move |_, window, cx| backdrop_handle.close(window, cx))
                })
                .child(panel);

            full_window(
                div().child(layer.size_full().with_animation(
                    ElementId::NamedInteger("modal-fade".into(), generation),
                    Animation::new(Duration::from_millis(150)),
                    |this, delta| this.opacity(delta),
                )),
                window,
            )
            .into_any_element()
        },
    );
}

/// The small "×" in the top-right corner of dialogs and sheets.
pub fn close_button(handle: &DialogHandle) -> impl IntoElement {
    let handle = handle.clone();
    div()
        .id("dialog-close")
        .absolute()
        .top_4()
        .right_4()
        .rounded(px(2.))
        .opacity(0.7)
        .cursor_pointer()
        .hover(|s| s.opacity(1.))
        .child(Icon::new(IconName::X))
        .on_click(move |_, window, cx| handle.close(window, cx))
}

type SlotBuilder = Rc<dyn Fn(&DialogHandle, &mut Window, &mut App) -> AnyElement>;

pub(super) fn slot<E: IntoElement>(f: impl Fn(&DialogHandle, &mut Window, &mut App) -> E + 'static) -> SlotBuilder {
    Rc::new(move |handle, window, cx| f(handle, window, cx).into_any_element())
}

#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    trigger: Option<AnyElement>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    content: Option<SlotBuilder>,
    footer: Option<SlotBuilder>,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    show_close_button: bool,
    style: StyleRefinement,
}

crate::impl_styled!(Dialog);

impl Dialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            title: None,
            description: None,
            content: None,
            footer: None,
            open: None,
            on_open_change: None,
            show_close_button: true,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn content<E: IntoElement>(mut self, content: impl Fn(&DialogHandle, &mut Window, &mut App) -> E + 'static) -> Self {
        self.content = Some(slot(content));
        self
    }

    pub fn footer<E: IntoElement>(mut self, footer: impl Fn(&DialogHandle, &mut Window, &mut App) -> E + 'static) -> Self {
        self.footer = Some(slot(footer));
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }
}

/// Title + description block shared by dialog-like components.
pub(super) fn header(
    title: Option<SharedString>,
    description: Option<SharedString>,
    cx: &App,
) -> Option<gpui::Div> {
    if title.is_none() && description.is_none() {
        return None;
    }
    let muted = cx.theme().muted_foreground;
    Some(
        div()
            .flex()
            .flex_col()
            .gap_2()
            .children(title.map(|title| {
                div()
                    .text_lg()
                    .line_height(relative(1.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(title)
            }))
            .children(description.map(|description| div().text_sm().text_color(muted).child(description))),
    )
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let modal = use_modal(&self.id, self.open, self.on_open_change.clone(), window, cx);
        let trigger_handle = modal.handle.clone();

        let (title, description, content, footer, style, close) =
            (self.title, self.description, self.content, self.footer, self.style, self.show_close_button);
        let window_width = window.viewport_size().width;
        render_modal(&modal, ModalPlacement::Center, true, window, cx, move |handle, _, _| {
            let handle = handle.clone();
            DialogPanel { handle, title, description, content, footer, style, close, window_width }.into_any_element()
        });

        div().id(self.id).flex().children(self.trigger.map(|trigger| {
            div()
                .id("dialog-trigger")
                .child(trigger)
                .on_click(move |_, window, cx| trigger_handle.open(window, cx))
        }))
    }
}

#[derive(IntoElement)]
struct DialogPanel {
    handle: DialogHandle,
    title: Option<SharedString>,
    description: Option<SharedString>,
    content: Option<SlotBuilder>,
    footer: Option<SlotBuilder>,
    style: StyleRefinement,
    close: bool,
    window_width: gpui::Pixels,
}

impl RenderOnce for DialogPanel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let content = self.content.map(|f| f(&self.handle, window, cx));
        let footer = self.footer.map(|f| f(&self.handle, window, cx));
        let header = header(self.title, self.description, cx);
        let theme = cx.theme();
        div()
            .relative()
            .flex()
            .flex_col()
            .gap_4()
            .w(px(512.).min(self.window_width - px(32.)))
            .p_6()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .bg(theme.background)
            .text_color(theme.foreground)
            .when(theme.shadows, |this| this.shadow_lg())
            .refine_style(&self.style)
            .children(header)
            .children(content)
            .children(footer.map(|footer| div().flex().justify_end().gap_2().child(footer)))
            .when(self.close, |this| this.child(close_button(&self.handle)))
    }
}

//! An opinionated toast component.
//!
//! Render a [`Toaster`] once (anywhere in your root view, before the
//! `PortalHost`), then call [`toast`] from anywhere:
//!
//! ```ignore
//! .child(Toaster::new())
//!
//! toast("Event has been created", cx);
//! toast(
//!     Toast::new("Event has been created")
//!         .description("Sunday, December 03, 2023 at 9:00 AM")
//!         .action("Undo", |_, _| println!("Undo")),
//!     cx,
//! );
//! toast(Toast::new("Saved!").success(), cx);
//! ```

use gpui::{
    Animation, AnimationExt, App, Corner, ElementId, FontWeight, Global, IntoElement, RenderOnce,
    SharedString, Task, Window, anchored, div, point, prelude::*, px,
};
use std::rc::Rc;
use std::time::{Duration, Instant};

use super::icon::{Icon, IconName};
use super::portal::portal;
use super::spinner::Spinner;
use super::theme::ActiveTheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Info,
    Warning,
    Error,
    Loading,
}

#[derive(Clone)]
pub struct Toast {
    id: u64,
    title: SharedString,
    description: Option<SharedString>,
    variant: ToastVariant,
    action: Option<(SharedString, Rc<dyn Fn(&mut Window, &mut App)>)>,
    duration: Duration,
    created: Instant,
}

impl Toast {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            id: 0,
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            action: None,
            duration: Duration::from_secs(4),
            created: Instant::now(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn success(self) -> Self {
        self.variant(ToastVariant::Success)
    }

    pub fn info(self) -> Self {
        self.variant(ToastVariant::Info)
    }

    pub fn warning(self) -> Self {
        self.variant(ToastVariant::Warning)
    }

    pub fn error(self) -> Self {
        self.variant(ToastVariant::Error)
    }

    /// Spinner icon; stays until dismissed or updated (`toast.loading`).
    pub fn loading(self) -> Self {
        self.variant(ToastVariant::Loading).duration(Duration::MAX)
    }

    /// A button in the toast; clicking it also dismisses the toast.
    pub fn action(mut self, label: impl Into<SharedString>, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.action = Some((label.into(), Rc::new(handler)));
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    fn expires_at(&self) -> Option<Instant> {
        self.created.checked_add(self.duration)
    }
}

impl From<&str> for Toast {
    fn from(title: &str) -> Self {
        Toast::new(title.to_string())
    }
}

impl From<String> for Toast {
    fn from(title: String) -> Self {
        Toast::new(title)
    }
}

impl From<SharedString> for Toast {
    fn from(title: SharedString) -> Self {
        Toast::new(title)
    }
}

#[derive(Default)]
struct ToastStore {
    toasts: Vec<Toast>,
    next_id: u64,
    timer: Option<(Instant, Task<()>)>,
}

impl Global for ToastStore {}

/// Show a toast. Returns its id for [`dismiss`] / [`update`].
pub fn toast(toast: impl Into<Toast>, cx: &mut App) -> u64 {
    let mut toast = toast.into();
    let store = cx.default_global::<ToastStore>();
    store.next_id += 1;
    toast.id = store.next_id;
    toast.created = Instant::now();
    let id = toast.id;
    store.toasts.push(toast);
    cx.refresh_windows();
    id
}

/// Replace a toast's content (e.g. turn a loading toast into a success).
pub fn update(id: u64, toast: impl Into<Toast>, cx: &mut App) {
    let mut toast = toast.into();
    let store = cx.default_global::<ToastStore>();
    if let Some(existing) = store.toasts.iter_mut().find(|t| t.id == id) {
        toast.id = id;
        toast.created = Instant::now();
        *existing = toast;
    }
    cx.refresh_windows();
}

pub fn dismiss(id: u64, cx: &mut App) {
    cx.default_global::<ToastStore>().toasts.retain(|t| t.id != id);
    cx.refresh_windows();
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToasterPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

/// Renders the active toasts. Place once in your root view.
#[derive(IntoElement)]
pub struct Toaster {
    position: ToasterPosition,
    visible: usize,
}

impl Toaster {
    pub fn new() -> Self {
        Self { position: ToasterPosition::BottomRight, visible: 3 }
    }

    pub fn position(mut self, position: ToasterPosition) -> Self {
        self.position = position;
        self
    }

    /// Maximum toasts shown at once (default 3).
    pub fn visible_toasts(mut self, visible: usize) -> Self {
        self.visible = visible.max(1);
        self
    }
}

impl Default for Toaster {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Toaster {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let now = Instant::now();
        let store = cx.default_global::<ToastStore>();
        store.toasts.retain(|t| t.expires_at().is_none_or(|at| at > now));
        let next_expiry = store.toasts.iter().filter_map(|t| t.expires_at()).min();
        let toasts: Vec<Toast> = store.toasts.iter().rev().take(self.visible).cloned().collect();

        // Wake up when the next toast expires.
        if let Some(at) = next_expiry
            && store.timer.as_ref().is_none_or(|(scheduled, _)| *scheduled != at)
        {
            let task = cx.spawn(async move |cx| {
                cx.background_executor().timer(at.saturating_duration_since(Instant::now())).await;
                cx.update(|cx| cx.refresh_windows()).ok();
            });
            cx.default_global::<ToastStore>().timer = Some((at, task));
        }

        if !toasts.is_empty() {
            let position = self.position;
            portal("sonner-toaster", window, cx, move |window, cx| {
                let viewport = window.viewport_size();
                let margin = px(24.);
                let (anchor_point, corner) = match position {
                    ToasterPosition::TopLeft => (point(margin, margin), Corner::TopLeft),
                    ToasterPosition::TopCenter => (point(viewport.width / 2. - px(178.), margin), Corner::TopLeft),
                    ToasterPosition::TopRight => (point(viewport.width - margin, margin), Corner::TopRight),
                    ToasterPosition::BottomLeft => (point(margin, viewport.height - margin), Corner::BottomLeft),
                    ToasterPosition::BottomCenter => (point(viewport.width / 2. - px(178.), viewport.height - margin), Corner::BottomLeft),
                    ToasterPosition::BottomRight => (point(viewport.width - margin, viewport.height - margin), Corner::BottomRight),
                };
                let top = matches!(position, ToasterPosition::TopLeft | ToasterPosition::TopCenter | ToasterPosition::TopRight);
                let cards: Vec<_> = toasts.into_iter().map(|t| ToastCard(t).into_any_element()).collect();
                let cards: Vec<_> = if top { cards } else { cards.into_iter().rev().collect() };
                let _ = cx;
                anchored()
                    .position(anchor_point)
                    .anchor(corner)
                    .child(div().flex().flex_col().gap_3p5().w(px(356.)).children(cards))
                    .into_any_element()
            });
        }
        gpui::Empty
    }
}

#[derive(IntoElement)]
struct ToastCard(Toast);

impl RenderOnce for ToastCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let toast = self.0;
        let theme = cx.theme();
        let icon = match toast.variant {
            ToastVariant::Default => None,
            ToastVariant::Success => Some(Icon::new(IconName::CircleCheck).into_any_element()),
            ToastVariant::Info => Some(Icon::new(IconName::Info).into_any_element()),
            ToastVariant::Warning => Some(Icon::new(IconName::TriangleAlert).into_any_element()),
            ToastVariant::Error => Some(Icon::new(IconName::CircleX).into_any_element()),
            ToastVariant::Loading => Some(Spinner::new().id(("toast-spinner", toast.id)).into_any_element()),
        };
        let id = toast.id;

        div()
            .id(ElementId::NamedInteger("toast".into(), id))
            .occlude()
            .flex()
            .items_center()
            .gap_1p5()
            .w_full()
            .p_4()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .text_size(px(13.))
            .when(theme.shadows, |this| this.shadow_lg())
            .children(icon.map(|icon| div().flex_none().mr_1().child(icon)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .gap_0p5()
                    .child(div().font_weight(FontWeight::MEDIUM).child(toast.title))
                    .children(toast.description.map(|d| div().text_color(theme.muted_foreground).child(d))),
            )
            .children(toast.action.map(|(label, handler)| {
                div()
                    .id("toast-action")
                    .flex_none()
                    .h_6()
                    .px_2()
                    .flex()
                    .items_center()
                    .rounded(theme.radius_sm())
                    .bg(theme.primary)
                    .text_color(theme.primary_foreground)
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .child(label)
                    .on_click(move |_, window, cx| {
                        handler(window, cx);
                        dismiss(id, cx);
                    })
            }))
            .with_animation(
                ElementId::NamedInteger("toast-in".into(), id),
                Animation::new(Duration::from_millis(250)).with_easing(gpui::ease_in_out),
                |this, delta| this.opacity(delta).mt(px(12.) * (1. - delta)),
            )
    }
}

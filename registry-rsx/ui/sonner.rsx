//! An opinionated toast component.
//!
//! Render a [`Toaster`] once (anywhere in your root component), then call
//! [`toast`] from anywhere: no context needed, the toasts live in a global
//! signal like the theme.
//!
//! ```ignore
//! <Toaster />
//!
//! toast("Event has been created");
//! toast(
//!     Toast::new("Event has been created")
//!         .description("Sunday, December 03, 2023 at 9:00 AM")
//!         .action("Undo", || println!("Undo")),
//! );
//! toast(Toast::new("Saved!").success());
//!
//! // Loading, then done:
//! let id = toast(Toast::new("Uploading…").loading());
//! update(id, Toast::new("Upload complete").success());
//! ```

use std::rc::Rc;
use std::time::{Duration, Instant};

use rsx::prelude::*;
use rsx::Scope;

use super::icon::{Icon, IconName};
use super::spinner::Spinner;
use super::theme::theme;
use super::utils::{ease_in_out, use_transition};

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
    /// Bumped by [`update`], so the card and its timer follow the new content.
    revision: u64,
    title: SharedString,
    description: Option<SharedString>,
    variant: ToastVariant,
    action: Option<(SharedString, Rc<dyn Fn()>)>,
    duration: Duration,
    created: Instant,
}

impl PartialEq for Toast {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.revision == other.revision
    }
}

impl Toast {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            id: 0,
            revision: 0,
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
    pub fn action(mut self, label: impl Into<SharedString>, handler: impl Fn() + 'static) -> Self {
        self.action = Some((label.into(), Rc::new(handler)));
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &SharedString {
        &self.title
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
}

thread_local! {
    /// The active toasts, oldest first. Global, like the theme.
    static TOASTS: Signal<ToastStore> = {
        let scope = Scope::root();
        let s = scope.run(|| signal(ToastStore::default()));
        std::mem::forget(scope);
        s
    };
}

fn store() -> Signal<ToastStore> {
    TOASTS.with(|s| *s)
}

/// The active toasts, oldest first (tracked).
pub fn toasts() -> Vec<Toast> {
    store().with(|s| s.toasts.clone())
}

/// Dismiss `id` when its time is up, unless it was updated or dismissed since.
fn schedule_expiry(toast: &Toast) {
    let Some(at) = toast.expires_at() else { return };
    let (id, revision) = (toast.id, toast.revision);
    spawn_local(async move {
        sleep(at.saturating_duration_since(Instant::now())).await;
        let current = store().with_untracked(|s| s.toasts.iter().any(|t| t.id == id && t.revision == revision));
        if current {
            dismiss(id);
        }
    });
}

/// Show a toast. Returns its id for [`dismiss`] / [`update`].
pub fn toast(toast: impl Into<Toast>) -> u64 {
    let mut toast = toast.into();
    let id = store().update(|s| {
        s.next_id += 1;
        s.next_id
    });
    toast.id = id;
    toast.created = Instant::now();
    schedule_expiry(&toast);
    store().update(|s| s.toasts.push(toast));
    id
}

/// Replace a toast's content (e.g. turn a loading toast into a success).
pub fn update(id: u64, toast: impl Into<Toast>) {
    let mut toast = toast.into();
    let found = store().with_untracked(|s| s.toasts.iter().find(|t| t.id == id).map(|t| t.revision));
    let Some(revision) = found else { return };
    toast.id = id;
    toast.revision = revision + 1;
    toast.created = Instant::now();
    schedule_expiry(&toast);
    store().update(|s| {
        if let Some(existing) = s.toasts.iter_mut().find(|t| t.id == id) {
            *existing = toast;
        }
    });
}

pub fn dismiss(id: u64) {
    if store().with_untracked(|s| s.toasts.iter().any(|t| t.id == id)) {
        store().update(|s| s.toasts.retain(|t| t.id != id));
    }
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

/// Renders the active toasts. Place once in your root component.
#[component]
pub fn Toaster(
    #[prop(optional)] position: ToasterPosition,
    /// Maximum toasts shown at once (default 3).
    #[prop(default = 3)]
    visible_toasts: usize,
) -> impl IntoElement {
    let visible = visible_toasts.max(1);
    let top = matches!(position, ToasterPosition::TopLeft | ToasterPosition::TopCenter | ToasterPosition::TopRight);
    // Newest first at the top edge, newest last at the bottom edge.
    let shown = memo(move || {
        let newest: Vec<Toast> = store().with(|s| s.toasts.iter().rev().take(visible).cloned().collect());
        if top { newest } else { newest.into_iter().rev().collect() }
    });
    // Anchored 24px in from the corner (centered: across the window).
    let (row, edge) = match position {
        ToasterPosition::TopLeft => ("left-[24px] justify-start", "top-[24px]"),
        ToasterPosition::TopCenter => ("left-0 right-0 justify-center", "top-[24px]"),
        ToasterPosition::TopRight => ("right-[24px] justify-end", "top-[24px]"),
        ToasterPosition::BottomLeft => ("left-[24px] justify-start", "bottom-[24px]"),
        ToasterPosition::BottomCenter => ("left-0 right-0 justify-center", "bottom-[24px]"),
        ToasterPosition::BottomRight => ("right-[24px] justify-end", "bottom-[24px]"),
    };
    <Show when={!shown.with(|s| s.is_empty())}>
        <Portal>
            <div class={format!("absolute flex {row} {edge}")}>
                <div class="flex flex-col gap-3.5 w-[356px]">
                    <Key each={shown} by={|t: &Toast| t.id}>
                        {|toast: ReadSignal<Toast>| <ToastCard toast={toast} />}
                    </Key>
                </div>
            </div>
        </Portal>
    </Show>
}

#[component]
fn ToastCard(toast: ReadSignal<Toast>) -> impl IntoElement {
    let t = theme();
    let enter = use_transition(250);
    let id = toast.get_untracked().id;
    // rsx#7: no `shadow-lg`. rsx#16: slides in with a margin, not a transform.
    <div
        class="flex items-center gap-1.5 w-full p-4 border text-[13px]"
        corner_radius={t.get().radius_lg()}
        border_color={t.get().border}
        bg={t.get().popover}
        text_color={t.get().popover_foreground}
        opacity={ease_in_out(enter.get())}
        mt={px(12.0 * (1.0 - ease_in_out(enter.get())))}
        // Clicks on a toast don't reach what's under it.
        onMouseDown={move |e: &MouseEvent| e.stop_propagation()}
    >
        {move || {
            let icon = match toast.with(|t| t.variant) {
                ToastVariant::Default => None,
                ToastVariant::Success => Some(<Icon name={IconName::CircleCheck} />),
                ToastVariant::Info => Some(<Icon name={IconName::Info} />),
                ToastVariant::Warning => Some(<Icon name={IconName::TriangleAlert} />),
                ToastVariant::Error => Some(<Icon name={IconName::CircleX} />),
                ToastVariant::Loading => Some(<Spinner />),
            };
            icon.map(|icon| <div class="flex-none mr-1">{icon}</div>)
        }}
        <div class="flex flex-col flex-1 min-w-0 gap-0.5">
            <div class="font-medium">{move || toast.with(|t| t.title.clone())}</div>
            {move || toast.with(|t| t.description.clone()).map(|d| <div text_color={t.get().muted_foreground}>{d}</div>)}
        </div>
        {move || toast.with(|t| t.action.clone()).map(|(label, handler)| <div
            class="flex-none h-6 px-2 flex items-center text-xs leading-[1rem] font-medium cursor-pointer"
            corner_radius={t.get().radius_sm()}
            bg={t.get().primary}
            text_color={t.get().primary_foreground}
            onClick={move |e: &ClickEvent| {
                e.stop_propagation();
                handler();
                dismiss(id);
            }}
        >
            {label}
        </div>)}
    </div>
}

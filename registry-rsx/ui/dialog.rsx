//! A window overlaid on either the primary window or another dialog window,
//! rendering the content underneath inert.
//!
//! Also provides the modal machinery used by `alert_dialog`, `sheet` and
//! `drawer` ([`DialogHandle`], [`use_modal`], [`Modal`], [`close_button`], [`header`]).
//!
//! ```ignore
//! <Dialog
//!     trigger={<Button outline label="Edit Profile" />}
//!     title="Edit profile"
//!     description="Make changes to your profile here. Click save when you're done."
//!     content={|_| <div class="flex flex-col gap-4"><Input default_value="Pedro Duarte" /></div>}
//!     footer={|dialog: DialogHandle| <Button label="Save changes" onClick={move |_| dialog.close()} />}
//! />
//!
//! // Controlled:
//! let open = signal(false);
//! <Dialog open={open} onOpenChange={move |o: &bool| open.set(*o)} title="Settings" />
//! ```

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::theme::theme;
use super::utils::{ease_in_out, pointer_pressed, use_transition};

/// Open/close a dialog (or sheet, drawer…) from callbacks. `Copy`: content
/// and footer render functions receive it; create one with
/// [`DialogHandle::new`] and pass `handle={..}` to drive a dialog from outside.
#[derive(Clone, Copy)]
pub struct DialogHandle {
    open: Signal<bool>,
    on_open_change: StoredValue<Option<Callback<bool>>>,
    /// The backdrop: takes focus while open, so Escape reaches it.
    layer: NodeRef,
    /// The trigger wrapper: focus returns here on close.
    trigger: NodeRef,
}

impl Default for DialogHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogHandle {
    pub fn new() -> Self {
        DialogHandle { open: signal(false), on_open_change: StoredValue::new(None), layer: NodeRef::new(), trigger: NodeRef::new() }
    }

    pub fn set_open(&self, open: bool) {
        if self.apply(open) {
            if let Some(f) = self.on_open_change.get_value() {
                f.call(&open);
            }
        }
    }

    pub fn open(&self) {
        self.set_open(true)
    }

    pub fn close(&self) {
        self.set_open(false)
    }

    /// Whether the dialog is open (tracked).
    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    /// Whether the dialog is open, without subscribing.
    pub fn is_open_untracked(&self) -> bool {
        self.open.get_untracked()
    }

    /// The trigger wrapper (focus returns to it on close).
    pub fn trigger_ref(&self) -> NodeRef {
        self.trigger
    }

    /// The full-window backdrop of the open dialog.
    pub fn layer_ref(&self) -> NodeRef {
        self.layer
    }

    /// Change the state without calling `on_open_change`. Returns whether it changed.
    fn apply(&self, open: bool) -> bool {
        if self.open.get_untracked() == open {
            return false;
        }
        self.open.set(open);
        if !open {
            // Return focus to the trigger, if there is one.
            let trigger = self.trigger;
            spawn_local(async move {
                if trigger.bounds().is_some() {
                    trigger.focus();
                }
            });
        }
        true
    }
}

/// Shared open-state bookkeeping for every modal component: the handle
/// (yours, or a new one), kept in sync with a controlled `open`.
pub fn use_modal(open: Option<Accessor<bool>>, on_open_change: Option<Callback<bool>>, handle: Option<DialogHandle>) -> DialogHandle {
    let handle = handle.unwrap_or_default();
    handle.on_open_change.set_value(on_open_change);
    if let Some(open) = open {
        effect(move || {
            let open = open.get();
            handle.apply(open);
        });
    }
    handle
}

/// Where the modal panel sits inside the backdrop.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModalPlacement {
    #[default]
    Center,
    Top,
    Right,
    Bottom,
    Left,
}

/// The backdrop + panel layer of an open modal (gpui's `render_modal`):
/// a full-window `<Portal>` dimmed with `theme.overlay`, the panel placed by
/// `placement`, Escape closes, and so does a press on the backdrop when
/// `dismissible` (false for alert dialogs). Fades in and slides 24px.
#[component]
pub fn Modal(
    handle: DialogHandle,
    #[prop(optional)] placement: ModalPlacement,
    #[prop(default = true)] dismissible: bool,
    /// Classes for the box around the panel (e.g. `max-h-[80%]`); it is
    /// sized by the panel and bounded by the window.
    wrapper_class: Option<String>,
    /// Renders the panel.
    panel: Render<DialogHandle>,
) -> impl IntoElement {
    let wrapper_class = StoredValue::new(wrapper_class.unwrap_or_default());
    let layer = move || {
        let t = theme();
        let fade = use_transition(150);
        let slide = use_transition(200);
        // Focus the backdrop when it appears (applied when it's first drawn).
        handle.layer.focus();
        let backdrop = match placement {
            ModalPlacement::Center => "items-center justify-center px-4",
            ModalPlacement::Top => "flex-col justify-start",
            ModalPlacement::Bottom => "flex-col justify-end",
            ModalPlacement::Left => "justify-start",
            ModalPlacement::Right => "justify-end",
        };
        let fit = match placement {
            ModalPlacement::Center => "max-w-full",
            ModalPlacement::Top | ModalPlacement::Bottom => "w-full",
            ModalPlacement::Left | ModalPlacement::Right => "h-full",
        };
        <Portal>
            <div
                ref={handle.layer}
                class={format!("absolute inset-0 flex {backdrop}")}
                bg={t.get().overlay}
                opacity={fade.get()}
                onKeyDown={move |e: &KeyboardEvent| {
                    if e.key == "Escape" {
                        e.stop_propagation();
                        handle.close();
                    }
                }}
                onMouseDown={move |e: &MouseEvent| {
                    e.stop_propagation();
                    if dismissible {
                        pointer_pressed();
                        handle.close();
                    }
                }}
            >
                <div
                    class={format!("flex {fit} {}", wrapper_class.get_value())}
                    // A press inside the panel doesn't reach the backdrop.
                    onMouseDown={move |e: &MouseEvent| e.stop_propagation()}
                    bind={move |el: rsx::Div| {
                        // rsx#16: no transforms; slide with margins like gpuicn does.
                        let offset = 24.0 * (1.0 - ease_in_out(slide.get()));
                        match placement {
                            ModalPlacement::Center => el.mt(px(offset / 2.0)),
                            ModalPlacement::Top => el.mt(px(-offset)),
                            ModalPlacement::Bottom => el.mb(px(-offset)),
                            ModalPlacement::Left => el.ml(px(-offset)),
                            ModalPlacement::Right => el.mr(px(-offset)),
                        }
                    }}
                >
                    {panel.render(handle)}
                </div>
            </div>
        </Portal>
    };
    <Show when={handle.open.get()}>{layer()}</Show>
}

/// The small "×" in the top-right corner of dialogs and sheets.
pub fn close_button(handle: DialogHandle) -> Element {
    let hovered = signal(false);
    <div
        class="absolute top-4 right-4 rounded-[2px] cursor-pointer"
        opacity={if hovered.get() { 1.0 } else { 0.7 }}
        onHover={move |h: &bool| hovered.set(*h)}
        onClick={move |e: &ClickEvent| {
            e.stop_propagation();
            handle.close();
        }}
    >
        <Icon name={IconName::X} />
    </div>
}

/// Title + description block shared by dialog-like components. `class` is
/// added after the defaults (`flex flex-col gap-2`), e.g. `"gap-1.5 p-4"`.
pub fn header(title: Option<Accessor<String>>, description: Option<Accessor<String>>, class: &str) -> Option<Element> {
    if title.is_none() && description.is_none() {
        return None;
    }
    let t = theme();
    Some(
        <div class={format!("flex flex-col gap-2 {class}")}>
            {title.map(|title| <div class="text-lg leading-none font-semibold">{title}</div>)}
            {description.map(|d| <div class="text-sm leading-[1.25rem]" text_color={t.get().muted_foreground}>{d}</div>)}
        </div>,
    )
}

/// The element that opens a modal: clicking it calls `handle.open()`.
pub fn modal_trigger(handle: DialogHandle, trigger: Option<Children>) -> Option<Element> {
    trigger.map(|trigger| {
        <div
            ref={handle.trigger}
            class="flex"
            onClick={move |_| {
                pointer_pressed();
                handle.open();
            }}
        >
            {trigger}
        </div>
    })
}

#[component]
pub fn Dialog(
    /// The element that opens the dialog.
    trigger: Option<Children>,
    title: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// The body, between the header and the footer.
    content: Option<Render<DialogHandle>>,
    /// Buttons, right-aligned under the content.
    footer: Option<Render<DialogHandle>>,
    /// Control the open state from outside.
    open: Option<Accessor<bool>>,
    on_open_change: Option<Callback<bool>>,
    /// Share state with the caller: `handle={dialog}` with `let dialog = DialogHandle::new();`.
    handle: Option<DialogHandle>,
    #[prop(default = true)] show_close_button: bool,
    /// Styles for the panel.
    attrs: Attributes,
) -> impl IntoElement {
    let handle = use_modal(open, on_open_change, handle);
    let t = theme();
    let attrs = StoredValue::new(attrs);
    let panel = move |handle: DialogHandle| {
        // rsx#7: no `shadow-lg`. Colors are bindings so the content isn't
        // rebuilt on theme changes (rsx#10: static classes can't override them).
        <div
            class="relative flex flex-col gap-4 w-[512px] max-w-full p-6 border"
            corner_radius={t.get().radius_lg()}
            border_color={t.get().border}
            bg={t.get().background}
            text_color={t.get().foreground}
            {...attrs.with_value(|a| a.clone())}
        >
            {header(title, description, "")}
            {content.map(|c| c.render(handle))}
            {footer.map(|f| <div class="flex justify-end gap-2">{f.render(handle)}</div>)}
            {show_close_button.then(|| close_button(handle))}
        </div>
    };
    <div class="flex">
        {modal_trigger(handle, trigger)}
        <Modal handle={handle} placement={ModalPlacement::Center} panel={panel} />
    </div>
}

//! Displays a form input field or a component that looks like an input field.
//!
//! The text editing itself (caret, selection, clipboard, undo, IME, Enter
//! submitting a surrounding `<form>`) is rsx's own `<input>`/`<textarea>`;
//! this component gives it the shadcn look and gpuicn's options. `Textarea`,
//! `InputGroup` and `Form` are built on it.
//!
//! `input::init()` (called by `ui::init`) installs nothing: rsx's text
//! fields come with their key bindings.
//!
//! ```ignore
//! // Self-managed: the text lives as long as the input.
//! <Input placeholder="Email" onChange={move |text: &String| println!("{text}")} />
//!
//! // Two-way binding: own the text in a signal, read/write it from anywhere.
//! let email = signal(String::new());
//! <Input text={email} placeholder="Email" />
//! email.get()
//!
//! // Controlled: shows `value`, reports edits through `onChange`.
//! <Input value={name.get()} onChange={move |t: &String| name.set(t.to_uppercase())} />
//! ```
//!
//! gpuicn's `InputState` / `Input::from_state` / keyed state have no rsx
//! counterpart: components run once, so the input's state is a signal. Pass
//! `text={signal}` to own it (the equivalent of `Input::from_state(&state)`
//! plus `state.read(cx).text()` / `set_text`).

use rsx::prelude::*;
use rsx::{Cursor, Div};

use super::theme::{theme, ColorExt, Theme};
use super::utils::{focus_ring, invalid_ring, on_tab_key, pointer_pressed, to_px};

/// Called by `ui::init`. Nothing to install: rsx text fields bring their own
/// editing keys (gpuicn binds them here).
pub fn init() {}

/// The surface of a form control: `bg-transparent dark:bg-input/30 shadow-xs`
/// (gpuicn's `control_surface`; the shadow only in light mode, and only with
/// `theme.shadows`).
pub fn control_surface(theme: &Theme, el: Div) -> Div {
    if theme.is_dark() {
        el.bg(theme.input.opacity(0.3))
    } else if theme.shadows {
        el.bg(theme.background).shadow_xs()
    } else {
        el.bg(theme.background)
    }
}

/// Provided by `InputGroup` to the `Input` inside it: the input then drops
/// its own border and ring and reports its focus, so the group draws them.
#[derive(Clone, Copy)]
pub struct InputGroupContext {
    pub focused: Signal<bool>,
}

#[component]
pub fn Input(
    placeholder: Option<Accessor<String>>,
    /// Initial text for self-managed inputs.
    default_value: Option<String>,
    /// Controlled text: the input shows this; edits go to `on_change`.
    value: Option<Accessor<String>>,
    /// Two-way binding: the input reads and writes this signal (replaces
    /// gpuicn's `Input::from_state`).
    text: Option<Signal<String>>,
    /// `type="password"`
    masked: Option<Accessor<bool>>,
    disabled: Option<Accessor<bool>>,
    /// `aria-invalid`: red border and ring.
    invalid: Option<Accessor<bool>>,
    /// Multi-line, wrapping text (used by `Textarea`).
    multiline: bool,
    /// Focus the input the first time it is rendered.
    autofocus: bool,
    /// Without border, background, padding and focus ring (for `InputGroup`).
    unstyled: bool,
    /// Element shown before the text, inside the border.
    prefix: Option<Children>,
    /// Element shown after the text, inside the border.
    suffix: Option<Children>,
    on_change: Option<Callback<String>>,
    /// Enter in a single-line input.
    on_submit: Option<Callback<String>>,
    /// Connected to the text field, to focus it (`node.focus()`) or read its focus.
    node_ref: Option<NodeRef>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let group = use_context::<InputGroupContext>();
    let appearance = !unstyled && group.is_none();
    let node = node_ref.unwrap_or_default();
    let flag = |a: Option<Accessor<bool>>| memo(move || a.map(|a| a.get()).unwrap_or(false));
    let (masked, disabled, invalid) = (flag(masked), flag(disabled), flag(invalid));

    // The text shown: the bound signal, or a local one (synced from `value`
    // when controlled).
    let initial = value.map(|v| v.get_untracked()).or(default_value).unwrap_or_default();
    let text = text.unwrap_or_else(|| signal(initial));
    if let Some(value) = value {
        effect(move || text.set(value.get()));
    }
    if let Some(group) = group {
        effect(move || group.focused.set(node.is_focused()));
    }
    let on_input = move |e: &InputEvent| {
        let next = e.value.to_string();
        if value.is_none() {
            text.set(next.clone());
        }
        if let Some(f) = on_change {
            f.call(&next);
        }
    };
    let on_key = move |e: &KeyboardEvent| {
        on_tab_key(e);
        if e.key == "Enter" && !multiline {
            if let Some(f) = on_submit {
                f.call(&text.get_untracked());
            }
        }
    };

    // The text field: rsx's own `<input>` / `<textarea>` (which wraps), with
    // its default border, padding and radius reset so it's just the text, in
    // gpuicn's colors: a `muted-foreground` placeholder, a `primary/25`
    // selection and a `foreground` caret. A disabled field is inert.
    let field = rsx::element(if multiline { "textarea" } else { "input" })
        .class("flex-1 w-full min-w-0 min-h-[1.25rem] p-0 border-0 rounded-none")
        .bg(Color::TRANSPARENT)
        // For the HTML target (GPUI ignores unknown CSS): the page's font, no
        // resize handle or browser outline.
        .css("font: inherit; resize: none; outline: none")
        .node_ref(node)
        .attr("value", text)
        .attr("placeholder", move || placeholder.map(|p| p.get()))
        .attr("type", move || if multiline { None } else { Some(if masked.get() { "password" } else { "text" }) })
        .attr("disabled", disabled)
        .attr("autofocus", autofocus)
        .bind(move |el| {
            let theme = t.get();
            el.placeholder_color(theme.muted_foreground)
                .selection_background(theme.primary.opacity(0.25))
                .caret_color(theme.foreground)
                .cursor(if disabled.get() { Cursor::NotAllowed } else { Cursor::Text })
        })
        .on_input(on_input)
        .on_key_down(on_key)
        .on_mouse_down(move |_| pointer_pressed());

    // Border, surface and disabled state, reactive (a binding, so the text
    // field inside is never rebuilt and keeps its focus and caret).
    let surface = move |el: Div| {
        let theme = t.get();
        let el = if appearance {
            let border = if invalid.get() {
                theme.destructive
            } else if node.is_focused() {
                theme.ring
            } else {
                theme.input
            };
            control_surface(&theme, el.corner_radius(theme.radius_md()).border_color(border))
        } else {
            el
        };
        // No shadow when faded: it would tint a disabled field grey.
        if disabled.get() { el.shadow_none().opacity(0.5).cursor(Cursor::NotAllowed) } else { el.cursor(Cursor::Text) }
    };

    <div
        class="flex gap-2 w-full min-w-0 text-sm leading-[1.25rem]"
        class:items-start={multiline}
        class:items-center={!multiline}
        class:relative={appearance}
        class:border={appearance}
        class:px-3={appearance}
        class:h-9={appearance && !multiline}
        class:py-1={appearance && !multiline}
        class:min-h-16={appearance && multiline}
        class:py-2={appearance && multiline}
        text_color={t.get().foreground}
        bind={surface}
        // A press on the padding, prefix or suffix focuses the field too.
        onMouseDown={move |_| if !disabled.get_untracked() { node.focus() }}
        {...attrs}
    >
        {prefix}
        <div class="flex flex-1 min-w-0 overflow-hidden" class:items-start={multiline} class:items-center={!multiline}>
            {field}
        </div>
        {suffix}
        {move || {
            let theme = t.get();
            let radius = to_px(theme.radius_md());
            (appearance && !disabled.get() && (node.is_focused() || invalid.get()))
                .then(|| if invalid.get() { invalid_ring(&theme, radius) } else { focus_ring(&theme, radius) })
        }}
    </div>
}

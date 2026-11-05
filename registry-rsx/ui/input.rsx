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

use super::theme::{theme, ColorExt};
use super::utils::{control_surface, focus_ring, invalid_ring, on_tab_key, pointer_pressed, to_px};

/// Called by `ui::init`. Nothing to install: rsx text fields bring their own
/// editing keys (gpuicn binds them here).
pub fn init() {}

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

    // rsx-gpui focuses every focusable ancestor (anything with a `ref` or a
    // key handler) on click, the outermost last, so a click would move the
    // focus out of the field (e.g. to a popover's panel). Keep clicks here.
    let stop_click = move |e: &ClickEvent| e.stop_propagation();

    // rsx#13: rsx's text fields add their own border, padding, radius and
    // `min-w-[160px]`; reset them so the field is just the text. The
    // placeholder is drawn here (rsx's is a fixed grey that hides on focus);
    // the caret and selection keep rsx's blue.
    let field = move || {
        let tag = if multiline { "textarea" } else { "input" };
        rsx::element(tag)
            .class("flex-1 w-full min-w-0 min-h-[1.25rem] p-0 border-0 rounded-none")
            .bg(Color::TRANSPARENT)
            // For the HTML target (GPUI ignores unknown CSS): no resize handle or browser outline.
            .css("resize: none; outline: none")
            .node_ref(node)
            .attr("value", move || Some(text.get()))
            .attr("type", move || if multiline { None } else { Some(if masked.get() { "password" } else { "text" }) })
            .attr("autofocus", autofocus)
            .on_input(on_input)
            .on_key_down(on_key)
            .on_mouse_down(move |_| pointer_pressed())
            .on_click(stop_click)
    };

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
        if disabled.get() { el.opacity(0.5).cursor(Cursor::NotAllowed) } else { el.cursor(Cursor::Text) }
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
        onMouseDown={move |e: &MouseEvent| {
            // Also keeps the press from a click-away backdrop under the
            // input (an open popover's), which would close it.
            e.stop_propagation();
            if !disabled.get_untracked() {
                node.focus()
            }
        }}
        onClick={stop_click}
        {...&attrs}
    >
        {prefix}
        <div class="relative flex flex-1 min-w-0 min-h-[1.25rem] overflow-hidden" class:items-start={multiline} class:items-center={!multiline}>
            {move || (text.with(|s| s.is_empty()) && placeholder.is_some()).then(|| <div
                class="absolute inset-0 flex overflow-hidden"
                class:items-start={multiline}
                class:items-center={!multiline}
                text_color={t.get().muted_foreground}
            >{placeholder}</div>)}
            {move || if disabled.get() {
                // rsx-gpui ignores `disabled` on text fields: show the text instead.
                let shown = move || if masked.get() { "•".repeat(text.with(|s| s.chars().count())) } else { text.get() };
                <div class="flex-1 min-w-0 overflow-hidden">{shown}</div>
            } else {
                field().into_element()
            }}
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

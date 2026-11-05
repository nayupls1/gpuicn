//! Displays a form textarea or a component that looks like a textarea.
//! Grows with its content (`field-sizing-content`).
//!
//! rsx-gpui gap: its `<textarea>` doesn't wrap long lines yet (one row per
//! line break, the rest is clipped); gpuicn's wraps.
//!
//! ```ignore
//! <Textarea placeholder="Type your message here." />
//! <Textarea text={message} invalid={message.with(|m| m.len() > 280)} />
//! ```

use rsx::prelude::*;

use super::input::Input;

#[component]
pub fn Textarea(
    placeholder: Option<Accessor<String>>,
    default_value: Option<String>,
    /// Controlled text (see `Input`).
    value: Option<Accessor<String>>,
    /// Two-way binding (replaces gpuicn's `Textarea::from_state`).
    text: Option<Signal<String>>,
    disabled: Option<Accessor<bool>>,
    invalid: Option<Accessor<bool>>,
    on_change: Option<Callback<String>>,
    node_ref: Option<NodeRef>,
    attrs: Attributes,
) -> impl IntoElement {
    // An `Option<Callback>` prop can't be forwarded as is, hence the closure.
    <Input
        multiline
        placeholder={placeholder}
        default_value={default_value}
        value={value}
        text={text}
        disabled={disabled}
        invalid={invalid}
        on_change={move |t: &String| if let Some(f) = on_change { f.call(t) }}
        node_ref={node_ref}
        {...attrs}
    />
}

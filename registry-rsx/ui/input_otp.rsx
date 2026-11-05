//! Accessible one-time password component with copy paste functionality.
//!
//! ```ignore
//! <InputOtp
//!     length={6usize} // rsx: an integer literal needs its suffix for a `usize` prop
//!     groups={[3, 3]}
//!     onComplete={move |code: &String| println!("{code}")}
//! />
//! ```
//!
//! The slots are drawn over an invisible rsx text field, which takes the
//! typing, pasting and IME input.

use rsx::prelude::*;
use rsx::{Cursor, Div, Length};

use super::icon::{Icon, IconName};
use super::theme::{theme, ColorExt, Theme};
use super::utils::{control_surface, on_tab_key, pointer_pressed};

/// Which characters a slot accepts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OtpPattern {
    #[default]
    Digits,
    Alphanumeric,
}

impl OtpPattern {
    fn accepts(self, c: char) -> bool {
        match self {
            OtpPattern::Digits => c.is_ascii_digit(),
            OtpPattern::Alphanumeric => c.is_ascii_alphanumeric(),
        }
    }
}

/// Sets per-side borders and corners (rsx has no `border-y`, `rounded-l`, ...).
fn sides(mut el: Div, left: bool, radius_left: Option<Length>, radius_right: Option<Length>) -> Div {
    let one = Some(px(1.0));
    let s = el.style();
    s.border_widths.top = one;
    s.border_widths.bottom = one;
    s.border_widths.right = one;
    s.border_widths.left = if left { one } else { Some(px(0.0)) };
    if let Some(r) = radius_left {
        s.corner_radii.top_left = Some(r);
        s.corner_radii.bottom_left = Some(r);
    }
    if let Some(r) = radius_right {
        s.corner_radii.top_right = Some(r);
        s.corner_radii.bottom_right = Some(r);
    }
    el
}

fn slot(theme: &Theme, ch: Option<char>, first: bool, last: bool, active: bool, caret: bool) -> Div {
    let radius = theme.radius_md();
    let el = div()
        .class("relative flex size-9 items-center justify-center text-sm leading-[1.25rem]")
        .border_color(if active { theme.ring } else { theme.input });
    let el = sides(el, first || active, first.then_some(radius), last.then_some(radius));
    control_surface(theme, el)
        .child(ch.map(|c| c.to_string()))
        .child(caret.then(|| div().class("h-4 w-px").bg(theme.foreground)))
}

/// The active slot's ring, drawn over its whole group so neighbouring slots
/// (painted later) don't cover it.
fn slot_ring(theme: &Theme, i: usize, first: bool, last: bool) -> Div {
    let w = 3.0;
    let radius = Some(px(theme.radius - 2.0 + w));
    let el = div()
        .class("absolute")
        .top(px(-w))
        .left(px(36.0 * i as f32 - w))
        .size(px(36.0 + w * 2.0))
        .border_width(px(w))
        .border_color(theme.ring.opacity(0.5));
    let mut el = el;
    let s = el.style();
    if first {
        s.corner_radii.top_left = radius;
        s.corner_radii.bottom_left = radius;
    }
    if last {
        s.corner_radii.top_right = radius;
        s.corner_radii.bottom_right = radius;
    }
    el
}

#[component]
pub fn InputOtp(
    /// Number of slots (`maxLength`).
    #[prop(default = 6)]
    length: usize,
    /// Split slots into groups with a separator between, e.g. `[3, 3]`.
    #[prop(optional)]
    groups: Vec<usize>,
    #[prop(optional)]
    pattern: OtpPattern,
    /// Controlled value.
    value: Option<Accessor<String>>,
    disabled: Option<Accessor<bool>>,
    on_change: Option<Callback<String>>,
    /// Called once every slot is filled.
    on_complete: Option<Callback<String>>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let node = NodeRef::new();
    let disabled = memo(move || disabled.map(|d| d.get()).unwrap_or(false));
    let current = signal(value.map(|v| v.get_untracked()).unwrap_or_default());
    if let Some(value) = value {
        effect(move || current.set(value.get()));
    }
    let groups = if groups.is_empty() { vec![length] } else { groups };

    let apply = move |next: String| {
        let next: String = next.chars().filter(|c| pattern.accepts(*c)).take(length).collect();
        if next == current.get_untracked() {
            return;
        }
        if value.is_none() {
            current.set(next.clone());
        }
        if let Some(f) = on_change {
            f.call(&next);
        }
        if next.chars().count() == length {
            if let Some(f) = on_complete {
                f.call(&next);
            }
        }
    };
    // The hidden field's caret may be anywhere (a click places it), so
    // typed or pasted text is always appended, like filling the next slot.
    let on_input = move |e: &InputEvent| {
        let old: Vec<char> = current.get_untracked().chars().collect();
        let new: Vec<char> = e.value.chars().collect();
        if new.len() <= old.len() {
            return apply(new.into_iter().collect());
        }
        let prefix = old.iter().zip(&new).take_while(|(a, b)| a == b).count();
        let suffix = old[prefix..].iter().rev().zip(new[prefix..].iter().rev()).take_while(|(a, b)| a == b).count();
        let inserted = &new[prefix..new.len() - suffix];
        apply(old.iter().chain(inserted).collect());
    };
    let on_key = move |e: &KeyboardEvent| {
        on_tab_key(e);
        match &*e.key {
            "Backspace" => {
                e.prevent_default();
                let mut v = current.get_untracked();
                v.pop();
                apply(v);
            }
            "Delete" | "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" | "Home" | "End" => e.prevent_default(),
            _ => {}
        }
    };
    // Rebuilt only when `disabled` changes (rsx-gpui ignores `disabled` on
    // text fields, so a disabled OTP has no field at all).
    let field = move || {
        (!disabled.get()).then(|| {
            rsx::element("input")
                .class("absolute inset-0 min-w-0 p-0 border-0 overflow-hidden")
                .opacity(0.0)
                .css("outline: none")
                .node_ref(node)
                .attr("value", current)
                .attr("autocomplete", "one-time-code")
                .on_input(on_input)
                .on_key_down(on_key)
                .on_mouse_down(move |_| pointer_pressed())
                .on_click(move |e: &ClickEvent| e.stop_propagation())
        })
    };
    let slots = move || {
        let theme = t.get();
        let focused = node.is_focused() && !disabled.get();
        let chars: Vec<char> = current.with(|v| v.chars().collect());
        let active = chars.len().min(length.saturating_sub(1));
        let mut out = Vec::new();
        let mut slot_ix = 0;
        for (group_ix, &group_len) in groups.iter().enumerate() {
            if group_ix > 0 {
                out.push(<div class="flex items-center"><Icon name={IconName::Minus} /></div>);
            }
            let group_start = slot_ix;
            let mut group = div().class("relative flex items-center");
            for i in 0..group_len {
                let ix = group_start + i;
                let is_active = focused && ix == active;
                group = group.child(slot(&theme, chars.get(ix).copied(), i == 0, i + 1 == group_len, is_active, is_active && chars.len() < length));
            }
            slot_ix += group_len;
            if focused && (group_start..slot_ix).contains(&active) {
                let i = active - group_start;
                group = group.child(slot_ring(&theme, i, i == 0, i + 1 == group_len));
            }
            out.push(group.into_element());
        }
        out
    };

    <div
        class="relative flex items-center gap-2"
        opacity={if disabled.get() { 0.5 } else { 1.0 }}
        cursor={if disabled.get() { Cursor::Default } else { Cursor::Text }}
        onMouseDown={move |e: &MouseEvent| {
            e.stop_propagation();
            if !disabled.get_untracked() {
                node.focus()
            }
        }}
        // Keep presses and clicks from a backdrop or focusable ancestor (see `Input`).
        onClick={move |e: &ClickEvent| e.stop_propagation()}
        {...&attrs}
    >
        {field}
        {slots}
    </div>
}

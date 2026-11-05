//! A control that allows the user to toggle between checked and not checked.
//!
//! ```ignore
//! // uncontrolled
//! <Checkbox label="Accept terms and conditions" />
//! // controlled
//! <Checkbox checked={accepted} onChange={move |c: &bool| accepted.set(*c)} label="Accept terms" />
//! ```
//!
//! Drawn with divs and an `Icon` rather than `<input type="checkbox">`, whose
//! look is fixed on GPUI (rsx#13). Click, or focus it and press Space / Enter.

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::label::Label;
use super::theme::{theme, ColorExt};
use super::utils::{control_border, focus_ring, is_focus_visible, on_tab_key, pointer_pressed};

#[component]
pub fn Checkbox(
    /// Controlled checked state.
    checked: Option<Accessor<bool>>,
    default_checked: bool,
    /// Show a dash instead of a check (`checked="indeterminate"`).
    indeterminate: Option<Accessor<bool>>,
    disabled: Option<Accessor<bool>>,
    label: Option<Accessor<String>>,
    on_change: Option<Callback<bool>>,
    /// Style overrides, applied to the box (like gpuicn's `Styled`).
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let node = NodeRef::new();
    let state = signal(default_checked);
    let is_checked = memo(move || checked.map(|c| c.get()).unwrap_or_else(|| state.get()));
    let indeterminate = memo(move || indeterminate.map(|i| i.get()).unwrap_or(false));
    let disabled = memo(move || disabled.map(|d| d.get()).unwrap_or(false));

    // The row keeps its identity (only bindings change), so focus survives toggling.
    <div
        ref={node}
        class="flex items-center gap-2"
        class:cursor-pointer={!disabled.get()}
        opacity={if disabled.get() { 0.5 } else { 1.0 }}
        onMouseDown={move |_| pointer_pressed()}
        onKeyDown={on_tab_key}
        onClick={move |e: &ClickEvent| {
            if disabled.get_untracked() {
                return;
            }
            e.stop_propagation();
            let next = !is_checked.get_untracked();
            state.set(next);
            if let Some(f) = on_change {
                f.call(&next);
            }
        }}
    >
        {move || {
            let theme = t.get();
            let on = is_checked.get() || indeterminate.get();
            let (bg, border) = if on {
                (theme.primary, theme.primary)
            } else if theme.is_dark() {
                (theme.input.opacity(0.3), control_border(&theme))
            } else {
                (theme.background, control_border(&theme))
            };
            <div
                class="relative flex flex-none size-4 items-center justify-center border"
                corner_radius={px(4.0)}
                bg={bg}
                border_color={border}
                text_color={theme.primary_foreground}
                {...&attrs}
            >
                {on.then(|| {
                    let icon = if indeterminate.get() { IconName::Minus } else { IconName::Check };
                    <Icon name={icon} class="size-3.5" />
                })}
                {(is_focus_visible(node) && !disabled.get()).then(|| focus_ring(&theme, 4.0))}
            </div>
        }}
        {label.map(|l| <Label text={l} />)}
    </div>
}

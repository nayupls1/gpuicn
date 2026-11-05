//! A set of checkable buttons where no more than one can be checked at a time.
//!
//! ```ignore
//! <RadioGroup default_value="comfortable" items={vec![
//!     RadioGroupItem::new("default", "Default"),
//!     RadioGroupItem::new("comfortable", "Comfortable"),
//!     RadioGroupItem::new("compact", "Compact"),
//! ]} />
//! // controlled
//! <RadioGroup value={density} onChange={move |v: &SharedString| density.set(Some(v.clone()))} items={...} />
//! ```
//!
//! Click an item, or focus one and press Space / Enter; the arrow keys move
//! the selection to the next or previous item.

use rsx::prelude::*;

use super::label::Label;
use super::theme::{theme, ColorExt};
use super::utils::{control_border, focus_ring, is_focus_visible, keyboard_focus, on_tab_key, pointer_pressed};

#[derive(Clone, Debug, PartialEq)]
pub struct RadioGroupItem {
    value: SharedString,
    label: SharedString,
    disabled: bool,
}

impl RadioGroupItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self { value: value.into(), label: label.into(), disabled: false }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[component]
pub fn RadioGroup(
    #[prop(optional)] items: Vec<RadioGroupItem>,
    /// Controlled value.
    value: Option<Accessor<Option<SharedString>>>,
    default_value: Option<SharedString>,
    /// Lay the items out in a row (default is a column).
    horizontal: bool,
    disabled: Option<Accessor<bool>>,
    on_change: Option<Callback<SharedString>>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let state = signal(default_value);
    let selected = memo(move || value.map(|v| v.get()).unwrap_or_else(|| state.get()));
    let group_disabled = memo(move || disabled.map(|d| d.get()).unwrap_or(false));

    let refs: Vec<NodeRef> = items.iter().map(|_| NodeRef::new()).collect();
    let items = StoredValue::new(items);
    let refs = StoredValue::new(refs);
    let select = move |ix: usize| {
        let value = items.with_value(|items| items[ix].value.clone());
        state.set(Some(value.clone()));
        if let Some(f) = on_change {
            f.call(&value);
        }
    };
    // Arrow keys: select and focus the next enabled item, wrapping around.
    let step = move |from: usize, forward: bool| {
        let n = items.with_value(|i| i.len());
        let mut ix = from;
        for _ in 0..n {
            ix = if forward { (ix + 1) % n } else { (ix + n - 1) % n };
            if !items.with_value(|i| i[ix].disabled) {
                select(ix);
                keyboard_focus();
                refs.with_value(|r| r[ix].focus());
                return;
            }
        }
    };

    <div
        class="flex"
        class:flex-col={!horizontal}
        class:gap-3={!horizontal}
        class:gap-4={horizontal}
        {...attrs}
    >
        {(0..items.with_value(|i| i.len())).map(|ix| {
            let (value, label, item_disabled) = items.with_value(|i| (i[ix].value.clone(), i[ix].label.clone(), i[ix].disabled));
            let node = refs.with_value(|r| r[ix]);
            let checked = memo(move || selected.with(|s| s.as_ref() == Some(&value)));
            let disabled = memo(move || group_disabled.get() || item_disabled);
            <div
                ref={node}
                class="flex items-center gap-3"
                class:cursor-pointer={!disabled.get()}
                opacity={if disabled.get() { 0.5 } else { 1.0 }}
                onMouseDown={move |_| pointer_pressed()}
                onKeyDown={move |e: &KeyboardEvent| {
                    on_tab_key(e);
                    if group_disabled.get_untracked() {
                        return;
                    }
                    match e.key.as_ref() {
                        "ArrowDown" | "ArrowRight" => step(ix, true),
                        "ArrowUp" | "ArrowLeft" => step(ix, false),
                        _ => return,
                    }
                    e.prevent_default();
                }}
                onClick={move |e: &ClickEvent| {
                    if !disabled.get_untracked() {
                        e.stop_propagation();
                        select(ix);
                    }
                }}
            >
                {move || {
                    let theme = t.get();
                    let surface = if theme.is_dark() { theme.input.opacity(0.3) } else { theme.background };
                    let dot = checked.get().then(|| <div class="size-2 rounded-full" bg={theme.primary} />);
                    let ring = (is_focus_visible(node) && !disabled.get()).then(|| focus_ring(&theme, 8.0));
                    <div
                        class="relative flex flex-none size-4 items-center justify-center rounded-full border"
                        border_color={control_border(&theme)}
                        bg={surface}
                    >
                        {dot}
                        {ring}
                    </div>
                }}
                <Label text={label.to_string()} />
            </div>
        }).collect::<Vec<_>>()}
    </div>
}

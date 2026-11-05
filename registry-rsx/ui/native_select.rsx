//! A styled select with a flat list of options — shadcn's `NativeSelect`.
//! Lighter than [`Select`](super::select::Select): no groups, full width.
//!
//! ```ignore
//! <NativeSelect options={vec![
//!     option("todo", "Todo"),
//!     option("in-progress", "In Progress"),
//!     option("done", "Done"),
//! ]} />
//! ```
//!
//! gpuicn's `.option(value, label)` / `.disabled_option(value, label)`
//! builders become the [`option`] / [`disabled_option`] helpers in the
//! `options` prop. Like gpuicn it's built on [`Select`] rather than rsx's own
//! `<select>`, so it looks and behaves like the rest of the kit.

use rsx::prelude::*;

use super::select::{Select, SelectItem};

/// One option of a [`NativeSelect`].
#[derive(Clone)]
pub struct NativeSelectOption {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
}

/// An option (`<option value>label</option>`).
pub fn option(value: impl Into<SharedString>, label: impl Into<SharedString>) -> NativeSelectOption {
    NativeSelectOption { value: value.into(), label: label.into(), disabled: false }
}

/// An option that can't be picked (`<option disabled>`).
pub fn disabled_option(value: impl Into<SharedString>, label: impl Into<SharedString>) -> NativeSelectOption {
    NativeSelectOption { value: value.into(), label: label.into(), disabled: true }
}

#[component]
pub fn NativeSelect(
    #[prop(optional)] options: Vec<NativeSelectOption>,
    placeholder: Option<Accessor<String>>,
    /// Controlled value.
    value: Option<Accessor<Option<SharedString>>>,
    default_value: Option<SharedString>,
    disabled: Option<Accessor<bool>>,
    on_change: Option<Callback<SharedString>>,
    attrs: Attributes,
) -> impl IntoElement {
    // Without a placeholder a native <select> shows its first option.
    let default = default_value.or_else(|| placeholder.is_none().then(|| options.first().map(|o| o.value.clone())).flatten());
    let items: Vec<SelectItem> = options.into_iter().map(|o| SelectItem::new(o.value, o.label).disabled(o.disabled)).collect();
    <Select
        class="w-full"
        disabled={disabled}
        items={items}
        placeholder={placeholder}
        value={value}
        default_value={default}
        on_change={move |v: &SharedString| if let Some(f) = on_change { f.call(v) }}
        {...attrs}
    />
}

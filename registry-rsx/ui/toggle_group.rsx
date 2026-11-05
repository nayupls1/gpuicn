//! A set of two-state buttons that can be toggled on or off.
//!
//! ```ignore
//! <ToggleGroup multiple outline items={vec![
//!     ToggleGroupItem::new("bold").icon(IconName::Bold),
//!     ToggleGroupItem::new("italic").icon(IconName::Italic),
//!     ToggleGroupItem::new("underline").icon(IconName::Underline),
//! ]} />
//! <ToggleGroup default_value={vec!["left".into()]} items={...} onChange={move |v: &Vec<SharedString>| ..} />
//! ```

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::theme::theme;
use super::toggle::{GroupPosition, ToggleButton, ToggleSize, ToggleVariant};

#[derive(Clone, Debug, PartialEq)]
pub struct ToggleGroupItem {
    value: SharedString,
    icon: Option<IconName>,
    label: Option<SharedString>,
    disabled: bool,
}

impl ToggleGroupItem {
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self { value: value.into(), icon: None, label: None, disabled: false }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[component]
pub fn ToggleGroup(
    #[prop(optional)] items: Vec<ToggleGroupItem>,
    /// `type="multiple"` (default is `"single"`).
    multiple: bool,
    /// Controlled pressed values.
    value: Option<Accessor<Vec<SharedString>>>,
    #[prop(optional)] default_value: Vec<SharedString>,
    /// The outline variant.
    outline: bool,
    #[prop(optional)] size: ToggleSize,
    disabled: Option<Accessor<bool>>,
    on_change: Option<Callback<Vec<SharedString>>>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let state = signal(default_value);
    let values = memo(move || value.map(|v| v.get()).unwrap_or_else(|| state.get()));
    let group_disabled = memo(move || disabled.map(|d| d.get()).unwrap_or(false));
    let variant = if outline { ToggleVariant::Outline } else { ToggleVariant::Default };
    let count = items.len();

    let buttons = items.into_iter().enumerate().map(|(ix, item)| {
        let ToggleGroupItem { value, icon, label, disabled } = item;
        let item_value = value.clone();
        let pressed = memo(move || values.with(|v| v.contains(&item_value)));
        let item_disabled = memo(move || group_disabled.get() || disabled);
        <ToggleButton
            pressed={pressed}
            disabled={item_disabled}
            variant={variant}
            size={size}
            group={GroupPosition { first: ix == 0, last: ix + 1 == count }}
            onToggle={move |_| {
                let current = values.get_untracked();
                let was_pressed = current.contains(&value);
                let next = if multiple {
                    let mut next = current;
                    if was_pressed {
                        next.retain(|v| v != &value);
                    } else {
                        next.push(value.clone());
                    }
                    next
                } else if was_pressed {
                    Vec::new()
                } else {
                    vec![value.clone()]
                };
                state.set(next.clone());
                if let Some(f) = on_change {
                    f.call(&next);
                }
            }}
        >
            {icon.map(|i| <Icon name={i} />)}
            {label.clone()}
        </ToggleButton>
    }).collect::<Vec<_>>();

    <div class="flex items-center" corner_radius={t.get().radius_md()} {...attrs}>
        {buttons}
    </div>
}

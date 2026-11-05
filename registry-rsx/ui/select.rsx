//! Displays a list of options for the user to pick from — triggered by a button.
//!
//! ```ignore
//! <Select
//!     placeholder="Select a fruit"
//!     groups={vec![SelectGroup::new("Fruits")
//!         .item(SelectItem::new("apple", "Apple"))
//!         .item(SelectItem::new("banana", "Banana"))]}
//!     onChange={|value: &SharedString| println!("{value}")}
//! />
//!
//! // Controlled:
//! let fruit = signal(Some(SharedString::from("apple")));
//! <Select value={fruit.get()} onChange={move |v: &SharedString| fruit.set(Some(v.clone()))} items={..} />
//! ```
//!
//! gpuicn's `.group(..)` / `.item(..)` builders become the `groups` and
//! `items` props; `items` (no group label) come first.

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::popover::{Popover, PopoverHandle};
use super::theme::{theme, ColorExt};
use super::utils::{
    focus_ring, invalid_ring, is_focus_visible, keyboard_focus_mode as keyboard_opened, on_tab_key, pointer_pressed, to_px, Align,
};

#[derive(Clone)]
pub struct SelectItem {
    value: SharedString,
    label: SharedString,
    icon: Option<IconName>,
    disabled: bool,
}

impl SelectItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self { value: value.into(), label: label.into(), icon: None, disabled: false }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone)]
pub struct SelectGroup {
    label: Option<SharedString>,
    items: Vec<SelectItem>,
}

impl SelectGroup {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: Some(label.into()), items: Vec::new() }
    }

    pub fn untitled() -> Self {
        Self { label: None, items: Vec::new() }
    }

    pub fn item(mut self, item: SelectItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = SelectItem>) -> Self {
        self.items.extend(items);
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectSize {
    #[default]
    Default,
    Sm,
}

/// The dropdown list of a [`Select`]; reusable by other pickers.
#[component]
pub fn SelectList(
    #[prop(optional)] groups: Vec<SelectGroup>,
    /// The value shown with a check mark (and highlighted first).
    selected: Option<SharedString>,
    /// At least this wide (usually the trigger's width); never under 128px.
    min_width: Option<Accessor<f32>>,
    /// Cap the list's height (default 384px, shadcn's `max-h-96`); it scrolls past that.
    max_height: Option<Accessor<f32>>,
    on_pick: Option<Callback<SharedString>>,
    /// The popover it lives in: the list takes focus when it opens.
    popover: Option<PopoverHandle>,
) -> impl IntoElement {
    let t = theme();
    let node = NodeRef::new();
    let items: Vec<SelectItem> = groups.iter().flat_map(|g| g.items.clone()).collect();
    let initial = items.iter().position(|i| Some(&i.value) == selected.as_ref());
    let highlighted = signal(initial);
    // Take focus so the arrow keys reach the list (the popover's panel asks
    // too, but the innermost request wins).
    match popover {
        Some(p) => p.focus_on_open(node),
        None => node.focus(),
    }
    // One handle per item, to keep the highlighted one scrolled into view;
    // the list opens scrolled to the selected item.
    let row_refs: Vec<NodeRef> = items.iter().map(|_| NodeRef::new()).collect();
    if let Some(row) = initial.and_then(|i| row_refs.get(i)) {
        row.scroll_into_view();
    }
    let row_refs = StoredValue::new(row_refs);
    let pick = move |value: &SharedString| {
        if let Some(f) = on_pick {
            f.call(value)
        }
    };

    let groups = StoredValue::new(groups);
    let selected_value = StoredValue::new(selected);
    let rows = move || {
        let mut rows: Vec<Element> = Vec::new();
        let mut ix = 0;
        let selected = selected_value.get_value();
        groups.with_value(|groups| for (group_ix, group) in groups.iter().enumerate() {
            if group_ix > 0 {
                rows.push(<div class="h-px -mx-1 my-1" bg={t.get().border} />);
            }
            if let Some(label) = group.label.clone() {
                rows.push(<div class="px-2 py-1.5 text-xs leading-[1rem]" text_color={t.get().muted_foreground}>{label}</div>);
            }
            for item in &group.items {
                let item_ix = ix;
                ix += 1;
                let is_selected = Some(&item.value) == selected.as_ref();
                let (value, disabled, icon, label) = (item.value.clone(), item.disabled, item.icon, item.label.clone());
                let row_ref = row_refs.with_value(|r| r[item_ix]);
                let row = move || {
                    let theme = t.get();
                    let lit = highlighted.get() == Some(item_ix) && !disabled;
                    let value = value.clone();
                    <div
                        ref={row_ref}
                        class="relative flex items-center gap-2 py-1.5 pl-2 pr-8 text-sm leading-[1.25rem]"
                        class:opacity-50={disabled}
                        corner_radius={theme.radius_sm()}
                        bg={if lit { theme.accent } else { Color::TRANSPARENT }}
                        text_color={if lit { theme.accent_foreground } else { theme.popover_foreground }}
                        onMouseMove={move |_| {
                            if !disabled && highlighted.get_untracked() != Some(item_ix) {
                                highlighted.set(Some(item_ix));
                            }
                        }}
                        onClick={move |_| if !disabled { pick(&value) }}
                    >
                        {icon.map(|icon| icon.element().text_color(theme.muted_foreground))}
                        {label.clone()}
                        {is_selected.then(|| <div class="absolute right-2 flex"><Icon name={IconName::Check} class="size-3.5" /></div>)}
                    </div>
                };
                rows.push(row.into_element());
            }
        });
        rows
    };

    let enabled: Vec<usize> = (0..items.len()).filter(|i| !items[*i].disabled).collect();
    let (items, enabled) = (StoredValue::new(items), StoredValue::new(enabled));
    let on_key = move |e: &KeyboardEvent| {
        let enabled = enabled.get_value();
        let current = highlighted.get_untracked();
        let pos = current.and_then(|c| enabled.iter().position(|x| *x == c));
        let next = match e.key.as_str() {
            "ArrowDown" => pos.map(|p| (p + 1) % enabled.len()).or(Some(0)),
            "ArrowUp" => pos.map(|p| (p + enabled.len() - 1) % enabled.len()).or(Some(enabled.len().saturating_sub(1))),
            "Enter" | " " => {
                if let Some(value) = current.and_then(|c| items.with_value(|i| i.get(c).map(|i| i.value.clone()))) {
                    e.stop_propagation();
                    e.prevent_default();
                    pick(&value);
                }
                return;
            }
            _ => return,
        };
        if let Some(next) = next.and_then(|p| enabled.get(p).copied()) {
            e.stop_propagation();
            highlighted.set(Some(next));
            row_refs.with_value(|r| r[next].scroll_into_view());
        }
    };

    let size = move |el: rsx::Div| {
        let min = min_width.map(|w| w.get()).unwrap_or(0.0).max(128.0);
        let max = max_height.map(|h| h.get()).unwrap_or(384.0).min(384.0);
        el.min_w(px(min)).max_h(px(max))
    };
    move || {
        let theme = t.get();
        <div
            ref={node}
            class="flex flex-col overflow-y-scroll p-1 border"
            class:shadow-md={theme.shadows}
            corner_radius={theme.radius_md()}
            border_color={theme.border}
            bg={theme.popover}
            text_color={theme.popover_foreground}
            bind={size}
            onKeyDown={on_key}
        >
            {rows}
        </div>
    }
}

#[component]
pub fn Select(
    /// Labelled groups of items (gpuicn's `.group(..)`).
    #[prop(optional)]
    groups: Vec<SelectGroup>,
    /// Items without a group label (gpuicn's `.item(..)` / `.items(..)`), listed first.
    #[prop(optional)]
    items: Vec<SelectItem>,
    /// Shown while nothing is selected (default "Select…").
    placeholder: Option<Accessor<String>>,
    /// Controlled value.
    value: Option<Accessor<Option<SharedString>>>,
    default_value: Option<SharedString>,
    #[prop(optional)] size: SelectSize,
    disabled: Option<Accessor<bool>>,
    invalid: Option<Accessor<bool>>,
    on_change: Option<Callback<SharedString>>,
    attrs: Attributes,
) -> impl IntoElement {
    let mut all = Vec::new();
    if !items.is_empty() {
        all.push(SelectGroup::untitled().items(items));
    }
    all.extend(groups);
    let groups = StoredValue::new(all);
    let state = signal(default_value);
    let selected = move || match value {
        Some(v) => v.get(),
        None => state.get(),
    };
    let selected_label = move || {
        let value = selected()?;
        groups.with_value(|g| g.iter().flat_map(|g| &g.items).find(|i| i.value == value).map(|i| i.label.clone()))
    };
    let flag = |a: Option<Accessor<bool>>| move || a.map(|a| a.get()).unwrap_or(false);
    let (disabled, invalid) = (flag(disabled), flag(invalid));
    let t = theme();
    let node = NodeRef::new();
    let hovered = signal(false);
    let popover = PopoverHandle::new();
    // A width given to the select (`w-full`) also goes to the popover's wrapper.
    let wrapper = {
        let probe = rsx::element("div").spread(attrs.clone());
        match probe.style.size.width {
            Some(rsx::Length::Fraction(_)) => Some("w-full".to_string()),
            _ => None,
        }
    };
    let fill = wrapper.is_some();
    let attrs = StoredValue::new(attrs);

    let trigger = move || {
        let theme = t.get();
        let (disabled, invalid) = (disabled(), invalid());
        // While the list is open the trigger keeps its focus styling.
        let focused = is_focus_visible(node) || is_focus_visible(popover.trigger_ref());
        let focus_visible = !disabled && (focused || (popover.is_open() && keyboard_opened()));
        let radius = to_px(theme.radius_md());
        let hover_bg = if theme.is_dark() { theme.input.opacity(0.5) } else { theme.accent.opacity(0.5) };
        let surface = if theme.is_dark() { theme.input.opacity(0.3) } else { theme.background };
        let border = if invalid {
            theme.destructive
        } else if focus_visible {
            theme.ring
        } else {
            theme.input
        };
        <div
            ref={node}
            class="relative flex items-center justify-between gap-2 px-3 py-2 border text-sm leading-[1.25rem] whitespace-nowrap"
            class:flex-1={fill}
            class:cursor-pointer={!disabled}
            class:cursor-not-allowed={disabled}
            class:opacity-50={disabled}
            // `shadow-xs`: light mode only, like gpuicn's `control_surface`.
            class:shadow-xs={theme.shadows && !theme.is_dark()}
            h={rsx::rems(if size == SelectSize::Sm { 2.0 } else { 2.25 })}
            corner_radius={theme.radius_md()}
            border_color={border}
            bg={if hovered.get() && !disabled { hover_bg } else { surface }}
            onHover={move |h: &bool| hovered.set(*h)}
            onMouseDown={move |_| if !disabled { pointer_pressed() }}
            onKeyDown={move |e: &KeyboardEvent| {
                on_tab_key(e);
                if disabled {
                    e.stop_propagation();
                }
            }}
            onClick={move |e: &ClickEvent| if disabled { e.stop_propagation() }}
            {...attrs.with_value(|a| a.clone())}
        >
            {match selected_label() {
                Some(label) => <div text_color={theme.foreground}>{label}</div>,
                None => <div text_color={theme.muted_foreground}>{placeholder.map(|p| p.get()).unwrap_or_else(|| "Select…".into())}</div>,
            }}
            <Icon name={IconName::ChevronDown} class="opacity-50" />
            {(invalid && !disabled).then(|| invalid_ring(&theme, radius))}
            {(focus_visible && !invalid).then(|| focus_ring(&theme, radius))}
        </div>
    };

    <Popover
        handle={popover}
        unstyled
        align={Align::Start}
        wrapper_class={wrapper}
        trigger={trigger}
        content={move |popover: PopoverHandle| {
            let on_pick = move |value: &SharedString| {
                popover.close();
                state.set(Some(value.clone()));
                if let Some(f) = on_change {
                    f.call(value);
                }
            };
            <SelectList
                groups={groups.get_value()}
                selected={untrack(selected)}
                min_width={move || popover.trigger_bounds().size.width}
                max_height={move || popover.available_height().unwrap_or(384.0)}
                on_pick={on_pick}
                popover={popover}
            />
        }}
    />
}

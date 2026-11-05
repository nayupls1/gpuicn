//! Autocomplete input and command palette with a list of suggestions —
//! shadcn's Popover + Command recipe.
//!
//! ```ignore
//! <Combobox
//!     placeholder="Select framework..."
//!     search_placeholder="Search framework..."
//!     empty="No framework found."
//!     items={vec![("next.js".into(), "Next.js".into()), ("sveltekit".into(), "SvelteKit".into())]}
//!     onChange={|value: &Option<SharedString>| println!("{value:?}")}
//! />
//!
//! // Controlled:
//! let framework = signal(None::<SharedString>);
//! <Combobox value={framework} onChange={move |v: &Option<SharedString>| framework.set(v.clone())} items={..} />
//! ```
//!
//! gpuicn's `.item(value, label)` builder becomes the `items` prop
//! (`(value, label)` pairs, in order).

use rsx::prelude::*;
use rsx::Div;

use super::button::Button;
use super::command::{Command, CommandGroup, CommandItem};
use super::icon::{Icon, IconName};
use super::popover::{outer_layout_class, Popover, PopoverHandle};
use super::theme::theme;
use super::utils::Align;

#[component]
pub fn Combobox(
    /// `(value, label)` pairs (gpuicn's `.item(value, label)`).
    #[prop(optional)]
    items: Vec<(SharedString, SharedString)>,
    /// Shown on the trigger while nothing is selected (default "Select…").
    placeholder: Option<Accessor<String>>,
    /// The search field's placeholder (default "Search…").
    search_placeholder: Option<Accessor<String>>,
    /// Shown when nothing matches (default "No results found.").
    empty: Option<Accessor<String>>,
    /// Controlled value (`None`: nothing selected).
    value: Option<Accessor<Option<SharedString>>>,
    default_value: Option<SharedString>,
    /// Selecting the current value again clears it (`None`).
    on_change: Option<Callback<Option<SharedString>>>,
    /// Styles for the trigger button; layout (width, flex) also sizes the popover's wrapper.
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let state = signal(default_value);
    let selected = move || match value {
        Some(v) => v.get(),
        None => state.get(),
    };
    let items = StoredValue::new(items);
    let label = move || {
        let value = selected()?;
        items.with_value(|items| items.iter().find(|(v, _)| *v == value).map(|(_, l)| l.clone()))
    };
    let placeholder = placeholder.unwrap_or_else(|| Accessor::value("Select…".to_string()));
    let search_placeholder = search_placeholder.unwrap_or_else(|| Accessor::value("Search…".to_string()));
    let empty = empty.unwrap_or_else(|| Accessor::value("No results found.".to_string()));
    let wrapper = outer_layout_class(&attrs);
    let attrs = StoredValue::new(attrs);
    let popover = PopoverHandle::new();

    // The button is built once; only its label is a reactive hole.
    let trigger = move || {
        <Button outline class="w-[200px] justify-between" {...attrs.with_value(|a| a.clone())}>
            <div>{move || label().map(|l| l.to_string()).unwrap_or_else(|| placeholder.get())}</div>
            <Icon name={IconName::ChevronsUpDown} class="opacity-50" />
        </Button>
    };

    <Popover
        handle={popover}
        wrapper_class={wrapper}
        unstyled
        align={Align::Start}
        trigger={move || trigger.into_element()}
        content={move |popover: PopoverHandle| {
            let current = untrack(selected);
            let group = CommandGroup::untitled().items(items.with_value(|items| {
                items
                    .iter()
                    .map(|(value, label)| CommandItem::new(label.clone()).value(value.clone()).checked(Some(value) == current.as_ref()))
                    .collect::<Vec<_>>()
            }));
            // The search field's `autofocus` wins over the popover panel's
            // focus request (the innermost one does).
            let current = StoredValue::new(current);
            let select = move |value: &SharedString| {
                let next = current.with_value(|c| (c.as_ref() != Some(value)).then(|| value.clone()));
                popover.close();
                state.set(next.clone());
                if let Some(f) = on_change {
                    f.call(&next);
                }
            };
            let size = move |el: Div| {
                let theme = t.get();
                let el = el
                    .w(px(popover.trigger_bounds().size.width.max(200.0)))
                    .corner_radius(theme.radius_md())
                    .border_color(theme.border);
                if theme.shadows { el.class("shadow-md") } else { el }
            };
            <div class="flex flex-col border" bind={size}>
                <Command
                    placeholder={search_placeholder}
                    empty={empty}
                    groups={vec![group]}
                    onSelect={select}
                />
            </div>
        }}
    />
}

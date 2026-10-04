//! Autocomplete input and command palette with a list of suggestions —
//! shadcn's Popover + Command recipe.
//!
//! ```ignore
//! Combobox::new("framework")
//!     .placeholder("Select framework...")
//!     .search_placeholder("Search framework...")
//!     .empty("No framework found.")
//!     .item("next.js", "Next.js")
//!     .item("sveltekit", "SvelteKit")
//!     .on_change(|value, _, _| println!("{value:?}"))
//! ```

use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window, div, prelude::*, px};
use std::rc::Rc;

use super::button::Button;
use super::command::{Command, CommandGroup, CommandItem};
use super::icon::IconName;
use super::popover::Popover;
use super::theme::ActiveTheme;
use super::utils::{Align, StyledExt};

#[derive(IntoElement)]
pub struct Combobox {
    id: ElementId,
    items: Vec<(SharedString, SharedString)>,
    placeholder: SharedString,
    search_placeholder: SharedString,
    empty: SharedString,
    value: Option<Option<SharedString>>,
    default_value: Option<SharedString>,
    on_change: Option<Rc<dyn Fn(&Option<SharedString>, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Combobox);

impl Combobox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            placeholder: "Select…".into(),
            search_placeholder: "Search…".into(),
            empty: "No results found.".into(),
            value: None,
            default_value: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.items.push((value.into(), label.into()));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn search_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.search_placeholder = placeholder.into();
        self
    }

    pub fn empty(mut self, empty: impl Into<SharedString>) -> Self {
        self.empty = empty.into();
        self
    }

    pub fn value(mut self, value: Option<impl Into<SharedString>>) -> Self {
        self.value = Some(value.map(Into::into));
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Selecting the current value again clears it (`None`).
    pub fn on_change(mut self, handler: impl Fn(&Option<SharedString>, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Combobox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_value.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let selected = self.value.clone().unwrap_or_else(|| state.read(cx).clone());
        let label = selected
            .as_ref()
            .and_then(|v| self.items.iter().find(|(value, _)| value == v))
            .map(|(_, label)| label.clone());
        let muted = cx.theme().muted_foreground;

        let trigger = Button::new("combobox-trigger")
            .outline()
            .w(px(200.))
            .justify_between()
            .child(match label {
                Some(label) => div().child(label),
                None => div().text_color(muted).child(self.placeholder.clone()),
            })
            .icon_end(super::icon::Icon::new(IconName::ChevronsUpDown).opacity(0.5))
            .refine_style(&self.style);

        let items = Rc::new(self.items);
        let (search_placeholder, empty, on_change) = (self.search_placeholder, self.empty, self.on_change);
        Popover::new(self.id)
            .unstyled()
            .align(Align::Start)
            .trigger(trigger)
            .content_with_handle(move |popover, _, cx| {
                let theme = cx.theme();
                let width = popover.trigger_bounds(cx).size.width.max(px(200.));
                let popover = popover.clone();
                let state = state.clone();
                let on_change = on_change.clone();
                let selected = selected.clone();
                let group = CommandGroup::untitled().items(items.iter().map(|(value, label)| {
                    CommandItem::new(label.clone()).value(value.clone()).checked(Some(value) == selected.as_ref())
                }));
                let current = selected.clone();
                div()
                    .w(width)
                    .rounded(theme.radius_md())
                    .border_1()
                    .border_color(theme.border)
                    .when(theme.shadows, |this| this.shadow_md())
                    .child(
                        Command::new("combobox-command")
                            .placeholder(search_placeholder.clone())
                            .empty(empty.clone())
                            .group(group)
                            .on_select(move |value, window, cx| {
                                let next = (current.as_ref() != Some(value)).then(|| value.clone());
                                popover.close(window, cx);
                                state.update(cx, |s, cx| {
                                    *s = next.clone();
                                    cx.notify();
                                });
                                if let Some(handler) = &on_change {
                                    handler(&next, window, cx);
                                }
                            }),
                    )
            })
    }
}

//! Displays a list of options for the user to pick from — triggered by a button.
//!
//! ```ignore
//! Select::new("fruit")
//!     .placeholder("Select a fruit")
//!     .group(SelectGroup::new("Fruits")
//!         .item(SelectItem::new("apple", "Apple"))
//!         .item(SelectItem::new("banana", "Banana")))
//!     .on_change(|value, _, _| println!("{value}"))
//! ```

use gpui::{
    App, ElementId, FocusHandle, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::popover::{Popover, PopoverHandle};
use super::theme::ActiveTheme;
use super::utils::{Align, StyledExt, ring_shadow};

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

type ChangeHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    groups: Vec<SelectGroup>,
    placeholder: SharedString,
    value: Option<Option<SharedString>>,
    default_value: Option<SharedString>,
    size: SelectSize,
    disabled: bool,
    invalid: bool,
    on_change: Option<ChangeHandler>,
    style: StyleRefinement,
}

crate::impl_styled!(Select);

impl Select {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            groups: Vec::new(),
            placeholder: "Select…".into(),
            value: None,
            default_value: None,
            size: SelectSize::Default,
            disabled: false,
            invalid: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Items without a group label.
    pub fn item(mut self, item: SelectItem) -> Self {
        match self.groups.last_mut() {
            Some(group) if group.label.is_none() => group.items.push(item),
            _ => self.groups.push(SelectGroup::untitled().item(item)),
        }
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = SelectItem>) -> Self {
        for item in items {
            self = self.item(item);
        }
        self
    }

    pub fn group(mut self, group: SelectGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Controlled value.
    pub fn value(mut self, value: Option<impl Into<SharedString>>) -> Self {
        self.value = Some(value.map(Into::into));
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

struct ListState {
    focus_handle: FocusHandle,
    highlighted: Option<usize>,
    focused_once: bool,
}

/// The dropdown list of a [`Select`]; reusable by other pickers.
#[derive(IntoElement)]
pub struct SelectList {
    groups: Rc<Vec<SelectGroup>>,
    selected: Option<SharedString>,
    min_width: gpui::Pixels,
    on_pick: Rc<dyn Fn(&SharedString, &mut Window, &mut App)>,
}

impl SelectList {
    pub fn new(
        groups: Rc<Vec<SelectGroup>>,
        selected: Option<SharedString>,
        min_width: gpui::Pixels,
        on_pick: Rc<dyn Fn(&SharedString, &mut Window, &mut App)>,
    ) -> Self {
        Self { groups, selected, min_width, on_pick }
    }
}

impl RenderOnce for SelectList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let items: Vec<SelectItem> = self.groups.iter().flat_map(|g| g.items.clone()).collect();
        let initial = items.iter().position(|i| Some(&i.value) == self.selected.as_ref());
        let state = window.use_keyed_state("select-list", cx, move |_, cx| ListState {
            focus_handle: cx.focus_handle(),
            highlighted: initial,
            focused_once: false,
        });
        let (focus_handle, highlighted) = {
            let s = state.read(cx);
            (s.focus_handle.clone(), s.highlighted)
        };
        if !state.read(cx).focused_once {
            window.focus(&focus_handle);
            state.update(cx, |s, _| s.focused_once = true);
        }

        let theme = cx.theme();
        let (accent, accent_fg, muted_fg) = (theme.accent, theme.accent_foreground, theme.muted_foreground);
        let radius = theme.radius_sm();
        let mut rows = Vec::new();
        let mut ix = 0;
        for (group_ix, group) in self.groups.iter().enumerate() {
            if group_ix > 0 {
                rows.push(div().h_px().mx_neg_1().my_1().bg(theme.border).into_any_element());
            }
            if let Some(label) = &group.label {
                rows.push(div().px_2().py_1p5().text_xs().text_color(muted_fg).child(label.clone()).into_any_element());
            }
            for item in &group.items {
                let item_ix = ix;
                ix += 1;
                let is_selected = Some(&item.value) == self.selected.as_ref();
                let is_highlighted = highlighted == Some(item_ix);
                let state = state.clone();
                let on_pick = self.on_pick.clone();
                let value = item.value.clone();
                let disabled = item.disabled;
                rows.push(
                    div()
                        .id(("select-item", item_ix))
                        .relative()
                        .flex()
                        .items_center()
                        .gap_2()
                        .py_1p5()
                        .pl_2()
                        .pr_8()
                        .rounded(radius)
                        .text_sm()
                        .when(disabled, |this| this.opacity(0.5))
                        .when(is_highlighted && !disabled, |this| this.bg(accent).text_color(accent_fg))
                        .children(item.icon.map(|icon| Icon::new(icon).text_color(muted_fg)))
                        .child(item.label.clone())
                        .when(is_selected, |this| {
                            this.child(div().absolute().right_2().child(Icon::new(IconName::Check).size_3p5()))
                        })
                        .when(!disabled, |this| {
                            this.on_mouse_move(move |_, _, cx| {
                                state.update(cx, |s, cx| {
                                    if s.highlighted != Some(item_ix) {
                                        s.highlighted = Some(item_ix);
                                        cx.notify();
                                    }
                                })
                            })
                            .on_click(move |_, window, cx| on_pick(&value, window, cx))
                        })
                        .into_any_element(),
                );
            }
        }

        let enabled: Vec<usize> = (0..items.len()).filter(|i| !items[*i].disabled).collect();
        let key_state = state.clone();
        let on_pick = self.on_pick.clone();

        div()
            .id("select-content")
            .track_focus(&focus_handle)
            .occlude()
            .flex()
            .flex_col()
            .min_w(self.min_width.max(px(128.)))
            .max_h(px(384.))
            .overflow_y_scroll()
            .p_1()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.border)
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .when(theme.shadows, |this| this.shadow_md())
            .on_key_down(move |event, window, cx| {
                let current = key_state.read(cx).highlighted;
                let pos = current.and_then(|c| enabled.iter().position(|e| *e == c));
                let next = match event.keystroke.key.as_str() {
                    "down" => pos.map(|p| (p + 1) % enabled.len()).or(Some(0)),
                    "up" => pos.map(|p| (p + enabled.len() - 1) % enabled.len()).or(Some(enabled.len().saturating_sub(1))),
                    "enter" | "space" => {
                        if let Some(item) = current.and_then(|c| items.get(c)) {
                            cx.stop_propagation();
                            on_pick(&item.value, window, cx);
                        }
                        return;
                    }
                    _ => return,
                };
                if let Some(next) = next.and_then(|p| enabled.get(p).copied()) {
                    cx.stop_propagation();
                    key_state.update(cx, |s, cx| {
                        s.highlighted = Some(next);
                        cx.notify();
                    });
                }
            })
            .children(rows)
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_value.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let selected = self.value.clone().unwrap_or_else(|| state.read(cx).clone());
        let selected_label = selected.as_ref().and_then(|value| {
            self.groups.iter().flat_map(|g| &g.items).find(|i| &i.value == value).map(|i| i.label.clone())
        });
        let theme = cx.theme();
        let (disabled, invalid) = (self.disabled, self.invalid);

        let trigger = div()
            .id("select-trigger")
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .py_2()
            .map(|this| match self.size {
                SelectSize::Default => this.h_9(),
                SelectSize::Sm => this.h_8(),
            })
            .rounded(theme.radius_md())
            .border_1()
            .border_color(if invalid { theme.destructive } else { theme.input })
            .when(theme.is_dark(), |this| this.bg(theme.input.opacity(0.3)))
            .when(theme.shadows, |this| this.shadow_xs())
            .text_sm()
            .whitespace_nowrap()
            .when(invalid, |this| this.shadow(vec![ring_shadow(theme.destructive.opacity(0.2), px(3.))]))
            .when(disabled, |this| this.opacity(0.5).cursor_not_allowed())
            .when(!disabled, |this| this.cursor_pointer())
            .refine_style(&self.style)
            .child(match selected_label {
                Some(label) => div().min_w_0().truncate().text_color(theme.foreground).child(label),
                None => div().min_w_0().truncate().text_color(theme.muted_foreground).child(self.placeholder.clone()),
            })
            .child(Icon::new(IconName::ChevronDown).opacity(0.5));

        if disabled {
            return trigger.into_any_element();
        }

        let groups = Rc::new(self.groups);
        let on_change = self.on_change;
        Popover::new(self.id)
            .unstyled()
            .align(Align::Start)
            .trigger(trigger)
            .content_with_handle(move |popover: &PopoverHandle, _, cx| {
                let popover_handle = popover.clone();
                let state = state.clone();
                let on_change = on_change.clone();
                let on_pick: Rc<dyn Fn(&SharedString, &mut Window, &mut App)> = Rc::new(move |value, window, cx| {
                    popover_handle.close(window, cx);
                    state.update(cx, |s, cx| {
                        *s = Some(value.clone());
                        cx.notify();
                    });
                    if let Some(handler) = &on_change {
                        handler(value, window, cx);
                    }
                });
                SelectList::new(groups.clone(), selected.clone(), popover.trigger_bounds(cx).size.width, on_pick)
            })
            .into_any_element()
    }
}

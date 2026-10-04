//! A set of checkable buttons where no more than one can be checked at a time.
//!
//! ```ignore
//! RadioGroup::new("density")
//!     .default_value("comfortable")
//!     .item(RadioGroupItem::new("default", "Default"))
//!     .item(RadioGroupItem::new("comfortable", "Comfortable"))
//!     .item(RadioGroupItem::new("compact", "Compact"))
//! ```

use gpui::{
    App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window, div,
    prelude::*,
};
use std::rc::Rc;

use super::label::Label;
use super::theme::ActiveTheme;
use super::utils::{Orientation, StyledExt};

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

#[derive(IntoElement)]
pub struct RadioGroup {
    id: ElementId,
    items: Vec<RadioGroupItem>,
    value: Option<Option<SharedString>>,
    default_value: Option<SharedString>,
    orientation: Orientation,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(RadioGroup);

impl RadioGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            value: None,
            default_value: None,
            orientation: Orientation::Vertical,
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: RadioGroupItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = RadioGroupItem>) -> Self {
        self.items.extend(items);
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

    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_value.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let selected = self.value.clone().unwrap_or_else(|| state.read(cx).clone());
        let theme = cx.theme();
        let (primary, input, dark) = (theme.primary, theme.input, theme.is_dark());
        let shadows = theme.shadows;

        div()
            .id(self.id)
            .flex()
            .map(|this| match self.orientation {
                Orientation::Vertical => this.flex_col().gap_3(),
                Orientation::Horizontal => this.gap_4(),
            })
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map(|(ix, item)| {
                let checked = selected.as_ref() == Some(&item.value);
                let disabled = self.disabled || item.disabled;
                let state = state.clone();
                let on_change = self.on_change.clone();
                let value = item.value.clone();
                div()
                    .id(ix)
                    .flex()
                    .items_center()
                    .gap_3()
                    .when(disabled, |this| this.opacity(0.5))
                    .when(!disabled, |this| this.cursor_pointer())
                    .child(
                        div()
                            .flex()
                            .flex_none()
                            .size_4()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .border_1()
                            .border_color(input)
                            .when(dark, |this| this.bg(input.opacity(0.3)))
                            .when(shadows, |this| this.shadow_xs())
                            .when(checked, |this| this.child(div().size_2().rounded_full().bg(primary))),
                    )
                    .child(Label::new(item.label))
                    .when(!disabled, |this| {
                        this.on_click(move |_, window, cx| {
                            state.update(cx, |s, cx| {
                                *s = Some(value.clone());
                                cx.notify();
                            });
                            if let Some(handler) = &on_change {
                                handler(&value, window, cx);
                            }
                        })
                    })
            }))
    }
}

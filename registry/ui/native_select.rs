//! A styled select with a flat list of options — shadcn's `NativeSelect`.
//! Lighter than [`Select`](super::select::Select): no groups, full width.
//!
//! ```ignore
//! NativeSelect::new("status")
//!     .option("todo", "Todo")
//!     .option("in-progress", "In Progress")
//!     .option("done", "Done")
//! ```

use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window, prelude::*};
use std::rc::Rc;

use super::select::{Select, SelectItem};
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct NativeSelect {
    id: ElementId,
    options: Vec<(SharedString, SharedString, bool)>,
    placeholder: Option<SharedString>,
    value: Option<Option<SharedString>>,
    default_value: Option<SharedString>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(NativeSelect);

impl NativeSelect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            options: Vec::new(),
            placeholder: None,
            value: None,
            default_value: None,
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn option(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.options.push((value.into(), label.into(), false));
        self
    }

    pub fn disabled_option(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.options.push((value.into(), label.into(), true));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for NativeSelect {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Without a placeholder a native <select> shows its first option.
        let default = self.default_value.or_else(|| {
            self.placeholder.is_none().then(|| self.options.first().map(|o| o.0.clone())).flatten()
        });
        let mut select = Select::new(self.id)
            .w_full()
            .disabled(self.disabled)
            .items(self.options.into_iter().map(|(value, label, disabled)| SelectItem::new(value, label).disabled(disabled)))
            .refine_style(&self.style);
        if let Some(placeholder) = self.placeholder {
            select = select.placeholder(placeholder);
        }
        if let Some(value) = self.value {
            select = select.value(value);
        }
        if let Some(default) = default {
            select = select.default_value(default);
        }
        if let Some(handler) = self.on_change {
            select = select.on_change(move |value, window, cx| handler(value, window, cx));
        }
        select
    }
}

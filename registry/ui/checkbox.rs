//! A control that allows the user to toggle between checked and not checked.
//!
//! ```ignore
//! // uncontrolled
//! Checkbox::new("terms").label("Accept terms and conditions")
//! // controlled
//! Checkbox::new("terms")
//!     .checked(self.accepted)
//!     .on_change(cx.listener(|this, checked, _, cx| { this.accepted = *checked; cx.notify() }))
//! ```

use gpui::{
    App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window, div,
    prelude::*,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::label::Label;
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    checked: Option<bool>,
    default_checked: bool,
    indeterminate: bool,
    disabled: bool,
    label: Option<SharedString>,
    on_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Checkbox);

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: None,
            default_checked: false,
            indeterminate: false,
            disabled: false,
            label: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    /// Controlled checked state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    /// Show a dash instead of a check (`checked="indeterminate"`).
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_checked;
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let checked = self.checked.unwrap_or(*state.read(cx));
        let on = checked || self.indeterminate;
        let theme = cx.theme();

        let box_ = div()
            .flex()
            .flex_none()
            .size_4()
            .items_center()
            .justify_center()
            .rounded(gpui::px(4.))
            .border_1()
            .map(|this| {
                if on {
                    this.bg(theme.primary).border_color(theme.primary).text_color(theme.primary_foreground)
                } else {
                    this.border_color(theme.input)
                        .when(theme.is_dark(), |this| this.bg(theme.input.opacity(0.3)))
                }
            })
            .when(theme.shadows, |this| this.shadow_xs())
            .refine_style(&self.style)
            .when(on, |this| {
                this.child(
                    Icon::new(if self.indeterminate { IconName::Minus } else { IconName::Check }).size_3p5(),
                )
            });

        let disabled = self.disabled;
        let on_change = self.on_change;
        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .when(disabled, |this| this.opacity(0.5))
            .when(!disabled, |this| this.cursor_pointer())
            .child(box_)
            .children(self.label.map(Label::new))
            .when(!disabled, |this| {
                this.on_click(move |_, window, cx| {
                    let next = !checked;
                    state.update(cx, |s, cx| {
                        *s = next;
                        cx.notify();
                    });
                    if let Some(handler) = &on_change {
                        handler(&next, window, cx);
                    }
                })
            })
    }
}

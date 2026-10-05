//! Combine labels, controls, and help text to compose accessible form fields.
//!
//! ```ignore
//! FieldSet::new()
//!     .legend("Payment Method")
//!     .description("All transactions are secure and encrypted")
//!     .child(
//!         FieldGroup::new()
//!             .child(Field::new().label("Name on Card").child(Input::new("name")))
//!             .child(
//!                 Field::new()
//!                     .label("Card Number")
//!                     .description("Enter your 16-digit card number")
//!                     .error(error_message)
//!                     .child(Input::new("card")),
//!             ),
//!     )
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};

use super::label::Label;
use super::separator::Separator;
use super::theme::ActiveTheme;
use super::utils::{Orientation, StyledExt};

#[derive(IntoElement)]
pub struct Field {
    orientation: Orientation,
    label: Option<SharedString>,
    description: Option<SharedString>,
    error: Option<SharedString>,
    disabled: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Field, FieldGroup, FieldSet);

impl Field {
    pub fn new() -> Self {
        Self {
            orientation: Orientation::Vertical,
            label: None,
            description: None,
            error: None,
            disabled: false,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    /// Label beside the control (checkboxes, switches).
    pub fn horizontal(mut self) -> Self {
        self.orientation = Orientation::Horizontal;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Validation message; also tints the label.
    pub fn error(mut self, error: Option<impl Into<SharedString>>) -> Self {
        self.error = error.map(Into::into);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Field {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let invalid = self.error.is_some();
        let label = self.label.map(|label| {
            Label::new(label)
                .disabled(self.disabled)
                .when(invalid, |this| this.text_color(theme.destructive))
        });
        let description = self.description.map(|description| {
            div().text_sm().text_color(theme.muted_foreground).child(description)
        });
        let error = self
            .error
            .map(|error| div().text_sm().text_color(theme.destructive).child(error));

        match self.orientation {
            Orientation::Vertical => div()
                .flex()
                .flex_col()
                .w_full()
                .gap_3()
                .refine_style(&self.style)
                .children(label)
                .children(self.children)
                .children(description)
                .children(error),
            Orientation::Horizontal => div()
                .flex()
                .items_start()
                .w_full()
                .gap_3()
                .refine_style(&self.style)
                .children(self.children)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .gap_1p5()
                        .children(label)
                        .children(description)
                        .children(error),
                ),
        }
    }
}

/// Stacks fields with consistent spacing.
#[derive(IntoElement)]
pub struct FieldGroup {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl FieldGroup {
    pub fn new() -> Self {
        Self { children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for FieldGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().w_full().gap_7().refine_style(&self.style).children(self.children)
    }
}

/// A group of related fields with a legend (`<fieldset>`).
#[derive(IntoElement)]
pub struct FieldSet {
    legend: Option<SharedString>,
    description: Option<SharedString>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl FieldSet {
    pub fn new() -> Self {
        Self { legend: None, description: None, children: Vec::new(), style: StyleRefinement::default() }
    }

    pub fn legend(mut self, legend: impl Into<SharedString>) -> Self {
        self.legend = Some(legend.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl RenderOnce for FieldSet {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .refine_style(&self.style)
            .when(self.legend.is_some() || self.description.is_some(), |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .children(self.legend.map(|legend| {
                            div().text_base().font_weight(FontWeight::MEDIUM).child(legend)
                        }))
                        .children(self.description.map(|description| {
                            div().text_sm().text_color(theme.muted_foreground).child(description)
                        })),
                )
            })
            .children(self.children)
    }
}

/// A separator between fields, optionally with inline text ("Or continue with").
#[derive(IntoElement)]
pub struct FieldSeparator {
    text: Option<SharedString>,
}

impl FieldSeparator {
    pub fn new() -> Self {
        Self { text: None }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }
}

impl RenderOnce for FieldSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        match self.text {
            None => div().py_2().child(Separator::horizontal()),
            Some(text) => div()
                .flex()
                .items_center()
                .gap_2()
                .text_sm()
                .text_color(theme.muted_foreground)
                .child(Separator::horizontal().flex_1())
                .child(text)
                .child(Separator::horizontal().flex_1()),
        }
    }
}

//! Displays a form textarea or a component that looks like a textarea.
//! Grows with its content (`field-sizing-content`).
//!
//! ```ignore
//! Textarea::new("message").placeholder("Type your message here.")
//! ```

use gpui::{App, ElementId, Entity, IntoElement, RenderOnce, SharedString, StyleRefinement, Window};
use std::rc::Rc;

use super::input::{Input, InputState};
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Textarea {
    input: Input,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Textarea);

impl Textarea {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { input: Input::new(id).multiline(true), on_change: None, style: StyleRefinement::default() }
    }

    pub fn from_state(state: &Entity<InputState>) -> Self {
        Self { input: Input::from_state(state).multiline(true), on_change: None, style: StyleRefinement::default() }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.input = self.input.placeholder(placeholder);
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.input = self.input.default_value(value);
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.input = self.input.value(value);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.input = self.input.disabled(disabled);
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.input = self.input.invalid(invalid);
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Textarea {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut input = self.input;
        if let Some(handler) = self.on_change {
            input = input.on_change(move |text, window, cx| handler(text, window, cx));
        }
        input.refine_style(&self.style)
    }
}

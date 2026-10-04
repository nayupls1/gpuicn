//! Building forms with validation — the gpui take on shadcn's
//! react-hook-form wrapper: declare fields with validators, get the values on
//! a valid submit.
//!
//! ```ignore
//! Form::new("profile")
//!     .field(
//!         FormField::text("username")
//!             .label("Username")
//!             .placeholder("shadcn")
//!             .description("This is your public display name.")
//!             .validate(|value| (value.len() < 2).then(|| "Username must be at least 2 characters.".into())),
//!     )
//!     .field(FormField::textarea("bio").label("Bio"))
//!     .on_submit(|values, _, _| println!("{:?}", values.get("username")))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*,
};
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use super::button::Button;
use super::field::Field;
use super::input::Input;
use super::utils::StyledExt;

/// The submitted values, by field name.
pub type FormValues = BTreeMap<SharedString, SharedString>;
type Validator = Rc<dyn Fn(&str) -> Option<SharedString>>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FieldKind {
    Text,
    Password,
    Textarea,
}

enum FieldControl {
    Input(FieldKind),
    Custom(AnyElement),
}

pub struct FormField {
    name: SharedString,
    label: Option<SharedString>,
    description: Option<SharedString>,
    placeholder: Option<SharedString>,
    default_value: Option<SharedString>,
    validators: Vec<Validator>,
    control: FieldControl,
}

impl FormField {
    fn with_control(name: impl Into<SharedString>, control: FieldControl) -> Self {
        Self {
            name: name.into(),
            label: None,
            description: None,
            placeholder: None,
            default_value: None,
            validators: Vec::new(),
            control,
        }
    }

    pub fn text(name: impl Into<SharedString>) -> Self {
        Self::with_control(name, FieldControl::Input(FieldKind::Text))
    }

    pub fn password(name: impl Into<SharedString>) -> Self {
        Self::with_control(name, FieldControl::Input(FieldKind::Password))
    }

    pub fn textarea(name: impl Into<SharedString>) -> Self {
        Self::with_control(name, FieldControl::Input(FieldKind::Textarea))
    }

    /// Any control (a select, a switch…); you manage its value.
    pub fn custom(name: impl Into<SharedString>, control: impl IntoElement) -> Self {
        Self::with_control(name, FieldControl::Custom(control.into_any_element()))
    }

    /// `FormLabel`
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// `FormDescription`
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Return `Some(message)` to reject the value (`FormMessage`).
    pub fn validate(mut self, validator: impl Fn(&str) -> Option<SharedString> + 'static) -> Self {
        self.validators.push(Rc::new(validator));
        self
    }

    /// Shorthand for a non-empty check.
    pub fn required(self, message: impl Into<SharedString>) -> Self {
        let message = message.into();
        self.validate(move |value| value.trim().is_empty().then(|| message.clone()))
    }
}

#[derive(Default)]
struct FormState {
    submitted: bool,
    errors: HashMap<SharedString, SharedString>,
}

#[derive(IntoElement)]
pub struct Form {
    id: ElementId,
    fields: Vec<FormField>,
    submit_label: SharedString,
    on_submit: Option<Rc<dyn Fn(&FormValues, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Form);

impl Form {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), fields: Vec::new(), submit_label: "Submit".into(), on_submit: None, style: StyleRefinement::default() }
    }

    pub fn field(mut self, field: FormField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn submit_label(mut self, label: impl Into<SharedString>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// Called with all text values when every validator passes.
    pub fn on_submit(mut self, handler: impl Fn(&FormValues, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
    }
}

fn validate(validators: &[Validator], value: &str) -> Option<SharedString> {
    validators.iter().find_map(|v| v(value))
}

impl RenderOnce for Form {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| FormState::default());
        let mut inputs = Vec::new();
        let mut rows = Vec::new();
        let submitted = state.read(cx).submitted;

        for field in self.fields {
            let control = match field.control {
                FieldControl::Input(kind) => {
                    let mut input = Input::new(ElementId::Name(format!("form-{}", field.name).into()))
                        .masked(kind == FieldKind::Password)
                        .multiline(kind == FieldKind::Textarea);
                    if let Some(placeholder) = field.placeholder.clone() {
                        input = input.placeholder(placeholder);
                    }
                    if let Some(default) = field.default_value.clone() {
                        input = input.default_value(default);
                    }
                    let input_state = input.state(window, cx);
                    let value = input_state.read(cx).text().clone();
                    // Re-validate live once the user has tried to submit.
                    let error = if submitted { validate(&field.validators, &value) } else { state.read(cx).errors.get(&field.name).cloned() };
                    inputs.push((field.name.clone(), input_state.clone(), field.validators.clone()));
                    (input.with_state(&input_state).invalid(error.is_some()).into_any_element(), error)
                }
                FieldControl::Custom(element) => (element, None),
            };
            let (control, error) = control;
            let mut f = Field::new().error(error);
            if let Some(label) = field.label {
                f = f.label(label);
            }
            if let Some(description) = field.description {
                f = f.description(description);
            }
            rows.push(f.child(control));
        }

        let on_submit = self.on_submit;
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_6()
            .refine_style(&self.style)
            .children(rows)
            .child(div().child(Button::new("form-submit").label(self.submit_label).on_click(move |_, window, cx| {
                let mut values = FormValues::new();
                let mut errors = HashMap::new();
                for (name, input, validators) in &inputs {
                    let value = input.read(cx).text().clone();
                    if let Some(error) = validate(validators, &value) {
                        errors.insert(name.clone(), error);
                    }
                    values.insert(name.clone(), value);
                }
                let valid = errors.is_empty();
                state.update(cx, |s, cx| {
                    s.submitted = true;
                    s.errors = errors;
                    cx.notify();
                });
                if valid && let Some(handler) = &on_submit {
                    handler(&values, window, cx);
                }
            })))
    }
}

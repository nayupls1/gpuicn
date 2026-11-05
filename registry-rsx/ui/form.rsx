//! Building forms with validation: the rsx take on shadcn's react-hook-form
//! wrapper. Declare fields with validators, get the values on a valid submit.
//!
//! ```ignore
//! <Form
//!     fields={vec![
//!         FormField::text("username")
//!             .label("Username")
//!             .placeholder("shadcn")
//!             .description("This is your public display name.")
//!             .validate(|value| (value.len() < 2).then(|| "Username must be at least 2 characters.".into())),
//!         FormField::textarea("bio").label("Bio"),
//!     ]}
//!     onSubmit={move |values: &FormValues| println!("{:?}", values.get("username"))}
//! />
//! ```
//!
//! Enter in a single-line field submits too (it's a `<form>`).

use std::collections::BTreeMap;
use std::rc::Rc;

use rsx::prelude::*;

use super::button::Button;
use super::field::Field;
use super::input::Input;

/// The submitted values, by field name.
pub type FormValues = BTreeMap<SharedString, SharedString>;
type Validator = Rc<dyn Fn(&str) -> Option<SharedString>>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FieldKind {
    Text,
    Password,
    Textarea,
}

#[derive(Clone)]
enum FieldControl {
    Input(FieldKind),
    Custom(Element),
}

/// One field of a [`Form`] (`FormField` + `FormItem` + `FormControl`).
#[derive(Clone)]
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
        Self::with_control(name, FieldControl::Custom(control.into_element()))
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

fn validate(validators: &[Validator], value: &str) -> Option<SharedString> {
    validators.iter().find_map(|v| v(value))
}

#[component]
pub fn Form(
    /// The fields, in order (gpuicn's `.field(..)` calls).
    #[prop(optional)]
    fields: Vec<FormField>,
    #[prop(default = "Submit".to_string())]
    submit_label: String,
    /// Called with all text values when every validator passes.
    on_submit: Option<Callback<FormValues>>,
    attrs: Attributes,
) -> impl IntoElement {
    // Re-validate live once the user has tried to submit.
    let submitted = signal(false);
    let mut inputs = Vec::new();
    let mut rows = Vec::new();

    for field in fields {
        let (control, error) = match field.control {
            FieldControl::Input(kind) => {
                let value = signal(field.default_value.as_ref().map(|v| v.to_string()).unwrap_or_default());
                let validators = StoredValue::new(field.validators.clone());
                let error = memo(move || {
                    submitted.get().then(|| validators.with_value(|v| value.with(|text| validate(v, text)))).flatten().map(|e| e.to_string())
                });
                inputs.push((field.name.clone(), value, field.validators.clone()));
                let placeholder = field.placeholder.as_ref().map(|p| p.to_string());
                let input = <Input
                    text={value}
                    placeholder={placeholder}
                    masked={kind == FieldKind::Password}
                    multiline={kind == FieldKind::Textarea}
                    invalid={error.with(|e| e.is_some())}
                />;
                (input, Some(error))
            }
            FieldControl::Custom(element) => (element, None),
        };
        let label = field.label.map(|l| l.to_string());
        let description = field.description.map(|d| d.to_string());
        rows.push(match error {
            Some(error) => <Field label={label} description={description} error={error.get()}>{control}</Field>,
            None => <Field label={label} description={description}>{control}</Field>,
        });
    }

    let inputs = StoredValue::new(inputs);
    let submit = move || {
        let mut values = FormValues::new();
        let mut valid = true;
        inputs.with_value(|inputs| {
            for (name, value, validators) in inputs {
                let value = value.get_untracked();
                valid &= validate(validators, &value).is_none();
                values.insert(name.clone(), value.into());
            }
        });
        submitted.set(true);
        if valid {
            if let Some(f) = on_submit {
                f.call(&values);
            }
        }
    };

    <form class="flex flex-col gap-6" onSubmit={move |_| submit()} {...&attrs}>
        {rows}
        <div class="flex">
            <Button label={submit_label} onClick={move |_| submit()} />
        </div>
    </form>
}

//! Combine labels, controls, and help text to compose accessible form fields.
//!
//! ```ignore
//! <FieldSet legend="Payment Method" description="All transactions are secure and encrypted">
//!     <FieldGroup>
//!         <Field label="Name on Card"><Input /></Field>
//!         <Field label="Card Number" description="Enter your 16-digit card number" error={error.get()}>
//!             <Input />
//!         </Field>
//!     </FieldGroup>
//! </FieldSet>
//! ```

use rsx::prelude::*;
use rsx::Div;

use super::label::Label;
use super::separator::Separator;
use super::theme::theme;

#[component]
pub fn Field(
    /// Label beside the control (checkboxes, switches).
    horizontal: bool,
    label: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// Validation message; also tints the label. `error={err.get()}` with an
    /// `Option<String>` signal, or `error={"msg".to_string()}`.
    error: Option<Accessor<Option<String>>>,
    disabled: Option<Accessor<bool>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let error = memo(move || error.and_then(|e| e.get()));
    let label = label.map(|label| {
        // An invalid field's label turns red.
        let tint = move |el: Div| match error.with(|e| e.is_some()) {
            true => el.text_color(t.get().destructive),
            false => el,
        };
        // A label next to a checkbox reads as body text (`font-normal`).
        // rsx: a component's `Attributes` only forward classes, HTML attributes
        // and events, not style methods like `text_color`/`bind`; spread one.
        let tint = Attributes::new().with(move |el| el.bind(tint));
        <Label text={label} disabled={disabled} class={if horizontal { "font-normal" } else { "" }} {...tint} />
    });
    let description = description.map(|d| <div class="text-sm leading-[1.25rem]" text_color={t.get().muted_foreground}>{d}</div>);
    let error_text = move || error.get().map(|e| <div class="text-sm leading-[1.25rem]" text_color={t.get().destructive}>{e}</div>);

    if horizontal {
        <div class="flex items-start w-full gap-3" {...&attrs}>
            {children}
            <div class="flex flex-col flex-1 min-w-0 gap-1.5">
                {label}
                {description}
                {error_text}
            </div>
        </div>
    } else {
        let has_help = description.is_some();
        <div class="flex flex-col w-full gap-3" {...&attrs}>
            {label}
            {/* Help text and errors sit close to their control (`gap-2`) and to
                each other, so validation doesn't make the form jump. */}
            <div class="flex flex-col gap-2">
                {children}
                {move || (has_help || error.with(|e| e.is_some())).then(|| <div class="flex flex-col gap-1">
                    {description.clone()}
                    {error_text}
                </div>)}
            </div>
        </div>
    }
}

/// Stacks fields with consistent spacing.
#[component]
pub fn FieldGroup(children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    <div class="flex flex-col w-full gap-7" {...attrs}>{children}</div>
}

/// A group of related fields with a legend (`<fieldset>`).
#[component]
pub fn FieldSet(
    legend: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    <div class="flex flex-col gap-6" {...attrs}>
        {(legend.is_some() || description.is_some()).then(|| <div class="flex flex-col gap-1.5">
            {legend.map(|l| <div class="text-base leading-[1.5rem] font-medium">{l}</div>)}
            {description.map(|d| <div class="text-sm leading-[1.25rem]" text_color={t.get().muted_foreground}>{d}</div>)}
        </div>)}
        {children}
    </div>
}

/// A separator between fields, optionally with inline text ("Or continue with").
#[component]
pub fn FieldSeparator(text: Option<Accessor<String>>) -> impl IntoElement {
    let t = theme();
    match text {
        None => <div class="py-2"><Separator /></div>,
        Some(text) => <div class="flex items-center gap-2 text-sm leading-[1.25rem]" text_color={t.get().muted_foreground}>
            <Separator class="flex-1" />
            {text}
            <Separator class="flex-1" />
        </div>,
    }
}

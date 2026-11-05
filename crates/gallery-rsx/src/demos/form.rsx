use rsx::prelude::*;

use crate::ui::form::{Form, FormField, FormValues};
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    let submitted = signal(None::<String>);
    <div class="flex flex-col gap-6">
        <Form
            class="w-[420px]"
            fields={vec![
                FormField::text("username")
                    .label("Username")
                    .placeholder("shadcn")
                    .description("This is your public display name.")
                    .validate(|value| (value.chars().count() < 2).then(|| "Username must be at least 2 characters.".into())),
                FormField::text("email")
                    .label("Email")
                    .placeholder("m@example.com")
                    .validate(|value| (!value.contains('@')).then(|| "Please enter a valid email.".into())),
                FormField::textarea("bio").label("Bio").placeholder("Tell us a little bit about yourself"),
            ]}
            onSubmit={move |values: &FormValues| {
                let summary: Vec<String> = values.iter().map(|(k, v)| format!("{k}: {v}")).collect();
                submitted.set(Some(summary.join(", ")));
            }}
        />
        {/* TODO(sonner): `toast(Toast::new("You submitted the following values").description(summary))` instead of this line once `sonner` is ported. */}
        {move || submitted.get().map(|s| <div class="text-sm w-[420px]" text_color={t.get().muted_foreground}>You submitted the following values: {s}</div>)}
    </div>
}

//! A modal dialog that interrupts the user with important content and
//! expects a response. Clicking the backdrop does not dismiss it.
//!
//! ```ignore
//! <AlertDialog
//!     trigger={<Button outline label="Show Dialog" />}
//!     title="Are you absolutely sure?"
//!     description="This action cannot be undone. This will permanently delete your account."
//!     cancel="Cancel"
//!     action="Continue"
//!     onAction={move |_| println!("deleted")}
//! />
//! ```

use rsx::prelude::*;

use super::button::{Button, ButtonVariant};
use super::dialog::{header, modal_trigger, use_modal, DialogHandle, Modal, ModalPlacement};
use super::theme::theme;

#[component]
pub fn AlertDialog(
    /// The element that opens the dialog.
    trigger: Option<Children>,
    title: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// `AlertDialogCancel` label.
    #[prop(default = Accessor::value("Cancel".to_string()))]
    cancel: Accessor<String>,
    on_cancel: Option<Callback<()>>,
    /// `AlertDialogAction` label (`onAction` is its handler).
    #[prop(default = Accessor::value("Continue".to_string()))]
    action: Accessor<String>,
    on_action: Option<Callback<()>>,
    /// Style the action button (e.g. `ButtonVariant::Destructive`).
    #[prop(optional)]
    action_variant: ButtonVariant,
    /// Control the open state from outside.
    open: Option<Accessor<bool>>,
    on_open_change: Option<Callback<bool>>,
    /// Share state with the caller (see `DialogHandle`).
    handle: Option<DialogHandle>,
    /// Styles for the panel.
    attrs: Attributes,
) -> impl IntoElement {
    let handle = use_modal(open, on_open_change, handle);
    let t = theme();
    let attrs = StoredValue::new(attrs);
    let panel = move |handle: DialogHandle| {
        // rsx#7: no `shadow-lg`.
        <div
            class="flex flex-col gap-4 w-[512px] max-w-full p-6 border"
            corner_radius={t.get().radius_lg()}
            border_color={t.get().border}
            bg={t.get().background}
            text_color={t.get().foreground}
            {...attrs.with_value(|a| a.clone())}
        >
            {header(title, description, "")}
            <div class="flex justify-end gap-2">
                <Button outline label={cancel} onClick={move |_| {
                    handle.close();
                    if let Some(f) = on_cancel {
                        f.run();
                    }
                }} />
                <Button variant={action_variant} label={action} onClick={move |_| {
                    handle.close();
                    if let Some(f) = on_action {
                        f.run();
                    }
                }} />
            </div>
        </div>
    };
    <div class="flex">
        {modal_trigger(handle, trigger)}
        <Modal handle={handle} placement={ModalPlacement::Center} dismissible={false} panel={panel} />
    </div>
}

//! A modal dialog that interrupts the user with important content and
//! expects a response. Clicking the backdrop does not dismiss it.
//!
//! ```ignore
//! AlertDialog::new("delete-account")
//!     .trigger(Button::new("open").outline().label("Show Dialog"))
//!     .title("Are you absolutely sure?")
//!     .description("This action cannot be undone. This will permanently delete your account.")
//!     .cancel("Cancel")
//!     .action("Continue", |_, _| println!("deleted"))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px,
};
use std::rc::Rc;

use super::button::{Button, ButtonVariant};
use super::dialog::{DialogHandle, ModalPlacement, header, render_modal, use_modal};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct AlertDialog {
    id: ElementId,
    trigger: Option<AnyElement>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    cancel: SharedString,
    action: SharedString,
    action_variant: ButtonVariant,
    on_action: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    on_cancel: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(AlertDialog);

impl AlertDialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            title: None,
            description: None,
            cancel: "Cancel".into(),
            action: "Continue".into(),
            action_variant: ButtonVariant::Default,
            on_action: None,
            on_cancel: None,
            open: None,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// `AlertDialogCancel` label.
    pub fn cancel(mut self, label: impl Into<SharedString>) -> Self {
        self.cancel = label.into();
        self
    }

    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(handler));
        self
    }

    /// `AlertDialogAction` label and handler.
    pub fn action(mut self, label: impl Into<SharedString>, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.action = label.into();
        self.on_action = Some(Rc::new(handler));
        self
    }

    /// Style the action button (e.g. `ButtonVariant::Destructive`).
    pub fn action_variant(mut self, variant: ButtonVariant) -> Self {
        self.action_variant = variant;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for AlertDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let modal = use_modal(&self.id, self.open, self.on_open_change.clone(), window, cx);
        let trigger_handle = modal.handle.clone();
        let window_width = window.viewport_size().width;
        let (title, description, style) = (self.title, self.description, self.style);
        let (cancel, action, variant) = (self.cancel, self.action, self.action_variant);
        let (on_cancel, on_action) = (self.on_cancel, self.on_action);

        render_modal(&modal, ModalPlacement::Center, false, window, cx, move |handle: &DialogHandle, _, cx| {
            let theme = cx.theme();
            let (cancel_handle, action_handle) = (handle.clone(), handle.clone());
            div()
                .flex()
                .flex_col()
                .gap_4()
                .w(px(512.).min(window_width - px(32.)))
                .p_6()
                .rounded(theme.radius_lg())
                .border_1()
                .border_color(theme.border)
                .bg(theme.background)
                .text_color(theme.foreground)
                .when(theme.shadows, |this| this.shadow_lg())
                .refine_style(&style)
                .children(header(title, description, cx))
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap_2()
                        .child(Button::new("cancel").outline().label(cancel).on_click(move |_, window, cx| {
                            cancel_handle.close(window, cx);
                            if let Some(handler) = &on_cancel {
                                handler(window, cx);
                            }
                        }))
                        .child(Button::new("action").variant(variant).label(action).on_click(move |_, window, cx| {
                            action_handle.close(window, cx);
                            if let Some(handler) = &on_action {
                                handler(window, cx);
                            }
                        })),
                )
                .into_any_element()
        });

        div().id(self.id).flex().children(self.trigger.map(|trigger| {
            div()
                .id("alert-dialog-trigger")
                .child(trigger)
                .on_click(move |_, window, cx| trigger_handle.open(window, cx))
        }))
    }
}

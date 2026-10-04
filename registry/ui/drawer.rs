//! A drawer component that slides up from the bottom of the screen
//! (the `vaul` drawer in shadcn).
//!
//! ```ignore
//! Drawer::new("goal")
//!     .trigger(Button::new("open").outline().label("Open Drawer"))
//!     .title("Move Goal")
//!     .description("Set your daily activity goal.")
//!     .content(|_, _, _| goal_picker)
//!     .footer(|drawer, _, _| Button::new("submit").label("Submit"))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px,
};
use std::rc::Rc;

use super::dialog::{DialogHandle, ModalPlacement, header, render_modal, slot, use_modal};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

type SlotBuilder = Rc<dyn Fn(&DialogHandle, &mut Window, &mut App) -> AnyElement>;

#[derive(IntoElement)]
pub struct Drawer {
    id: ElementId,
    trigger: Option<AnyElement>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    content: Option<SlotBuilder>,
    footer: Option<SlotBuilder>,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Drawer);

impl Drawer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            title: None,
            description: None,
            content: None,
            footer: None,
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

    pub fn content<E: IntoElement>(mut self, content: impl Fn(&DialogHandle, &mut Window, &mut App) -> E + 'static) -> Self {
        self.content = Some(slot(content));
        self
    }

    pub fn footer<E: IntoElement>(mut self, footer: impl Fn(&DialogHandle, &mut Window, &mut App) -> E + 'static) -> Self {
        self.footer = Some(slot(footer));
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

impl RenderOnce for Drawer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let modal = use_modal(&self.id, self.open, self.on_open_change.clone(), window, cx);
        let trigger_handle = modal.handle.clone();
        let max_height = window.viewport_size().height * 0.8;
        let (title, description, content, footer, style) =
            (self.title, self.description, self.content, self.footer, self.style);

        render_modal(&modal, ModalPlacement::Bottom, true, window, cx, move |handle, window, cx| {
            let content = content.map(|f| f(handle, window, cx));
            let footer = footer.map(|f| f(handle, window, cx));
            let header = header(title, description, cx);
            let theme = cx.theme();
            div()
                .flex()
                .flex_col()
                .w_full()
                .max_h(max_height)
                .rounded_t(theme.radius_lg())
                .border_t_1()
                .border_color(theme.border)
                .bg(theme.background)
                .text_color(theme.foreground)
                .refine_style(&style)
                .child(div().mx_auto().mt_4().h_2().w(px(100.)).flex_none().rounded_full().bg(theme.muted))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .max_w(px(384.))
                        .mx_auto()
                        .children(header.map(|header| header.gap_1p5().p_4().items_center().text_center()))
                        .children(content.map(|content| div().px_4().child(content)))
                        .children(footer.map(|footer| div().flex().flex_col().gap_2().p_4().child(footer))),
                )
                .into_any_element()
        });

        div().id(self.id).flex().children(self.trigger.map(|trigger| {
            div()
                .id("drawer-trigger")
                .child(trigger)
                .on_click(move |_, window, cx| trigger_handle.open(window, cx))
        }))
    }
}

//! Extends the Dialog component to display content that complements the main
//! content of the screen, sliding in from an edge.
//!
//! ```ignore
//! Sheet::new("edit")
//!     .side(Side::Right)
//!     .trigger(Button::new("open").outline().label("Open"))
//!     .title("Edit profile")
//!     .description("Make changes to your profile here.")
//!     .content(|_, _, _| form)
//!     .footer(|sheet, _, _| Button::new("save").label("Save changes"))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px,
};
use std::rc::Rc;

use super::dialog::{DialogHandle, ModalPlacement, close_button, header, render_modal, slot, use_modal};
use super::theme::ActiveTheme;
use super::utils::{Side, StyledExt};

type SlotBuilder = Rc<dyn Fn(&DialogHandle, &mut Window, &mut App) -> AnyElement>;

#[derive(IntoElement)]
pub struct Sheet {
    id: ElementId,
    side: Side,
    trigger: Option<AnyElement>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    content: Option<SlotBuilder>,
    footer: Option<SlotBuilder>,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Sheet);

impl Sheet {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            side: Side::Right,
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

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
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

impl RenderOnce for Sheet {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let modal = use_modal(&self.id, self.open, self.on_open_change.clone(), window, cx);
        let trigger_handle = modal.handle.clone();
        let viewport = window.viewport_size();
        let side = self.side;
        let placement = match side {
            Side::Top => ModalPlacement::Top,
            Side::Right => ModalPlacement::Right,
            Side::Bottom => ModalPlacement::Bottom,
            Side::Left => ModalPlacement::Left,
        };
        let (title, description, content, footer, style) =
            (self.title, self.description, self.content, self.footer, self.style);

        render_modal(&modal, placement, true, window, cx, move |handle, window, cx| {
            let content = content.map(|f| f(handle, window, cx));
            let footer = footer.map(|f| f(handle, window, cx));
            let header = header(title, description, cx);
            let theme = cx.theme();
            div()
                .relative()
                .flex()
                .flex_col()
                .gap_4()
                .bg(theme.background)
                .text_color(theme.foreground)
                .border_color(theme.border)
                .when(theme.shadows, |this| this.shadow_lg())
                .map(|this| match side {
                    Side::Right => this.h_full().w((viewport.width * 0.75).min(px(384.))).border_l_1(),
                    Side::Left => this.h_full().w((viewport.width * 0.75).min(px(384.))).border_r_1(),
                    Side::Top => this.w_full().border_b_1(),
                    Side::Bottom => this.w_full().border_t_1(),
                })
                .refine_style(&style)
                .children(header.map(|header| header.gap_1p5().p_4()))
                .children(content.map(|content| div().flex_1().px_4().child(content)))
                .children(footer.map(|footer| div().mt_auto().flex().flex_col().gap_2().p_4().child(footer)))
                .child(close_button(handle))
                .into_any_element()
        });

        div().id(self.id).flex().children(self.trigger.map(|trigger| {
            div()
                .id("sheet-trigger")
                .child(trigger)
                .on_click(move |_, window, cx| trigger_handle.open(window, cx))
        }))
    }
}

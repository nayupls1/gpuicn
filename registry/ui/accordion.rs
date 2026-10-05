//! A vertically stacked set of interactive headings that each reveal a
//! section of content.
//!
//! ```ignore
//! Accordion::new("faq")
//!     .default_value(["item-1"])
//!     .item(AccordionItem::new("item-1", "Product Information").content(p("Our flagship product…")))
//!     .item(AccordionItem::new("item-2", "Shipping Details").content(p("We offer worldwide shipping…")))
//! ```

use gpui::{
    AnyElement, App, ElementId, FontWeight, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

pub struct AccordionItem {
    value: SharedString,
    title: SharedString,
    content: Option<AnyElement>,
    disabled: bool,
}

impl AccordionItem {
    pub fn new(value: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self { value: value.into(), title: title.into(), content: None, disabled: false }
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(IntoElement)]
pub struct Accordion {
    id: ElementId,
    items: Vec<AccordionItem>,
    multiple: bool,
    collapsible: bool,
    value: Option<Vec<SharedString>>,
    default_value: Vec<SharedString>,
    on_change: Option<Rc<dyn Fn(&Vec<SharedString>, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Accordion);

impl Accordion {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            multiple: false,
            collapsible: true,
            value: None,
            default_value: Vec::new(),
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }

    /// Allow several items open at once (`type="multiple"`).
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// In single mode, whether the open item can be closed again (default true).
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn value(mut self, value: Vec<SharedString>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.default_value = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&Vec<SharedString>, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Accordion {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_value.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let open_values = self.value.clone().unwrap_or_else(|| state.read(cx).clone());
        let theme = cx.theme();
        let border = theme.border;
        let muted_fg = theme.muted_foreground;
        let count = self.items.len();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w_full()
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map(|(ix, item)| {
                let open = open_values.contains(&item.value);
                let state = state.clone();
                let open_values = open_values.clone();
                let on_change = self.on_change.clone();
                let (multiple, collapsible) = (self.multiple, self.collapsible);
                let value = item.value.clone();
                let disabled = item.disabled;

                div()
                    .flex()
                    .flex_col()
                    .when(ix + 1 < count, |this| this.border_b_1().border_color(border))
                    .child(
                        div()
                            .id(ix)
                            .flex()
                            .items_start()
                            .justify_between()
                            .gap_4()
                            .py_4()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .when(disabled, |this| this.opacity(0.5))
                            .when(!disabled, |this| this.cursor_pointer().hover(|s| s.underline()))
                            .child(div().flex_1().min_w_0().child(item.title))
                            .child(
                                Icon::new(if open { IconName::ChevronUp } else { IconName::ChevronDown })
                                    .text_color(muted_fg)
                                    .mt_0p5(),
                            )
                            .when(!disabled, |this| {
                                this.on_click(move |_, window, cx| {
                                    let next = match (open, multiple) {
                                        (true, true) => open_values.iter().filter(|v| **v != value).cloned().collect(),
                                        (true, false) if collapsible => Vec::new(),
                                        (true, false) => open_values.clone(),
                                        (false, true) => {
                                            let mut next = open_values.clone();
                                            next.push(value.clone());
                                            next
                                        }
                                        (false, false) => vec![value.clone()],
                                    };
                                    state.update(cx, |s, cx| {
                                        *s = next.clone();
                                        cx.notify();
                                    });
                                    if let Some(handler) = &on_change {
                                        handler(&next, window, cx);
                                    }
                                })
                            }),
                    )
                    .when(open, |this| {
                        this.child(div().pb_4().text_sm().children(item.content))
                    })
            }))
    }
}

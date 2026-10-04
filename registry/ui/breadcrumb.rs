//! Displays the path to the current resource using a hierarchy of links.
//!
//! ```ignore
//! Breadcrumb::new()
//!     .item(BreadcrumbItem::new("Home").on_click(|_, _, _| {}))
//!     .ellipsis()
//!     .item(BreadcrumbItem::new("Components"))
//!     .item(BreadcrumbItem::new("Breadcrumb").current())
//! ```

use gpui::{
    AnyElement, App, ClickEvent, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

pub struct BreadcrumbItem {
    label: SharedString,
    current: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), current: false, on_click: None }
    }

    /// The current page (`BreadcrumbPage`).
    pub fn current(mut self) -> Self {
        self.current = true;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

enum Entry {
    Item(BreadcrumbItem),
    Ellipsis,
    Custom(AnyElement),
}

#[derive(IntoElement)]
pub struct Breadcrumb {
    entries: Vec<Entry>,
    separator: Option<Rc<dyn Fn() -> AnyElement>>,
    style: StyleRefinement,
}

crate::impl_styled!(Breadcrumb);

impl Breadcrumb {
    pub fn new() -> Self {
        Self { entries: Vec::new(), separator: None, style: StyleRefinement::default() }
    }

    pub fn item(mut self, item: BreadcrumbItem) -> Self {
        self.entries.push(Entry::Item(item));
        self
    }

    /// `BreadcrumbEllipsis` — collapsed items.
    pub fn ellipsis(mut self) -> Self {
        self.entries.push(Entry::Ellipsis);
        self
    }

    /// Any element as an entry, e.g. a dropdown menu.
    pub fn custom(mut self, element: impl IntoElement) -> Self {
        self.entries.push(Entry::Custom(element.into_any_element()));
        self
    }

    /// Replace the default chevron separator.
    pub fn separator<E: IntoElement>(mut self, separator: impl Fn() -> E + 'static) -> Self {
        self.separator = Some(Rc::new(move || separator().into_any_element()));
        self
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let muted = theme.muted_foreground;
        let foreground = theme.foreground;
        let count = self.entries.len();
        let mut children: Vec<AnyElement> = Vec::new();

        for (ix, entry) in self.entries.into_iter().enumerate() {
            children.push(match entry {
                Entry::Item(item) => {
                    let el = div().id(ix).child(item.label);
                    if item.current {
                        el.text_color(foreground).into_any_element()
                    } else if let Some(handler) = item.on_click {
                        el.cursor_pointer()
                            .hover(move |s| s.text_color(foreground))
                            .on_click(move |e, w, cx| handler(e, w, cx))
                            .into_any_element()
                    } else {
                        el.into_any_element()
                    }
                }
                Entry::Ellipsis => div()
                    .flex()
                    .size_9()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(IconName::Ellipsis))
                    .into_any_element(),
                Entry::Custom(el) => el,
            });
            if ix + 1 < count {
                children.push(match &self.separator {
                    Some(separator) => separator(),
                    None => Icon::new(IconName::ChevronRight).size_3p5().into_any_element(),
                });
            }
        }

        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_2p5()
            .text_sm()
            .text_color(muted)
            .refine_style(&self.style)
            .children(children)
    }
}

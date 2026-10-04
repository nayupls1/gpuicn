//! Pagination with page navigation, next and previous links.
//!
//! ```ignore
//! Pagination::new("pages")
//!     .total(10)
//!     .page(self.page)
//!     .on_change(cx.listener(|this, page, _, cx| { this.page = *page; cx.notify() }))
//! ```

use gpui::{App, ElementId, IntoElement, RenderOnce, StyleRefinement, Window, div, prelude::*};
use std::rc::Rc;

use super::button::{Button, ButtonSize};
use super::icon::{Icon, IconName};
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Pagination {
    id: ElementId,
    /// 1-based current page.
    page: usize,
    total: usize,
    siblings: usize,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Pagination);

impl Pagination {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), page: 1, total: 1, siblings: 1, on_change: None, style: StyleRefinement::default() }
    }

    pub fn page(mut self, page: usize) -> Self {
        self.page = page.max(1);
        self
    }

    pub fn total(mut self, total: usize) -> Self {
        self.total = total.max(1);
        self
    }

    /// How many page links to show on each side of the current page.
    pub fn siblings(mut self, siblings: usize) -> Self {
        self.siblings = siblings;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

/// Page numbers to display; `None` marks an ellipsis.
pub fn page_range(page: usize, total: usize, siblings: usize) -> Vec<Option<usize>> {
    let mut pages = Vec::new();
    let start = page.saturating_sub(siblings).max(2);
    let end = (page + siblings).min(total.saturating_sub(1));
    pages.push(Some(1));
    if start > 2 {
        pages.push(None);
    }
    for p in start..=end {
        pages.push(Some(p));
    }
    if end + 1 < total {
        pages.push(None);
    }
    if total > 1 {
        pages.push(Some(total));
    }
    pages
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let page = self.page.min(self.total);
        let total = self.total;
        let on_change = self.on_change;
        let go = move |target: usize| {
            let on_change = on_change.clone();
            move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                if let Some(handler) = &on_change {
                    handler(&target, window, cx);
                }
            }
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .gap_1()
            .refine_style(&self.style)
            .child(
                Button::new("previous")
                    .ghost()
                    .icon(IconName::ChevronLeft)
                    .label("Previous")
                    .pl_2p5()
                    .disabled(page <= 1)
                    .on_click(go(page.saturating_sub(1).max(1))),
            )
            .children(page_range(page, total, self.siblings).into_iter().enumerate().map(|(ix, entry)| {
                match entry {
                    Some(p) => Button::new(("page", p))
                        .size(ButtonSize::Icon)
                        .map(|b| if p == page { b.outline() } else { b.ghost() })
                        .label(p.to_string())
                        .on_click(go(p))
                        .into_any_element(),
                    None => div()
                        .id(("ellipsis", ix))
                        .flex()
                        .size_9()
                        .items_center()
                        .justify_center()
                        .child(Icon::new(IconName::Ellipsis))
                        .into_any_element(),
                }
            }))
            .child(
                Button::new("next")
                    .ghost()
                    .label("Next")
                    .icon_end(IconName::ChevronRight)
                    .pr_2p5()
                    .disabled(page >= total)
                    .on_click(go((page + 1).min(total))),
            )
    }
}

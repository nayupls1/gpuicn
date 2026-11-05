//! Pagination with page navigation, next and previous links.
//!
//! ```ignore
//! let page = signal(2usize);
//! <Pagination total={10usize} page={page} onChange={move |p: &usize| page.set(*p)} />
//! ```

use rsx::prelude::*;

use super::button::{Button, ButtonSize};
use super::icon::{Icon, IconName};

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

#[component]
pub fn Pagination(
    /// 1-based current page. Without it, the pagination keeps its own.
    page: Option<Accessor<usize>>,
    /// Number of pages.
    total: Option<Accessor<usize>>,
    /// How many page links to show on each side of the current page.
    #[prop(default = 1)]
    siblings: usize,
    on_change: Option<Callback<usize>>,
    attrs: Attributes,
) -> impl IntoElement {
    let own = signal(1usize);
    let total = move || total.map(|t| t.get()).unwrap_or(1).max(1);
    let current = move || page.map(|p| p.get()).unwrap_or_else(|| own.get()).max(1).min(total());
    let go = move |target: usize| {
        own.set(target);
        if let Some(f) = on_change {
            f.call(&target);
        }
    };

    <div class="flex items-center justify-center gap-1" {...attrs}>
        <Button
            ghost
            icon={IconName::ChevronLeft}
            label="Previous"
            class="gap-1 pl-2.5"
            disabled={move || current() <= 1}
            onClick={move |_| go(current().saturating_sub(1).max(1))}
        />
        {move || {
            let page = current();
            page_range(page, total(), siblings)
                .into_iter()
                .map(|entry| match entry {
                    Some(p) => <Button
                        size={ButtonSize::Icon}
                        outline={p == page}
                        ghost={p != page}
                        label={p.to_string()}
                        onClick={move |_| go(p)}
                    />,
                    None => <div class="flex size-9 items-center justify-center">
                        <Icon name={IconName::Ellipsis} />
                    </div>,
                })
                .collect::<Vec<_>>()
        }}
        <Button
            ghost
            label="Next"
            icon_end={IconName::ChevronRight}
            class="gap-1 pr-2.5"
            disabled={move || current() >= total()}
            onClick={move |_| go((current() + 1).min(total()))}
        />
    </div>
}

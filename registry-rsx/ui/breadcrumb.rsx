//! Displays the path to the current resource using a hierarchy of links.
//!
//! ```ignore
//! <Breadcrumb>
//!     <BreadcrumbItem label="Home" onClick={move |_| go_home()} />
//!     <BreadcrumbEllipsis />
//!     <BreadcrumbItem label="Components" onClick={move |_| {}} />
//!     <BreadcrumbItem label="Breadcrumb" current />
//! </Breadcrumb>
//! ```
//!
//! A separator (a chevron, or `separator`) is drawn between entries. Any
//! element can be an entry: pass it as a `BreadcrumbItem`'s children.

use rsx::prelude::*;
use rsx::Div;

use super::icon::{Icon, IconName};
use super::theme::theme;
use super::utils::Siblings;

#[derive(Clone, Copy)]
struct BreadcrumbContext {
    entries: Siblings,
    separator: Option<ChildrenFn>,
}

impl BreadcrumbContext {
    /// Registers an entry; returns the separator to draw before it (tracked).
    fn entry(self) -> impl Fn() -> Option<Element> + Copy {
        let id = self.entries.register();
        move || {
            (!self.entries.is_first(id)).then(|| match self.separator {
                Some(separator) => separator.render(),
                None => <Icon name={IconName::ChevronRight} class="size-3.5" />,
            })
        }
    }
}

#[component]
pub fn Breadcrumb(
    /// Replace the default chevron separator: `separator={|| <span>/</span>}`.
    separator: Option<ChildrenFn>,
    /// `BreadcrumbItem`s and `BreadcrumbEllipsis`es.
    children: Option<ChildrenFn>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let context = BreadcrumbContext { entries: Siblings::new(), separator };
    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div
        class="flex flex-wrap items-center gap-2.5 text-sm leading-[1.25rem]"
        text_color={t.get().muted_foreground}
        {...attrs}
    >
        <Provider value={context}>{children}</Provider>
    </div>
}

/// A link (with `onClick`), plain text, or the current page (`current`).
/// Children replace the label, e.g. a dropdown menu.
#[component]
pub fn BreadcrumbItem(
    label: Option<Accessor<String>>,
    /// The current page (`BreadcrumbPage`).
    current: bool,
    on_click: Option<Callback<ClickEvent>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let separator = expect_context::<BreadcrumbContext>().entry();
    let t = theme();
    let hovered = signal(false);
    let link = on_click.is_some() && !current;
    <>
        {separator}
        <div
            class:cursor-pointer={link}
            bind={move |el: Div| if current || (link && hovered.get()) { el.text_color(t.get().foreground) } else { el }}
            onHover={move |h: &bool| hovered.set(*h)}
            onClick={move |e: &ClickEvent| {
                if let Some(f) = on_click.filter(|_| link) {
                    f.call(e)
                }
            }}
            {...attrs}
        >
            {label}
            {children}
        </div>
    </>
}

/// `BreadcrumbEllipsis` — collapsed items.
#[component]
pub fn BreadcrumbEllipsis(attrs: Attributes) -> impl IntoElement {
    let separator = expect_context::<BreadcrumbContext>().entry();
    <>
        {separator}
        <div class="flex h-9 w-5 items-center justify-center" {...attrs}>
            <Icon name={IconName::Ellipsis} />
        </div>
    </>
}

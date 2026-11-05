//! A responsive table component.
//!
//! Cells share the row width equally by default; give a column a fixed size
//! with `width={px(100.)}` on both its `TableHead` and `TableCell`s.
//!
//! ```ignore
//! <Table caption="A list of your recent invoices.">
//!     <TableHeader>
//!         <TableRow>
//!             <TableHead text="Invoice" width={px(100.)} />
//!             <TableHead text="Amount" align_right />
//!         </TableRow>
//!     </TableHeader>
//!     <TableBody>
//!         {invoices.iter().map(|inv| <TableRow>
//!             <TableCell width={px(100.)}>{inv.id}</TableCell>
//!             <TableCell align_right>{inv.amount}</TableCell>
//!         </TableRow>)}
//!     </TableBody>
//! </Table>
//! ```

use rsx::prelude::*;
use rsx::Div;

use super::theme::{theme, ColorExt};
use super::utils::Siblings;

/// Which part of the table a row is in; sets the row's defaults.
#[derive(Clone, Copy)]
enum Section {
    Header,
    /// The last row has no border.
    Body(Siblings),
    Footer,
}

#[component]
pub fn Table(
    caption: Option<Accessor<String>>,
    /// `TableHeader`, `TableBody` and `TableFooter`.
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    <div class="flex flex-col w-full text-sm leading-[1.25rem]" {...attrs}>
        {children}
        {caption.map(|caption| <div class="mt-4 w-full text-center" text_color={t.get().muted_foreground}>{caption}</div>)}
    </div>
}

/// The header rows (`<thead>`).
#[component]
pub fn TableHeader(children: Option<ChildrenFn>, attrs: Attributes) -> impl IntoElement {
    <div class="flex flex-col" {...attrs}>
        <Provider value={Section::Header}>{children}</Provider>
    </div>
}

/// The body rows (`<tbody>`); the last row has no border.
#[component]
pub fn TableBody(children: Option<ChildrenFn>, attrs: Attributes) -> impl IntoElement {
    let section = Section::Body(Siblings::new());
    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div class="flex flex-col" {...attrs}>
        <Provider value={section}>{children}</Provider>
    </div>
}

/// The footer rows (`<tfoot>`).
#[component]
pub fn TableFooter(children: Option<ChildrenFn>, attrs: Attributes) -> impl IntoElement {
    let t = theme();
    // Not inside a `move ||` (which would rebuild the rows on theme changes),
    // so the colors are bindings and win over a caller's static classes (rsx#10).
    <div class="flex flex-col border-t font-medium" border_color={t.get().border} bg={t.get().muted.opacity(0.5)} {...attrs}>
        <Provider value={Section::Footer}>{children}</Provider>
    </div>
}

#[component]
pub fn TableRow(
    /// `data-state="selected"`.
    selected: Option<Accessor<bool>>,
    /// Border below the row (default: yes, except the last body row and footer rows).
    bordered: Option<bool>,
    /// Highlight on hover (default: yes, except header rows).
    hoverable: Option<bool>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let section = use_context::<Section>();
    let id = match section {
        Some(Section::Body(rows)) => Some(rows.register()),
        _ => None,
    };
    let hoverable = hoverable.unwrap_or(!matches!(section, Some(Section::Header)));
    let bordered = move || match (bordered, section, id) {
        (Some(b), _, _) => b,
        (None, Some(Section::Footer), _) => false,
        (None, Some(Section::Body(rows)), Some(id)) => !rows.is_last(id),
        _ => true,
    };
    let hovered = signal(false);
    move || {
        let theme = t.get();
        let selected = selected.map(|s| s.get()).unwrap_or(false);
        let bg = if selected { theme.muted } else { Color::TRANSPARENT };
        // Hover is a `bind` below, so it doesn't re-create the row.
        <div
            class="flex w-full items-center"
            class:border-b={bordered()}
            border_color={theme.border}
            bg={bg}
            {...&attrs}
            bind={move |el: Div| if hoverable && hovered.get() && !selected { el.bg(t.get().muted.opacity(0.5)) } else { el }}
            onHover={move |h: &bool| hovered.set(*h)}
        >
            {children}
        </div>
    }
}

/// The classes shared by heads and cells.
fn cell_class(width: Option<Length>, right: bool) -> Attributes {
    // rsx#9: no `whitespace-nowrap`.
    Attributes::new()
        .class("flex flex-col justify-center min-w-0 overflow-hidden px-2")
        .class(if right { "text-right items-end" } else { "items-start" })
        .with(move |el| match width {
            Some(width) => el.flex_none().w(width),
            None => el.flex_1(),
        })
}

#[component]
pub fn TableHead(
    text: Option<Accessor<String>>,
    /// A fixed column width (also give it to the column's cells).
    width: Option<Length>,
    align_right: bool,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let base = cell_class(width, align_right);
    <div {...base} class="h-10 font-medium" text_color={t.get().foreground} {...attrs}>
        {text}
        {children}
    </div>
}

#[component]
pub fn TableCell(
    /// A fixed column width (also give it to the column's head).
    width: Option<Length>,
    align_right: bool,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let base = cell_class(width, align_right);
    <div {...base} class="py-2 min-h-10" {...attrs}>
        {children}
    </div>
}

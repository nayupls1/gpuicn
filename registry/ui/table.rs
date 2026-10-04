//! A responsive table component.
//!
//! Cells share the row width equally by default; give a column a fixed size
//! with `.width(px(100.))` on both its `TableHead` and `TableCell`s.
//!
//! ```ignore
//! Table::new()
//!     .caption("A list of your recent invoices.")
//!     .header(TableHeader::new().row(
//!         TableRow::new().child(TableHead::new("Invoice").width(px(100.))).child(TableHead::new("Amount").align_right()),
//!     ))
//!     .body(TableBody::new().rows(invoices.iter().map(|inv| {
//!         TableRow::new()
//!             .child(TableCell::new().width(px(100.)).child(inv.id.clone()))
//!             .child(TableCell::new().align_right().child(inv.amount.clone()))
//!     })))
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, Pixels, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};

use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Table {
    header: Option<TableHeader>,
    body: Option<TableBody>,
    footer: Option<TableFooter>,
    caption: Option<SharedString>,
    style: StyleRefinement,
}

crate::impl_styled!(Table, TableHeader, TableBody, TableFooter);
crate::impl_styled_parent!(TableRow, TableHead, TableCell);

impl Table {
    pub fn new() -> Self {
        Self { header: None, body: None, footer: None, caption: None, style: StyleRefinement::default() }
    }

    pub fn header(mut self, header: TableHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn body(mut self, body: TableBody) -> Self {
        self.body = Some(body);
        self
    }

    pub fn footer(mut self, footer: TableFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn caption(mut self, caption: impl Into<SharedString>) -> Self {
        self.caption = Some(caption.into());
        self
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let muted = cx.theme().muted_foreground;
        div()
            .flex()
            .flex_col()
            .w_full()
            .text_sm()
            .refine_style(&self.style)
            .children(self.header)
            .children(self.body)
            .children(self.footer)
            .children(self.caption.map(|caption| {
                div().mt_4().w_full().text_center().text_color(muted).child(caption)
            }))
    }
}

macro_rules! row_container {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(IntoElement)]
        pub struct $name {
            rows: Vec<TableRow>,
            style: StyleRefinement,
        }

        impl $name {
            pub fn new() -> Self {
                Self { rows: Vec::new(), style: StyleRefinement::default() }
            }

            pub fn row(mut self, row: TableRow) -> Self {
                self.rows.push(row);
                self
            }

            pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow>) -> Self {
                self.rows.extend(rows);
                self
            }
        }
    };
}

row_container!(TableHeader, "The header rows (`<thead>`).");
row_container!(TableBody, "The body rows (`<tbody>`); the last row has no border.");
row_container!(TableFooter, "The footer rows (`<tfoot>`).");

impl RenderOnce for TableHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .refine_style(&self.style)
            .children(self.rows.into_iter().map(|row| row.hoverable(false)))
    }
}

impl RenderOnce for TableBody {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let count = self.rows.len();
        div()
            .flex()
            .flex_col()
            .refine_style(&self.style)
            .children(
                self.rows
                    .into_iter()
                    .enumerate()
                    .map(move |(ix, row)| row.bordered(ix + 1 < count)),
            )
    }
}

impl RenderOnce for TableFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_col()
            .border_t_1()
            .border_color(theme.border)
            .bg(theme.muted.opacity(0.5))
            .font_weight(FontWeight::MEDIUM)
            .refine_style(&self.style)
            .children(self.rows.into_iter().map(|row| row.bordered(false)))
    }
}

#[derive(IntoElement)]
pub struct TableRow {
    selected: bool,
    bordered: bool,
    hoverable: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl TableRow {
    pub fn new() -> Self {
        Self { selected: false, bordered: true, hoverable: true, children: Vec::new(), style: StyleRefinement::default() }
    }

    /// `data-state="selected"`.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
        self
    }
}

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let hover = theme.muted.opacity(0.5);
        div()
            .flex()
            .w_full()
            .items_center()
            .when(self.bordered, |this| this.border_b_1().border_color(theme.border))
            .when(self.selected, |this| this.bg(theme.muted))
            .when(self.hoverable && !self.selected, |this| this.hover(move |s| s.bg(hover)))
            .refine_style(&self.style)
            .children(self.children)
    }
}

fn cell_base(width: Option<Pixels>, right: bool) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .justify_center()
        .min_w_0()
        .overflow_hidden()
        .whitespace_nowrap()
        .px_2()
        .map(|this| match width {
            Some(width) => this.flex_none().w(width),
            None => this.flex_1().flex_basis(gpui::relative(0.)),
        })
        .when(right, |this| this.text_right().items_end())
}

#[derive(IntoElement)]
pub struct TableHead {
    width: Option<Pixels>,
    right: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl TableHead {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self::empty().child(text.into())
    }

    pub fn empty() -> Self {
        Self { width: None, right: false, children: Vec::new(), style: StyleRefinement::default() }
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn align_right(mut self) -> Self {
        self.right = true;
        self
    }
}

impl RenderOnce for TableHead {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        cell_base(self.width, self.right)
            .h_10()
            .text_color(cx.theme().foreground)
            .font_weight(FontWeight::MEDIUM)
            .refine_style(&self.style)
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct TableCell {
    width: Option<Pixels>,
    right: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl TableCell {
    pub fn new() -> Self {
        Self { width: None, right: false, children: Vec::new(), style: StyleRefinement::default() }
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn align_right(mut self) -> Self {
        self.right = true;
        self
    }
}

impl RenderOnce for TableCell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        cell_base(self.width, self.right)
            .py_2()
            .min_h_10()
            .refine_style(&self.style)
            .children(self.children)
    }
}

//! Powerful table and datagrids: sorting, filtering, row selection, column
//! visibility and pagination on top of `table`.
//!
//! ```ignore
//! DataTable::new("payments", payments.clone())
//!     .selectable()
//!     .filter("Filter emails...", |p: &Payment| p.email.to_string())
//!     .column(DataTableColumn::new("status", "Status", |p: &Payment, _, _| p.status.clone()))
//!     .column(
//!         DataTableColumn::new("email", "Email", |p: &Payment, _, _| p.email.clone())
//!             .sortable_by(|p| p.email.clone()),
//!     )
//!     .column(
//!         DataTableColumn::new("amount", "Amount", |p: &Payment, _, _| format!("${:.2}", p.amount))
//!             .sortable_by(|p| (p.amount * 100.) as i64)
//!             .align_right(),
//!     )
//!     .row_actions(|p: &Payment| vec![MenuItem::new("Copy payment ID").into()])
//!     .page_size(5)
//! ```

use gpui::{
    AnyElement, App, Component, ElementId, IntoElement, Pixels, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, px,
};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::rc::Rc;

use super::button::{Button, ButtonSize};
use super::checkbox::Checkbox;
use super::dropdown_menu::{DropdownMenu, MenuBuilder, MenuEntry, MenuItem};
use super::icon::IconName;
use super::input::Input;
use super::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use super::theme::ActiveTheme;
use super::utils::{Align, StyledExt};

type CellRenderer<T> = Rc<dyn Fn(&T, &mut Window, &mut App) -> AnyElement>;
type Comparator<T> = Rc<dyn Fn(&T, &T) -> Ordering>;

pub struct DataTableColumn<T> {
    id: SharedString,
    header: SharedString,
    cell: CellRenderer<T>,
    sort: Option<Comparator<T>>,
    width: Option<Pixels>,
    align_right: bool,
    hideable: bool,
}

impl<T: 'static> DataTableColumn<T> {
    pub fn new<E: IntoElement>(
        id: impl Into<SharedString>,
        header: impl Into<SharedString>,
        cell: impl Fn(&T, &mut Window, &mut App) -> E + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            cell: Rc::new(move |row, window, cx| cell(row, window, cx).into_any_element()),
            sort: None,
            width: None,
            align_right: false,
            hideable: true,
        }
    }

    /// Make the header clickable to sort by `key`.
    pub fn sortable_by<K: Ord>(mut self, key: impl Fn(&T) -> K + 'static) -> Self {
        self.sort = Some(Rc::new(move |a, b| key(a).cmp(&key(b))));
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn align_right(mut self) -> Self {
        self.align_right = true;
        self
    }

    /// Whether the column can be hidden from the "Columns" menu.
    pub fn hideable(mut self, hideable: bool) -> Self {
        self.hideable = hideable;
        self
    }
}

#[derive(Default)]
struct TableState {
    sort: Option<(usize, bool)>,
    page: usize,
    selected: HashSet<usize>,
    hidden: HashSet<SharedString>,
}

pub struct DataTable<T: 'static> {
    id: ElementId,
    rows: Rc<Vec<T>>,
    columns: Vec<DataTableColumn<T>>,
    filter: Option<(SharedString, Rc<dyn Fn(&T) -> String>)>,
    selectable: bool,
    page_size: usize,
    row_actions: Option<Rc<dyn Fn(&T) -> Vec<MenuEntry>>>,
    empty: SharedString,
    style: StyleRefinement,
}

impl<T: 'static> DataTable<T> {
    pub fn new(id: impl Into<ElementId>, rows: impl Into<Rc<Vec<T>>>) -> Self {
        Self {
            id: id.into(),
            rows: rows.into(),
            columns: Vec::new(),
            filter: None,
            selectable: false,
            page_size: 10,
            row_actions: None,
            empty: "No results.".into(),
            style: StyleRefinement::default(),
        }
    }

    pub fn column(mut self, column: DataTableColumn<T>) -> Self {
        self.columns.push(column);
        self
    }

    /// A filter input matching rows by the text `key` returns.
    pub fn filter(mut self, placeholder: impl Into<SharedString>, key: impl Fn(&T) -> String + 'static) -> Self {
        self.filter = Some((placeholder.into(), Rc::new(key)));
        self
    }

    /// Add a checkbox column for row selection.
    pub fn selectable(mut self) -> Self {
        self.selectable = true;
        self
    }

    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size.max(1);
        self
    }

    /// A "…" menu at the end of each row.
    pub fn row_actions(mut self, actions: impl Fn(&T) -> Vec<MenuEntry> + 'static) -> Self {
        self.row_actions = Some(Rc::new(actions));
        self
    }

    pub fn empty(mut self, empty: impl Into<SharedString>) -> Self {
        self.empty = empty.into();
        self
    }
}

impl<T: 'static> gpui::Styled for DataTable<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: 'static> IntoElement for DataTable<T> {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl<T: 'static> RenderOnce for DataTable<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| TableState::default());

        // Filter
        let filter_input = self.filter.as_ref().map(|(placeholder, _)| Input::new("data-table-filter").placeholder(placeholder.clone()).max_w(px(384.)));
        let query = match &filter_input {
            Some(input) => input.state(window, cx).read(cx).text().to_lowercase(),
            None => String::new(),
        };
        let mut visible: Vec<usize> = (0..self.rows.len())
            .filter(|ix| match &self.filter {
                Some((_, key)) if !query.is_empty() => key(&self.rows[*ix]).to_lowercase().contains(&query),
                _ => true,
            })
            .collect();

        // Sort
        let (sort, page, selected, hidden) = {
            let s = state.read(cx);
            (s.sort, s.page, s.selected.clone(), s.hidden.clone())
        };
        if let Some((col, desc)) = sort
            && let Some(cmp) = self.columns.get(col).and_then(|c| c.sort.clone())
        {
            visible.sort_by(|a, b| {
                let ord = cmp(&self.rows[*a], &self.rows[*b]);
                if desc { ord.reverse() } else { ord }
            });
        }

        // Paginate
        let total = visible.len();
        let pages = total.div_ceil(self.page_size).max(1);
        let page = page.min(pages - 1);
        let page_rows: Vec<usize> = visible.iter().skip(page * self.page_size).take(self.page_size).copied().collect();

        let theme = cx.theme().clone();
        let muted = theme.muted_foreground;
        let columns: Vec<(usize, &DataTableColumn<T>)> =
            self.columns.iter().enumerate().filter(|(_, c)| !hidden.contains(&c.id)).collect();

        // Header row
        let mut header = TableRow::new();
        if self.selectable {
            let all = !page_rows.is_empty() && page_rows.iter().all(|ix| selected.contains(ix));
            let some = page_rows.iter().any(|ix| selected.contains(ix));
            let state = state.clone();
            let page_rows = page_rows.clone();
            header = header.child(TableHead::empty().width(px(40.)).child(
                Checkbox::new("select-all").checked(all).indeterminate(some && !all).on_change(move |checked, _, cx| {
                    state.update(cx, |s, cx| {
                        for ix in &page_rows {
                            if *checked { s.selected.insert(*ix); } else { s.selected.remove(ix); }
                        }
                        cx.notify();
                    })
                }),
            ));
        }
        for (col_ix, column) in &columns {
            let mut head = TableHead::empty();
            if let Some(width) = column.width {
                head = head.width(width);
            }
            if column.align_right {
                head = head.align_right();
            }
            head = if column.sort.is_some() {
                let state = state.clone();
                let col_ix = *col_ix;
                let icon = match sort {
                    Some((c, false)) if c == col_ix => IconName::ArrowUp,
                    Some((c, true)) if c == col_ix => IconName::ArrowDown,
                    _ => IconName::ArrowUpDown,
                };
                head.child(
                    Button::new(("sort", col_ix))
                        .ghost()
                        .size(ButtonSize::Sm)
                        .ml(px(-12.))
                        .label(column.header.clone())
                        .icon_end(icon)
                        .on_click(move |_, _, cx| {
                            state.update(cx, |s, cx| {
                                s.sort = match s.sort {
                                    Some((c, false)) if c == col_ix => Some((col_ix, true)),
                                    Some((c, true)) if c == col_ix => None,
                                    _ => Some((col_ix, false)),
                                };
                                cx.notify();
                            })
                        }),
                )
            } else {
                head.child(column.header.clone())
            };
            header = header.child(head);
        }
        if self.row_actions.is_some() {
            header = header.child(TableHead::empty().width(px(48.)));
        }

        // Body rows
        let mut body = TableBody::new();
        if page_rows.is_empty() {
            body = body.row(
                TableRow::new()
                    .hoverable(false)
                    .child(TableCell::new().h_24().items_center().text_color(muted).child(self.empty.clone())),
            );
        }
        for row_ix in &page_rows {
            let row = &self.rows[*row_ix];
            let is_selected = selected.contains(row_ix);
            let mut tr = TableRow::new().selected(is_selected);
            if self.selectable {
                let state = state.clone();
                let row_ix = *row_ix;
                tr = tr.child(TableCell::new().width(px(40.)).child(
                    Checkbox::new(("select-row", row_ix)).checked(is_selected).on_change(move |checked, _, cx| {
                        state.update(cx, |s, cx| {
                            if *checked { s.selected.insert(row_ix); } else { s.selected.remove(&row_ix); }
                            cx.notify();
                        })
                    }),
                ));
            }
            for (_, column) in &columns {
                let mut cell = TableCell::new();
                if let Some(width) = column.width {
                    cell = cell.width(width);
                }
                if column.align_right {
                    cell = cell.align_right();
                }
                tr = tr.child(cell.child((column.cell)(row, window, cx)));
            }
            if let Some(actions) = &self.row_actions {
                let mut menu = DropdownMenu::new(("row-actions", *row_ix))
                    .align(Align::End)
                    .trigger(Button::new(("row-actions-trigger", *row_ix)).ghost().size(ButtonSize::IconSm).icon(IconName::Ellipsis));
                for entry in actions(row) {
                    menu = menu.entry(entry);
                }
                tr = tr.child(TableCell::new().width(px(48.)).child(menu));
            }
            body = body.row(tr);
        }

        // Toolbar: filter + column visibility
        let hideable: Vec<(SharedString, SharedString)> =
            self.columns.iter().filter(|c| c.hideable).map(|c| (c.id.clone(), c.header.clone())).collect();
        let columns_menu = (!hideable.is_empty()).then(|| {
            let mut menu = DropdownMenu::new("columns-menu")
                .align(Align::End)
                .trigger(Button::new("columns-trigger").outline().label("Columns").icon_end(IconName::ChevronDown));
            for (id, header) in hideable {
                let state = state.clone();
                let visible = !hidden.contains(&id);
                menu = menu.item(MenuItem::checkbox(header, visible).keep_open().on_select(move |_, cx| {
                    state.update(cx, |s, cx| {
                        if !s.hidden.remove(&id) {
                            s.hidden.insert(id.clone());
                        }
                        cx.notify();
                    })
                }));
            }
            menu
        });

        let (prev_state, next_state) = (state.clone(), state.clone());
        let selected_count = selected.len();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w_full()
            .refine_style(&self.style)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .py_4()
                    .children(filter_input)
                    .child(div().flex_1())
                    .children(columns_menu),
            )
            .child(
                div()
                    .overflow_hidden()
                    .rounded(theme.radius_md())
                    .border_1()
                    .border_color(theme.border)
                    .child(Table::new().header(TableHeader::new().row(header)).body(body)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .gap_2()
                    .py_4()
                    .child(
                        div().flex_1().min_w_0().text_sm().text_color(muted).child(if self.selectable {
                            format!("{selected_count} of {total} row(s) selected.")
                        } else {
                            format!("Page {} of {pages}", page + 1)
                        }),
                    )
                    .child(
                        Button::new("prev-page").outline().size(ButtonSize::Sm).label("Previous").disabled(page == 0).on_click(
                            move |_, _, cx| {
                                prev_state.update(cx, |s, cx| {
                                    s.page = page.saturating_sub(1);
                                    cx.notify();
                                })
                            },
                        ),
                    )
                    .child(
                        Button::new("next-page").outline().size(ButtonSize::Sm).label("Next").disabled(page + 1 >= pages).on_click(
                            move |_, _, cx| {
                                next_state.update(cx, |s, cx| {
                                    s.page = page + 1;
                                    cx.notify();
                                })
                            },
                        ),
                    ),
            )
    }
}

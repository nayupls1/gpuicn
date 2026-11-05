//! Powerful table and datagrids: sorting, filtering, row selection, column
//! visibility and pagination on top of `table`.
//!
//! Like gpuicn's, it's a builder (generic over the row type), used as a
//! child in markup:
//!
//! ```ignore
//! {DataTable::new(payments.clone())
//!     .class("w-[720px]")
//!     .selectable()
//!     .filter("Filter emails...", |p: &Payment| p.email.to_string())
//!     .column(DataTableColumn::new("status", "Status", |p: &Payment| p.status.into()))
//!     .column(DataTableColumn::new("email", "Email", |p: &Payment| p.email.into()).sortable_by(|p| p.email.to_lowercase()))
//!     .column(
//!         DataTableColumn::new("amount", "Amount", |p: &Payment| format!("${:.2}", p.amount).into())
//!             .align_right(),
//!     )
//!     .row_actions(|p: &Payment| vec![MenuItem::new("Copy payment ID").into()])
//!     .page_size(5)}
//! ```

use std::cmp::Ordering;
use std::collections::HashSet;
use std::rc::Rc;

use rsx::prelude::*;

use super::button::{Button, ButtonSize};
use super::checkbox::Checkbox;
use super::dropdown_menu::{DropdownMenu, Menu, MenuEntry, MenuItem};
use super::icon::IconName;
use super::input::Input;
use super::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use super::theme::theme;
use super::utils::Align;

type CellRenderer<T> = Rc<dyn Fn(&T) -> Element>;
type Comparator<T> = Rc<dyn Fn(&T, &T) -> Ordering>;

pub struct DataTableColumn<T> {
    id: SharedString,
    header: SharedString,
    cell: CellRenderer<T>,
    sort: Option<Comparator<T>>,
    width: Option<Length>,
    align_right: bool,
    hideable: bool,
}

impl<T> Clone for DataTableColumn<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            header: self.header.clone(),
            cell: self.cell.clone(),
            sort: self.sort.clone(),
            width: self.width,
            align_right: self.align_right,
            hideable: self.hideable,
        }
    }
}

impl<T: 'static> DataTableColumn<T> {
    /// A column whose cells `cell` draws from each row (anything that is an
    /// element: text, markup, a component).
    pub fn new<E: IntoElement>(id: impl Into<SharedString>, header: impl Into<SharedString>, cell: impl Fn(&T) -> E + 'static) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            cell: Rc::new(move |row| cell(row).into_element()),
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

    pub fn width(mut self, width: impl Into<Length>) -> Self {
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

pub struct DataTable<T: 'static> {
    rows: Rc<Vec<T>>,
    columns: Vec<DataTableColumn<T>>,
    filter: Option<(SharedString, Rc<dyn Fn(&T) -> String>)>,
    selectable: bool,
    page_size: usize,
    row_actions: Option<Rc<dyn Fn(&T) -> Vec<MenuEntry>>>,
    on_selection_change: Option<Rc<dyn Fn(&Vec<usize>)>>,
    empty: SharedString,
    attrs: Attributes,
}

impl<T: 'static> DataTable<T> {
    pub fn new(rows: impl Into<Rc<Vec<T>>>) -> Self {
        Self {
            rows: rows.into(),
            columns: Vec::new(),
            filter: None,
            selectable: false,
            page_size: 10,
            row_actions: None,
            on_selection_change: None,
            empty: "No results.".into(),
            attrs: Attributes::new(),
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

    /// Called with the selected rows (indices into `rows`, ascending) when
    /// the selection changes.
    pub fn on_selection_change(mut self, f: impl Fn(&Vec<usize>) + 'static) -> Self {
        self.on_selection_change = Some(Rc::new(f));
        self
    }

    pub fn empty(mut self, empty: impl Into<SharedString>) -> Self {
        self.empty = empty.into();
        self
    }

    /// Classes for the outer element (`.class("w-[720px]")`).
    pub fn class(mut self, classes: &'static str) -> Self {
        self.attrs = self.attrs.class(classes);
        self
    }

    /// Any other attributes for the outer element (`ref`, style methods).
    pub fn attrs(mut self, attrs: Attributes) -> Self {
        self.attrs = self.attrs.spread(attrs);
        self
    }
}

impl<T: 'static> IntoElement for DataTable<T> {
    fn into_element(self) -> Element {
        render(self)
    }
}

fn render<T: 'static>(table: DataTable<T>) -> Element {
    let DataTable { rows, columns, filter, selectable, page_size, row_actions, on_selection_change, empty, attrs } = table;
    let columns = Rc::new(columns);
    let t = theme();

    let query = signal(String::new());
    // `(column, descending)`
    let sort = signal(None::<(usize, bool)>);
    let page = signal(0usize);
    let selected = signal(HashSet::<usize>::new());
    let hidden = signal(HashSet::<SharedString>::new());

    let on_selection_change = StoredValue::new(on_selection_change);
    let set_selected = move |f: &dyn Fn(&mut HashSet<usize>)| {
        selected.update(|s| f(s));
        if let Some(cb) = on_selection_change.get_value() {
            let mut ixs: Vec<usize> = selected.with_untracked(|s| s.iter().copied().collect());
            ixs.sort_unstable();
            cb(&ixs);
        }
    };

    // Filtered, then sorted row indices.
    let filter_key = filter.as_ref().map(|(_, key)| key.clone());
    let visible = memo({
        let (rows, columns) = (rows.clone(), columns.clone());
        move || {
            let q = query.get().to_lowercase();
            let mut visible: Vec<usize> = (0..rows.len())
                .filter(|ix| match &filter_key {
                    Some(key) if !q.is_empty() => key(&rows[*ix]).to_lowercase().contains(&q),
                    _ => true,
                })
                .collect();
            if let Some((col, desc)) = sort.get() {
                if let Some(cmp) = columns.get(col).and_then(|c| c.sort.clone()) {
                    visible.sort_by(|a, b| {
                        let ord = cmp(&rows[*a], &rows[*b]);
                        if desc { ord.reverse() } else { ord }
                    });
                }
            }
            visible
        }
    });
    let pages = move || visible.with(|v| v.len()).div_ceil(page_size).max(1);
    let current_page = move || page.get().min(pages() - 1);
    let page_rows = memo(move || {
        let start = current_page() * page_size;
        visible.with(|v| v.iter().skip(start).take(page_size).copied().collect::<Vec<usize>>())
    });
    // The columns shown, with their index in `columns`.
    let shown = {
        let columns = columns.clone();
        move || -> Vec<(usize, DataTableColumn<T>)> {
            let hidden = hidden.get();
            columns.iter().cloned().enumerate().filter(|(_, c)| !hidden.contains(&c.id)).collect()
        }
    };

    // Header row: select-all, sortable headers, the actions column.
    let header = {
        let shown = shown.clone();
        let has_actions = row_actions.is_some();
        move || {
            let select_all = selectable.then(|| {
                let all = move || page_rows.with(|r| !r.is_empty() && selected.with(|s| r.iter().all(|ix| s.contains(ix))));
                let some = move || page_rows.with(|r| selected.with(|s| r.iter().any(|ix| s.contains(ix))));
                <TableHead width={px(40.)}>
                    <Checkbox
                        checked={all}
                        indeterminate={move || some() && !all()}
                        onChange={move |checked: &bool| {
                            let (checked, rows) = (*checked, page_rows.get_untracked());
                            set_selected(&|s| for ix in &rows {
                                if checked { s.insert(*ix); } else { s.remove(ix); }
                            });
                        }}
                    />
                </TableHead>
            });
            let heads = shown()
                .into_iter()
                .map(|(col_ix, column)| {
                    let content = if column.sort.is_some() {
                        let icon = match sort.get() {
                            Some((c, false)) if c == col_ix => IconName::ArrowUp,
                            Some((c, true)) if c == col_ix => IconName::ArrowDown,
                            _ => IconName::ArrowUpDown,
                        };
                        // `-ml-3`/`-mr-3` line the label up with the cell text below it.
                        <Button
                            ghost
                            size={ButtonSize::Sm}
                            class={if column.align_right { "-mr-3" } else { "-ml-3" }}
                            label={column.header.to_string()}
                            icon_end={icon}
                            onClick={move |_| sort.update(|s| {
                                *s = match *s {
                                    Some((c, false)) if c == col_ix => Some((col_ix, true)),
                                    Some((c, true)) if c == col_ix => None,
                                    _ => Some((col_ix, false)),
                                }
                            })}
                        />
                    } else {
                        column.header.clone().into_element()
                    };
                    <TableHead width={column.width} align_right={column.align_right}>{content}</TableHead>
                })
                .collect::<Vec<_>>();
            <TableRow>
                {select_all}
                {heads}
                {has_actions.then(|| <TableHead width={px(48.)} />)}
            </TableRow>
        }
    };

    // Body rows for the current page.
    let body = {
        let (rows, empty) = (rows.clone(), empty.clone());
        move || {
            let page_rows = page_rows.get();
            if page_rows.is_empty() {
                let text = empty.clone();
                return vec![<TableRow hoverable={false}>
                    <TableCell class="h-24 items-center" text_color={t.get().muted_foreground}>{text.clone()}</TableCell>
                </TableRow>];
            }
            let shown = shown();
            page_rows
                .into_iter()
                .map(|row_ix| {
                    let row = &rows[row_ix];
                    let is_selected = move || selected.with(|s| s.contains(&row_ix));
                    let check = selectable.then(|| {
                        <TableCell width={px(40.)}>
                            <Checkbox
                                checked={is_selected}
                                onChange={move |checked: &bool| {
                                    let checked = *checked;
                                    set_selected(&|s| {
                                        if checked { s.insert(row_ix); } else { s.remove(&row_ix); }
                                    });
                                }}
                            />
                        </TableCell>
                    });
                    let cells = shown
                        .iter()
                        .map(|(_, column)| {
                            let content = (column.cell)(row);
                            <TableCell width={column.width} align_right={column.align_right}>{content}</TableCell>
                        })
                        .collect::<Vec<_>>();
                    let actions = row_actions.as_ref().map(|actions| {
                        let menu = Menu::from(actions(row));
                        <TableCell width={px(48.)}>
                            <DropdownMenu
                                align={Align::End}
                                trigger={<Button ghost size={ButtonSize::IconSm} icon={IconName::Ellipsis} />}
                                menu={menu}
                            />
                        </TableCell>
                    });
                    <TableRow selected={is_selected}>
                        {check}
                        {cells}
                        {actions}
                    </TableRow>
                })
                .collect::<Vec<_>>()
        }
    };

    // Toolbar: filter + column visibility.
    let filter_input = filter.map(|(placeholder, _)| <Input class="max-w-[384px]" placeholder={placeholder.to_string()} text={query} />);
    let hideable: Rc<Vec<(SharedString, SharedString)>> =
        Rc::new(columns.iter().filter(|c| c.hideable).map(|c| (c.id.clone(), c.header.clone())).collect());
    let columns_menu = (!hideable.is_empty()).then(|| {
        let menu = move || {
            hideable
                .iter()
                .map(|(id, header)| {
                    let id = id.clone();
                    let visible = !hidden.with(|h| h.contains(&id));
                    MenuItem::checkbox(header.clone(), visible)
                        .keep_open()
                        .on_select(move || hidden.update(|h| {
                            if !h.remove(&id) {
                                h.insert(id.clone());
                            }
                        }))
                        .into()
                })
                .collect::<Vec<MenuEntry>>()
        };
        <DropdownMenu
            align={Align::End}
            trigger={<Button outline label="Columns" icon_end={IconName::ChevronDown} />}
            menu={menu}
        />
    });

    let status = move || {
        if selectable {
            format!("{} of {} row(s) selected.", selected.with(|s| s.len()), visible.with(|v| v.len()))
        } else {
            format!("Page {} of {}", current_page() + 1, pages())
        }
    };

    <div class="flex flex-col w-full" {...attrs}>
        <div class="flex items-center gap-2 py-4">
            {filter_input}
            <div class="flex-1" />
            {columns_menu}
        </div>
        <div class="overflow-hidden border" corner_radius={t.get().radius_md()} border_color={t.get().border}>
            <Table>
                <TableHeader>{header}</TableHeader>
                <TableBody>{body}</TableBody>
            </Table>
        </div>
        <div class="flex items-center justify-end gap-2 py-4">
            <div class="flex-1 min-w-0 text-sm leading-[1.25rem]" text_color={t.get().muted_foreground}>{status}</div>
            <Button
                outline
                size={ButtonSize::Sm}
                label="Previous"
                disabled={move || current_page() == 0}
                onClick={move |_| page.set(untrack(current_page).saturating_sub(1))}
            />
            <Button
                outline
                size={ButtonSize::Sm}
                label="Next"
                disabled={move || current_page() + 1 >= pages()}
                onClick={move |_| page.set(untrack(current_page) + 1)}
            />
        </div>
    </div>
}

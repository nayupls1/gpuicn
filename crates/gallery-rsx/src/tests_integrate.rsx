#![cfg(test)]
//! Interaction tests for the integrated pieces: the data table (filtering,
//! sorting, row selection, pagination) and the gallery shell.

use std::rc::Rc;

use gpui::{Modifiers, TestAppContext};
use rsx::prelude::*;

use crate::demos::data_table::{payments, Payment};
use crate::tests::{mount, settle};
use crate::ui::data_table::{DataTable, DataTableColumn};

fn point(x: f32, y: f32) -> gpui::Point<gpui::Pixels> {
    gpui::point(gpui::px(x), gpui::px(y))
}

/// The demo's table, with a `NodeRef` on every email cell (by row index)
/// and the selection reported to `selection`.
fn table(cells: Rc<Vec<NodeRef>>, selection: Signal<Vec<usize>>, root: NodeRef) -> Element {
    let refs = cells.clone();
    let rows = Rc::new(payments());
    let index = rows.clone();
    let row_ix = move |p: &Payment| index.iter().position(|r| r.id == p.id).unwrap();
    <div class="p-8 w-[800px]" ref={root}>
        {DataTable::new(rows)
            .selectable()
            .page_size(5)
            .filter("Filter emails...", |p: &Payment| p.email.to_string())
            .column(DataTableColumn::new("status", "Status", |p: &Payment| p.status).width(px(120.)))
            .column(
                DataTableColumn::new("email", "Email", move |p: &Payment| <div ref={refs[row_ix(p)]}>{p.email}</div>)
                    .sortable_by(|p| p.email.to_lowercase()),
            )
            .column(DataTableColumn::new("amount", "Amount", |p: &Payment| format!("${:.2}", p.amount)).align_right().width(px(120.)))
            .on_selection_change(move |s: &Vec<usize>| selection.set(s.clone()))}
    </div>
}

/// Which rows are on screen, by index into `payments()`.
fn shown(cells: &[NodeRef]) -> Vec<usize> {
    let mut rows: Vec<(f32, usize)> = cells.iter().enumerate().filter_map(|(ix, r)| r.bounds().map(|b| (b.origin.y, ix))).collect();
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    rows.into_iter().map(|(_, ix)| ix).collect()
}

#[gpui::test]
fn data_table_pages_sorts_filters_and_selects(cx: &mut TestAppContext) {
    let cells = cx.update(|_| Rc::new((0..payments().len()).map(|_| NodeRef::new()).collect::<Vec<_>>()));
    let selection = cx.update(|_| signal(Vec::<usize>::new()));
    let root = cx.update(|_| NodeRef::new());
    let c = cells.clone();
    let cx = mount(cx, move || table(c, selection, root));
    assert_eq!(cx.update(|_, _| shown(&cells)), vec![0, 1, 2, 3, 4], "the first page of 5");

    // Sort by email: click the "Email" header, just above the first row's email.
    let first = cx.update(|_, _| cells[0].bounds().unwrap());
    let second = cx.update(|_, _| cells[1].bounds().unwrap());
    // Rows are `pitch` apart (their height plus a 1px border); the 40px
    // header row and its border sit right above the first.
    let pitch = second.origin.y - first.origin.y;
    let row_y = first.origin.y + first.size.height / 2.0;
    let header_y = row_y - (pitch - 1.0) / 2.0 - 1.0 - 20.0;
    let header = point(first.origin.x + 10.0, header_y);
    cx.simulate_click(header, Modifiers::default());
    settle(cx);
    let mut by_email: Vec<usize> = (0..payments().len()).collect();
    by_email.sort_by_key(|ix| payments()[*ix].email.to_lowercase());
    assert_eq!(cx.update(|_, _| shown(&cells)), by_email[..5].to_vec(), "sorted ascending");
    // A second click sorts descending, a third restores the order.
    cx.simulate_click(header, Modifiers::default());
    settle(cx);
    let descending: Vec<usize> = by_email.iter().rev().copied().collect();
    assert_eq!(cx.update(|_, _| shown(&cells)), descending[..5].to_vec(), "sorted descending");
    cx.simulate_click(header, Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| shown(&cells)), vec![0, 1, 2, 3, 4]);

    // Select the first row with its checkbox (the 40px column before "Status").
    let status_left = first.origin.x - 120.0;
    let check = point(status_left - 32.0, row_y);
    cx.simulate_click(check, Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| selection.get_untracked()), vec![0]);
    // The header checkbox selects the whole page.
    cx.simulate_click(point(status_left - 32.0, header_y), Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| selection.get_untracked()), vec![0, 1, 2, 3, 4]);

    // "Next" (bottom right) shows the last two rows.
    let r = cx.update(|_, _| root.bounds().unwrap());
    let footer_y = r.origin.y + r.size.height - 32.0 - 34.0;
    cx.simulate_click(point(r.origin.x + r.size.width - 32.0 - 30.0, footer_y), Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| shown(&cells)), vec![5, 6], "the second page");

    // Typing in the filter (top left) narrows the rows to the matches.
    cx.simulate_click(point(r.origin.x + 32.0 + 40.0, r.origin.y + 32.0 + 16.0 + 18.0), Modifiers::default());
    settle(cx);
    cx.simulate_input("ken");
    settle(cx);
    assert_eq!(cx.update(|_, _| shown(&cells)), vec![0], "only ken99@example.com matches");
}

#[gpui::test]
fn gallery_reveals_the_current_page_in_the_sidebar(cx: &mut TestAppContext) {
    use crate::demos::DEMOS;
    use crate::ui::scroll_area::ScrollHandle;
    let (page, nav) = cx.update(|_| (signal(0usize), ScrollHandle::new()));
    crate::ui::utils::set_reduced_motion(true);
    let cx = mount(cx, move || crate::app::App::new().page(page).nav_scroll(nav));
    assert_eq!(cx.update(|_, _| nav.offset().y), 0.0);
    assert!(cx.update(|_, _| nav.max_offset().height) > 0.0, "more pages than fit in the sidebar");
    // The last page's row is below the fold: selecting it scrolls it into view.
    cx.update(|_, _| page.set(DEMOS.len() - 1));
    settle(cx);
    settle(cx);
    let (offset, max) = cx.update(|_, _| (nav.offset().y, nav.max_offset().height));
    // Just far enough for the row (the group's bottom padding may stay hidden).
    assert!(offset < -(max - 16.0), "scrolled to the end: {offset} of {max}");
    // And back to the first page: its row (below the group label) is in view again.
    cx.update(|_, _| page.set(0));
    settle(cx);
    settle(cx);
    let offset = cx.update(|_, _| nav.offset().y);
    assert!(offset > -41.0, "scrolled back up: {offset}");
}

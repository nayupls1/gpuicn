use std::rc::Rc;

use rsx::prelude::*;

use crate::ui::data_table::{DataTable, DataTableColumn};
use crate::ui::dropdown_menu::{MenuEntry, MenuItem};

#[derive(Clone)]
pub struct Payment {
    pub id: &'static str,
    pub amount: f64,
    pub status: &'static str,
    pub email: &'static str,
}

pub fn payments() -> Vec<Payment> {
    vec![
        Payment { id: "m5gr84i9", amount: 316., status: "Success", email: "ken99@example.com" },
        Payment { id: "3u1reuv4", amount: 242., status: "Success", email: "Abe45@example.com" },
        Payment { id: "derv1ws0", amount: 837., status: "Processing", email: "Monserrat44@example.com" },
        Payment { id: "5kma53ae", amount: 874., status: "Success", email: "Silas22@example.com" },
        Payment { id: "bhqecj4p", amount: 721., status: "Failed", email: "carmella@example.com" },
        Payment { id: "p0r8sd2x", amount: 129., status: "Pending", email: "jason@example.com" },
        Payment { id: "a1b2c3d4", amount: 450., status: "Success", email: "olivia@example.com" },
    ]
}

pub fn demo() -> Element {
    DataTable::new(Rc::new(payments()))
        .class("w-[720px]")
        .selectable()
        .page_size(5)
        .filter("Filter emails...", |p: &Payment| p.email.to_string())
        .column(DataTableColumn::new("status", "Status", |p: &Payment| p.status).width(px(120.)))
        .column(DataTableColumn::new("email", "Email", |p: &Payment| p.email).sortable_by(|p| p.email.to_lowercase()))
        .column(DataTableColumn::new("amount", "Amount", |p: &Payment| format!("${:.2}", p.amount)).align_right().width(px(120.)))
        .row_actions(|_: &Payment| {
            vec![
                // rsx gap: no clipboard API, so "Copy payment ID" can't copy.
                MenuItem::new("Copy payment ID").into(),
                MenuEntry::Separator,
                MenuItem::new("View customer").into(),
                MenuItem::new("View payment details").into(),
            ]
        })
        .into_element()
}

use rsx::prelude::*;

use crate::ui::table::{Table, TableBody, TableCell, TableFooter, TableHead, TableHeader, TableRow};

pub fn demo() -> Element {
    let invoices = [
        ("INV001", "Paid", "Credit Card", "$250.00"),
        ("INV002", "Pending", "PayPal", "$150.00"),
        ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
        ("INV004", "Paid", "Credit Card", "$450.00"),
        ("INV005", "Paid", "PayPal", "$550.00"),
    ];
    <Table class="w-[600px]" caption="A list of your recent invoices.">
        <TableHeader>
            <TableRow>
                <TableHead text="Invoice" width={px(100.)} />
                <TableHead text="Status" />
                <TableHead text="Method" />
                <TableHead text="Amount" align_right />
            </TableRow>
        </TableHeader>
        <TableBody>
            {invoices.into_iter().map(|(invoice, status, method, amount)| <TableRow>
                <TableCell width={px(100.)} class="font-medium">{invoice}</TableCell>
                <TableCell>{status}</TableCell>
                <TableCell>{method}</TableCell>
                <TableCell align_right>{amount}</TableCell>
            </TableRow>)}
        </TableBody>
        <TableFooter>
            <TableRow>
                <TableCell>Total</TableCell>
                <TableCell align_right>$2,500.00</TableCell>
            </TableRow>
        </TableFooter>
    </Table>
}

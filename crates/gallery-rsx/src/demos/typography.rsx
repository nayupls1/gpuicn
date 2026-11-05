use rsx::prelude::*;

use crate::ui::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use crate::ui::theme::theme;
use crate::ui::typography::{blockquote, h1, h2, h3, h4, inline_code, large, lead, muted, p, small, List, ListItem};

pub fn demo() -> Element {
    let t = theme();
    <div class="flex flex-col gap-4 w-[640px]">
        {h1("Taxing Laughter: The Joke Tax Chronicles").class("text-center")}
        {lead("Once upon a time, in a far-off land, there was a very lazy king who spent all day lounging on his throne.")}
        {h2("The King's Plan").class("mt-6")}
        {p("The king thought long and hard, and finally came up with a brilliant plan: he would tax the jokes in the kingdom.")}
        {blockquote("\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair that they should pay for the privilege.\"")}
        {h3("The Joke Tax").class("mt-4")}
        <List>
            <ListItem>1st level of puns: 5 gold coins</ListItem>
            <ListItem>2nd level of jokes: 10 gold coins</ListItem>
            <ListItem>3rd level of one-liners: 20 gold coins</ListItem>
        </List>
        {h3("The People of the Kingdom")}
        <Table class="rounded-md border" border_color={t.get().border}>
            <TableHeader>
                <TableRow>
                    <TableHead text="King's Treasury" />
                    <TableHead text="People's happiness" />
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow><TableCell>Empty</TableCell><TableCell>Overflowing</TableCell></TableRow>
                <TableRow><TableCell>Modest</TableCell><TableCell>Satisfied</TableCell></TableRow>
                <TableRow><TableCell>Full</TableCell><TableCell>Ecstatic</TableCell></TableRow>
            </TableBody>
        </Table>
        {h4("People stopped telling jokes")}
        <div class="flex flex-nowrap items-center gap-1">
            {p("Run")}
            {inline_code("gpuicn add typography")}
            {p("to get these styles.")}
        </div>
        {large("Are you absolutely sure?")}
        {small("Email address")}
        {muted("Enter your email address.")}
    </div>
}

use rsx::prelude::*;

use crate::ui::pagination::Pagination;

pub fn demo() -> Element {
    let page = signal(2usize);
    <Pagination total={10} page={page} onChange={move |p: &usize| page.set(*p)} />
}

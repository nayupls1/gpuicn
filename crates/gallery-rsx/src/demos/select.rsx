use rsx::prelude::*;

use crate::ui::select::{Select, SelectGroup, SelectItem};

pub fn demo() -> Element {
    <Select
        class="w-[180px]"
        placeholder="Select a fruit"
        groups={vec![SelectGroup::new("Fruits")
            .item(SelectItem::new("apple", "Apple"))
            .item(SelectItem::new("banana", "Banana"))
            .item(SelectItem::new("blueberry", "Blueberry"))
            .item(SelectItem::new("grapes", "Grapes"))
            .item(SelectItem::new("pineapple", "Pineapple"))]}
        onChange={|value: &SharedString| {
            // TODO(sonner): `toast(format!("Picked {value}"))` once `sonner` is ported.
            eprintln!("Picked {value}");
        }}
    />
}

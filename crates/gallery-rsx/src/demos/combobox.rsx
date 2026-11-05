use rsx::prelude::*;

use crate::ui::combobox::Combobox;

pub fn frameworks() -> [(&'static str, &'static str); 5] {
    [
        ("next.js", "Next.js"),
        ("sveltekit", "SvelteKit"),
        ("nuxt.js", "Nuxt.js"),
        ("remix", "Remix"),
        ("astro", "Astro"),
    ]
}

pub fn demo() -> Element {
    <Combobox
        default_value={SharedString::from("next.js")}
        placeholder="Select framework..."
        search_placeholder="Search framework..."
        empty="No framework found."
        items={frameworks().into_iter().map(|(value, label)| (value.into(), label.into())).collect::<Vec<_>>()}
    />
}

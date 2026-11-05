//! Styles for headings, paragraphs, lists… — shadcn's typography recipes.
//!
//! ```ignore
//! <Typography variant={TypographyVariant::H1}>Taxing Laughter: The Joke Tax Chronicles</Typography>
//! {h1("Taxing Laughter: The Joke Tax Chronicles").class("text-center")}
//! {p("The king, seeing how much happier his subjects were…")}
//! {blockquote("\"After all,\" he said, \"everyone enjoys a good joke.\"")}
//! <List>
//!     <ListItem>1st level of puns: 5 gold coins</ListItem>
//! </List>
//! ```

use rsx::prelude::*;
use rsx::Div;

use super::theme::theme;
use super::utils::Siblings;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    #[default]
    P,
    Blockquote,
    Lead,
    Large,
    Small,
    Muted,
    InlineCode,
}

/// Text in one of the typography styles. `variant` defaults to `P`.
#[component]
pub fn Typography(
    #[prop(optional)] variant: TypographyVariant,
    text: Option<Accessor<String>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();
        let el: Div = match variant {
            TypographyVariant::H1 => div().class("text-[2.25rem] leading-[2.5rem] font-extrabold"),
            TypographyVariant::H2 => div().class("text-3xl leading-[2.25rem] font-semibold pb-2 border-b").border_color(theme.border),
            TypographyVariant::H3 => div().class("text-2xl leading-[2rem] font-semibold"),
            TypographyVariant::H4 => div().class("text-xl leading-[1.75rem] font-semibold"),
            TypographyVariant::P => div().class("text-base leading-[1.75rem]"),
            TypographyVariant::Blockquote => {
                let mut el = div().class("text-base leading-[1.75rem] pl-6 italic").border_color(theme.border);
                // `border-l-2`
                el.style().border_widths.left = Some(px(2.0));
                el
            }
            TypographyVariant::Lead => div().class("text-xl leading-[1.75rem]").text_color(theme.muted_foreground),
            TypographyVariant::Large => div().class("text-lg leading-[1.75rem] font-semibold"),
            TypographyVariant::Small => div().class("text-sm leading-none font-medium"),
            TypographyVariant::Muted => div().class("text-sm leading-[1.25rem]").text_color(theme.muted_foreground),
            // rsx#8: no `font-family` (the theme's mono font).
            TypographyVariant::InlineCode => div()
                .class("flex-none px-[0.3rem] py-[0.2rem] rounded-[4px] text-[0.875rem] leading-[1.25rem] font-semibold")
                .bg(theme.muted),
        };
        el.spread(&attrs).child(text).child(children)
    }
}

macro_rules! shorthand {
    ($($name:ident => $variant:ident,)*) => {$(
        #[doc = concat!("`<Typography variant={TypographyVariant::", stringify!($variant), "}>`, as a builder (add classes with `.class(..)`).")]
        pub fn $name(text: impl Into<String>) -> Typography {
            Typography::new().variant(TypographyVariant::$variant).text(text.into())
        }
    )*};
}

shorthand! {
    h1 => H1,
    h2 => H2,
    h3 => H3,
    h4 => H4,
    p => P,
    blockquote => Blockquote,
    lead => Lead,
    large => Large,
    small => Small,
    muted => Muted,
    inline_code => InlineCode,
}

#[derive(Clone, Copy)]
struct ListContext {
    ordered: bool,
    items: Siblings,
}

/// A bulleted list (`ul.list-disc`) of [`ListItem`]s.
#[component]
pub fn List(
    /// Number items instead of bulleting them.
    ordered: bool,
    children: Option<ChildrenFn>,
    attrs: Attributes,
) -> impl IntoElement {
    let context = ListContext { ordered, items: Siblings::new() };
    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div class="flex flex-col gap-2 ml-6 text-base leading-[1.75rem]" {...attrs}>
        <Provider value={context}>{children}</Provider>
    </div>
}

/// One entry of a [`List`]: a bullet (or its number) and the content.
#[component]
pub fn ListItem(children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    let t = theme();
    let context = use_context::<ListContext>();
    let id = context.map(|c| c.items.register());
    let ordered = context.is_some_and(|c| c.ordered);
    let marker = move || {
        if ordered {
            let n = context.zip(id).and_then(|(c, id)| c.items.index(id)).unwrap_or(0) + 1;
            <div class="flex-none min-w-3">{format!("{n}.")}</div>
        } else {
            // A `list-disc` bullet: a small filled circle on the first line.
            <div class="flex-none flex items-center h-[1.75rem] w-3">
                <div class="size-1.5 rounded-full" bg={t.get().foreground} />
            </div>
        }
    };
    <div class="flex gap-2" {...attrs}>
        {marker}
        <div class="flex-1 min-w-0">{children}</div>
    </div>
}

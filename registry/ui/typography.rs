//! Styles for headings, paragraphs, lists… — shadcn's typography recipes.
//!
//! ```ignore
//! h1("Taxing Laughter: The Joke Tax Chronicles")
//! p("The king, seeing how much happier his subjects were…")
//! blockquote("\"After all,\" he said, \"everyone enjoys a good joke.\"")
//! List::new().item("1st level of puns: 5 gold coins")
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, px, relative, rems,
};

use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    P,
    Blockquote,
    Lead,
    Large,
    Small,
    Muted,
    InlineCode,
}

#[derive(IntoElement)]
pub struct Typography {
    variant: TypographyVariant,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Typography, List);

impl Typography {
    pub fn new(variant: TypographyVariant, text: impl Into<SharedString>) -> Self {
        Self {
            variant,
            children: vec![text.into().into_any_element()],
            style: StyleRefinement::default(),
        }
    }
}

pub fn h1(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::H1, text)
}
pub fn h2(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::H2, text)
}
pub fn h3(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::H3, text)
}
pub fn h4(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::H4, text)
}
pub fn p(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::P, text)
}
pub fn blockquote(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::Blockquote, text)
}
pub fn lead(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::Lead, text)
}
pub fn large(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::Large, text)
}
pub fn small(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::Small, text)
}
pub fn muted(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::Muted, text)
}
pub fn inline_code(text: impl Into<SharedString>) -> Typography {
    Typography::new(TypographyVariant::InlineCode, text)
}

impl RenderOnce for Typography {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let el = div();
        let el = match self.variant {
            TypographyVariant::H1 => el.text_size(rems(2.25)).line_height(rems(2.5)).font_weight(FontWeight::EXTRA_BOLD),
            TypographyVariant::H2 => el
                .text_3xl()
                .font_weight(FontWeight::SEMIBOLD)
                .pb_2()
                .border_b_1()
                .border_color(theme.border),
            TypographyVariant::H3 => el.text_2xl().font_weight(FontWeight::SEMIBOLD),
            TypographyVariant::H4 => el.text_xl().font_weight(FontWeight::SEMIBOLD),
            TypographyVariant::P => el.line_height(rems(1.75)),
            TypographyVariant::Blockquote => el.border_l_2().border_color(theme.border).pl_6().italic(),
            TypographyVariant::Lead => el.text_xl().text_color(theme.muted_foreground),
            TypographyVariant::Large => el.text_lg().font_weight(FontWeight::SEMIBOLD),
            TypographyVariant::Small => el.text_sm().line_height(relative(1.)).font_weight(FontWeight::MEDIUM),
            TypographyVariant::Muted => el.text_sm().text_color(theme.muted_foreground),
            TypographyVariant::InlineCode => el
                .flex_none()
                .px(rems(0.3))
                .py(rems(0.2))
                .rounded(px(4.))
                .bg(theme.muted)
                .font_family(theme.mono_font_family.clone())
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD),
        };
        el.refine_style(&self.style).children(self.children)
    }
}

/// A bulleted list (`ul.list-disc`).
#[derive(IntoElement)]
pub struct List {
    children: Vec<AnyElement>,
    ordered: bool,
    style: StyleRefinement,
}

impl List {
    pub fn new() -> Self {
        Self { children: Vec::new(), ordered: false, style: StyleRefinement::default() }
    }

    /// Number items instead of bulleting them.
    pub fn ordered(mut self) -> Self {
        self.ordered = true;
        self
    }

    pub fn item(self, item: impl IntoElement) -> Self {
        self.child(item)
    }
}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let ordered = self.ordered;
        div()
            .flex()
            .flex_col()
            .gap_2()
            .ml_6()
            .refine_style(&self.style)
            .children(self.children.into_iter().enumerate().map(move |(ix, child)| {
                let marker: SharedString = if ordered { format!("{}.", ix + 1).into() } else { "•".into() };
                div()
                    .flex()
                    .gap_2()
                    .child(div().flex_none().min_w_3().child(marker))
                    .child(div().flex_1().min_w_0().child(child))
            }))
    }
}

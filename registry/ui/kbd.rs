//! Used to display textual user input from keyboard.
//!
//! ```ignore
//! KbdGroup::new().child(Kbd::new("⌘")).child(Kbd::new("K"))
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};

use super::icon::{Icon, IconName};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Kbd {
    text: SharedString,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Kbd, KbdGroup);

impl Kbd {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        // Draw the modifier symbols as icons: most UI fonts (Geist, Inter,
        // system fonts) lack ⌘/⌥, and fallback glyphs come out mismatched.
        let icon = match self.text.as_ref() {
            "⌘" => Some(IconName::Command),
            "⇧" => Some(IconName::ArrowBigUp),
            "⌥" => Some(IconName::Option),
            "⌃" => Some(IconName::ChevronUp),
            _ => None,
        };
        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap_1()
            .h_5()
            .min_w_5()
            .px_1()
            .rounded(theme.radius_sm())
            .bg(theme.muted)
            .text_color(theme.muted_foreground)
            .text_xs().line_height(gpui::rems(1.))
            .font_weight(FontWeight::MEDIUM)
            .refine_style(&self.style)
            .map(|this| match icon {
                Some(icon) => this.child(Icon::new(icon).size_3()),
                None => this.child(self.text),
            })
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct KbdGroup {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl KbdGroup {
    pub fn new() -> Self {
        Self { children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for KbdGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().items_center().gap_1().refine_style(&self.style).children(self.children)
    }
}

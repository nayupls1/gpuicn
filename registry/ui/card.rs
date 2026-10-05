//! Displays a card with header, content, and footer.
//!
//! ```ignore
//! Card::new()
//!     .child(
//!         CardHeader::new()
//!             .child(CardTitle::new("Login to your account"))
//!             .child(CardDescription::new("Enter your email below"))
//!             .action(Button::new("signup").link().label("Sign Up")),
//!     )
//!     .child(CardContent::new().child(...))
//!     .child(CardFooter::new().child(Button::new("login").label("Login").w_full()))
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, relative,
};

use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Card {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Card {
    pub fn new() -> Self {
        Self { children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_col()
            .gap_6()
            .py_6()
            .rounded(theme.radius_xl())
            .border_1()
            .border_color(theme.border)
            .bg(theme.card)
            .text_color(theme.card_foreground)
            .when(theme.shadows, |this| this.shadow_sm())
            .refine_style(&self.style)
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct CardHeader {
    action: Option<AnyElement>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl CardHeader {
    pub fn new() -> Self {
        Self { action: None, children: Vec::new(), style: StyleRefinement::default() }
    }

    /// Content pinned to the top-right corner of the header (`CardAction`).
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl RenderOnce for CardHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_start()
            .gap_2()
            .px_6()
            .refine_style(&self.style)
            .child(div().flex().flex_col().flex_1().min_w_0().gap_1p5().children(self.children))
            .children(self.action.map(|action| div().flex_none().child(action)))
    }
}

#[derive(IntoElement)]
pub struct CardTitle {
    text: SharedString,
    style: StyleRefinement,
}

impl CardTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for CardTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .line_height(relative(1.))
            .font_weight(FontWeight::SEMIBOLD)
            .refine_style(&self.style)
            .child(self.text)
    }
}

#[derive(IntoElement)]
pub struct CardDescription {
    text: SharedString,
    style: StyleRefinement,
}

impl CardDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for CardDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .refine_style(&self.style)
            .child(self.text)
    }
}

#[derive(IntoElement)]
pub struct CardContent {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl CardContent {
    pub fn new() -> Self {
        Self { children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for CardContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().px_6().refine_style(&self.style).children(self.children)
    }
}

#[derive(IntoElement)]
pub struct CardFooter {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl CardFooter {
    pub fn new() -> Self {
        Self { children: Vec::new(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for CardFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .px_6()
            .refine_style(&self.style)
            .children(self.children)
    }
}

crate::impl_styled_parent!(Card, CardHeader, CardContent, CardFooter);
crate::impl_styled!(CardTitle, CardDescription);

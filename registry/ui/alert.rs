//! Displays a callout for user attention.
//!
//! ```ignore
//! Alert::new()
//!     .icon(IconName::CircleCheck)
//!     .title("Success! Your changes have been saved")
//!     .description("This is an alert with icon, title and description.")
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};

use super::icon::Icon;
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

#[derive(IntoElement)]
pub struct Alert {
    variant: AlertVariant,
    icon: Option<Icon>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Alert);

impl Alert {
    pub fn new() -> Self {
        Self {
            variant: AlertVariant::Default,
            icon: None,
            title: None,
            description: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn destructive(self) -> Self {
        self.variant(AlertVariant::Destructive)
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (fg, description_fg) = match self.variant {
            AlertVariant::Default => (theme.card_foreground, theme.muted_foreground),
            AlertVariant::Destructive => (theme.destructive, theme.destructive.opacity(0.9)),
        };

        div()
            .w_full()
            .flex()
            .items_start()
            .gap_3()
            .px_4()
            .py_3()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .bg(theme.card)
            .text_color(fg)
            .text_sm()
            .refine_style(&self.style)
            .children(self.icon.map(|icon| div().pt_0p5().child(icon)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap_0p5()
                    .children(self.title.map(|title| {
                        div().font_weight(FontWeight::MEDIUM).line_clamp(1).child(title)
                    }))
                    .when(self.description.is_some() || !self.children.is_empty(), |this| {
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .text_color(description_fg)
                                .children(self.description)
                                .children(self.children),
                        )
                    }),
            )
    }
}

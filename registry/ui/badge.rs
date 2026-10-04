//! Displays a badge or a component that looks like a badge.
//!
//! ```ignore
//! Badge::new("New")
//! Badge::new("Failed").destructive()
//! Badge::new("v1.0").outline().icon(IconName::Check)
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*,
};

use super::icon::Icon;
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

#[derive(IntoElement)]
pub struct Badge {
    label: Option<SharedString>,
    icon: Option<Icon>,
    variant: BadgeVariant,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Badge);

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self::empty().label(label)
    }

    /// A badge without a label, for custom children.
    pub fn empty() -> Self {
        Self {
            label: None,
            icon: None,
            variant: BadgeVariant::Default,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn secondary(self) -> Self {
        self.variant(BadgeVariant::Secondary)
    }

    pub fn destructive(self) -> Self {
        self.variant(BadgeVariant::Destructive)
    }

    pub fn outline(self) -> Self {
        self.variant(BadgeVariant::Outline)
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (bg, fg, border) = match self.variant {
            BadgeVariant::Default => (theme.primary, theme.primary_foreground, gpui::transparent_black()),
            BadgeVariant::Secondary => (theme.secondary, theme.secondary_foreground, gpui::transparent_black()),
            BadgeVariant::Destructive => {
                let bg = if theme.is_dark() { theme.destructive.opacity(0.6) } else { theme.destructive };
                (bg, gpui::white(), gpui::transparent_black())
            }
            BadgeVariant::Outline => (gpui::transparent_black(), theme.foreground, theme.border),
        };

        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap_1()
            .px_2()
            .py_0p5()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(border)
            .bg(bg)
            .text_color(fg)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .whitespace_nowrap()
            .refine_style(&self.style)
            .children(self.icon.map(|icon| icon.size_3()))
            .children(self.label)
            .children(self.children)
    }
}

//! Displays a badge or a component that looks like a badge.
//!
//! ```ignore
//! <Badge label="New" />
//! <Badge destructive label="Failed" />
//! <Badge outline icon={IconName::Check} label="v1.0" />
//! <Badge label="8" class="rounded-full h-5 min-w-5 px-1" />
//! ```

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::theme::{theme, ColorExt, Theme};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

/// `(background, text, border)` for a badge variant.
pub fn badge_colors(variant: BadgeVariant, theme: &Theme) -> (Color, Color, Color) {
    let clear = Color::TRANSPARENT;
    match variant {
        BadgeVariant::Default => (theme.primary, theme.primary_foreground, clear),
        BadgeVariant::Secondary => (theme.secondary, theme.secondary_foreground, clear),
        BadgeVariant::Destructive => {
            let bg = if theme.is_dark() { theme.destructive.opacity(0.6) } else { theme.destructive };
            (bg, Color::WHITE, clear)
        }
        BadgeVariant::Outline => (clear, theme.foreground, theme.border),
    }
}

#[component]
pub fn Badge(
    label: Option<Accessor<String>>,
    icon: Option<IconName>,
    #[prop(optional)] variant: BadgeVariant,
    /// Shorthands for `variant`: `<Badge outline>`.
    secondary: bool,
    destructive: bool,
    outline: bool,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let variant = if secondary {
        BadgeVariant::Secondary
    } else if destructive {
        BadgeVariant::Destructive
    } else if outline {
        BadgeVariant::Outline
    } else {
        variant
    };
    let t = theme();
    move || {
        let theme = t.get();
        let (bg, fg, border) = badge_colors(variant, &theme);
        // rsx#9: `whitespace-nowrap` isn't supported; `flex-none` keeps the badge from squeezing.
        <div
            class="flex flex-none items-center justify-center gap-1 px-2 py-0.5 border text-xs leading-[1rem] font-medium whitespace-nowrap"
            corner_radius={theme.radius_md()}
            border_color={border}
            bg={bg}
            text_color={fg}
            {...&attrs}
        >
            {icon.map(|i| <Icon name={i} class="size-3" />)}
            {label.map(|l| l.get())}
            {children.map(|c| c.render())}
        </div>
    }
}

//! Displays a callout for user attention.
//!
//! ```ignore
//! <Alert
//!     icon={IconName::CircleCheck}
//!     title="Success! Your changes have been saved"
//!     description="This is an alert with icon, title and description."
//! />
//! <Alert destructive icon={IconName::CircleAlert} title="Unable to process your payment." />
//! ```

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::theme::{theme, ColorExt};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

#[component]
pub fn Alert(
    #[prop(optional)] variant: AlertVariant,
    /// Shorthand for `variant={AlertVariant::Destructive}`.
    destructive: bool,
    icon: Option<IconName>,
    title: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// Extra description content, below `description`.
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let variant = if destructive { AlertVariant::Destructive } else { variant };
    let t = theme();
    move || {
        let theme = t.get();
        let (fg, description_fg) = match variant {
            AlertVariant::Default => (theme.card_foreground, theme.muted_foreground),
            AlertVariant::Destructive => (theme.destructive, theme.destructive.opacity(0.9)),
        };
        <div
            class="w-full flex items-start gap-3 px-4 py-3 border text-sm leading-[1.25rem]"
            corner_radius={theme.radius_lg()}
            border_color={theme.border}
            bg={theme.card}
            text_color={fg}
            {...&attrs}
        >
            {icon.map(|icon| <div class="pt-0.5"><Icon name={icon} /></div>)}
            <div class="flex flex-col flex-1 min-w-0 gap-0.5">
                {title.map(|title| <div class="font-medium line-clamp-1">{title}</div>)}
                {(description.is_some() || children.is_some()).then(|| <div class="flex flex-col gap-1" text_color={description_fg}>
                    {description}
                    {children}
                </div>)}
            </div>
        </div>
    }
}

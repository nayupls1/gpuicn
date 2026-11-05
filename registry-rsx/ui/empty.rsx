//! Use the Empty component to display an empty state.
//!
//! ```ignore
//! <Empty
//!     icon={IconName::Folder}
//!     title="No Projects Yet"
//!     description="You haven't created any projects yet."
//! >
//!     <Button label="Create Project" />
//! </Empty>
//! ```

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::theme::{theme, ColorExt};

#[component]
pub fn Empty(
    /// Icon in a muted tile (`EmptyMedia variant="icon"`).
    icon: Option<IconName>,
    /// Arbitrary media, e.g. an avatar group (takes the place of `icon`).
    media: Option<Children>,
    title: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// Dashed border around the empty state.
    outline: bool,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();
        let icon_tile = icon.map(|icon| <div
            class="flex flex-none size-10 items-center justify-center"
            corner_radius={theme.radius_lg()}
            bg={theme.muted}
            text_color={theme.foreground}
        >
            <Icon name={icon} class="size-6" />
        </div>);
        let media = media.map(|m| m.render()).or(icon_tile);
        // rsx gap: no dashed borders, so `outline` draws a solid one in a lighter tone.
        <div
            class="flex flex-col flex-1 min-w-0 items-center justify-center gap-6 p-12"
            corner_radius={theme.radius_lg()}
            border_width={px(if outline { 1.0 } else { 0.0 })}
            border_color={theme.muted_foreground.opacity(0.3)}
            {...&attrs}
        >
            <div class="flex flex-col max-w-[384px] items-center gap-2 text-center">
                {media.map(|media| <div class="mb-2">{media}</div>)}
                {title.map(|title| <div class="text-lg leading-[1.75rem] font-medium">{title}</div>)}
                {description.map(|description| <div class="text-sm leading-[1.625]" text_color={theme.muted_foreground}>
                    {description}
                </div>)}
            </div>
            {children.map(|children| <div class="flex flex-col w-full max-w-[384px] items-center gap-4 text-sm leading-[1.25rem]">
                {children}
            </div>)}
        </div>
    }
}

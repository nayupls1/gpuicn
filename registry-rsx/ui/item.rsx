//! A versatile component that you can use to display any content.
//!
//! ```ignore
//! <Item
//!     outline
//!     icon={IconName::CircleCheck}
//!     title="Your profile has been verified."
//!     description="Two-factor authentication is enabled."
//!     action={<Button outline size={ButtonSize::Sm} label="Review" />}
//! />
//! <ItemGroup separated>
//!     <Item title="shadcn" description="shadcn@vercel.com" />
//!     <Item title="maxleiter" description="maxleiter@vercel.com" />
//! </ItemGroup>
//! ```

use rsx::prelude::*;
use rsx::Div;

use super::icon::{Icon, IconName};
use super::separator::Separator;
use super::theme::{theme, ColorExt};
use super::utils::Siblings;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

#[derive(Clone, Copy)]
struct GroupContext {
    separated: bool,
    items: Siblings,
}

#[component]
pub fn Item(
    #[prop(optional)] variant: ItemVariant,
    /// Shorthands for `variant`: `<Item outline>`.
    outline: bool,
    muted: bool,
    #[prop(optional)] size: ItemSize,
    /// `ItemMedia variant="icon"`.
    icon: Option<IconName>,
    /// `ItemMedia` with arbitrary content (avatar, image, …); takes the place of `icon`.
    media: Option<Children>,
    title: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// Elements in the trailing `ItemActions` slot.
    action: Option<Children>,
    /// Make the whole item clickable (like `<Item asChild><a/></Item>`).
    on_click: Option<Callback<ClickEvent>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let variant = if outline {
        ItemVariant::Outline
    } else if muted {
        ItemVariant::Muted
    } else {
        variant
    };
    let t = theme();
    let hovered = signal(false);
    // Inside an `<ItemGroup separated>`: a separator above every item but the first.
    let group = use_context::<GroupContext>();
    let id = group.map(|g| g.items.register());
    let separator = move || match (group, id) {
        (Some(g), Some(id)) if g.separated && !g.items.is_first(id) => Some(<Separator />),
        _ => None,
    };
    let has_content = title.is_some() || description.is_some() || children.is_some();

    let item = move || {
        let theme = t.get();
        let media = media.map(|m| m.render()).or_else(|| {
            icon.map(|icon| <div
                class="flex flex-none size-8 items-center justify-center border"
                corner_radius={theme.radius_sm()}
                border_color={theme.border}
                bg={theme.muted}
            >
                <Icon name={icon} />
            </div>)
        });
        // Hover is a `bind` below, so it doesn't re-create the item (losing a click in progress).
        let (border, bg) = match variant {
            ItemVariant::Default => (Color::TRANSPARENT, Color::TRANSPARENT),
            ItemVariant::Outline => (theme.border, Color::TRANSPARENT),
            ItemVariant::Muted => (Color::TRANSPARENT, theme.muted.opacity(0.5)),
        };
        <div
            class="flex items-center border text-sm leading-[1.25rem]"
            class:p-4={size == ItemSize::Default}
            class:gap-4={size == ItemSize::Default}
            class:py-3={size == ItemSize::Sm}
            class:px-4={size == ItemSize::Sm}
            class:gap-2.5={size == ItemSize::Sm}
            class:cursor-pointer={on_click.is_some()}
            corner_radius={theme.radius_md()}
            border_color={border}
            bg={bg}
            {...&attrs}
            bind={move |el: Div| if on_click.is_some() && hovered.get() { el.bg(t.get().accent.opacity(0.5)) } else { el }}
            onHover={move |h: &bool| hovered.set(*h)}
            onClick={move |e: &ClickEvent| {
                if let Some(f) = on_click {
                    f.call(e)
                }
            }}
        >
            {media}
            {has_content.then(|| <div class="flex flex-col flex-1 min-w-0 gap-1">
                {title.map(|title| <div class="flex items-center gap-2 font-medium leading-[1.375]">{title}</div>)}
                {/* rsx#9: no `line-clamp-2` on the description. */}
                {description.map(|description| <div text_color={theme.muted_foreground}>{description}</div>)}
                {children}
            </div>)}
            {action.map(|action| <div class="flex items-center gap-2">{action}</div>)}
        </div>
    };
    <>
        {separator}
        {item}
    </>
}

/// A vertical list of items, optionally separated.
#[component]
pub fn ItemGroup(
    /// Insert an `ItemSeparator` between items.
    separated: bool,
    /// [`Item`]s.
    children: Option<ChildrenFn>,
    attrs: Attributes,
) -> impl IntoElement {
    let context = GroupContext { separated, items: Siblings::new() };
    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div class="flex flex-col" {...attrs}>
        <Provider value={context}>{children}</Provider>
    </div>
}

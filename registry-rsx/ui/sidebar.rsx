//! A composable, themeable and customizable sidebar component.
//!
//! The collapsed state is shared by id (like `SidebarProvider`), so a
//! [`SidebarTrigger`] anywhere can toggle it:
//!
//! ```ignore
//! <div class="flex size-full">
//!     <Sidebar id="app" collapsible={SidebarCollapsible::Icon} header={<div>Acme Inc.</div>}>
//!         <SidebarGroup label="Application">
//!             <SidebarMenuItem label="Home" icon={IconName::House} active />
//!             <SidebarMenuItem label="Inbox" icon={IconName::Inbox} badge="24" />
//!             <SidebarMenuItem label="Settings" icon={IconName::Settings}>
//!                 <SidebarMenuItem label="General" />
//!                 <SidebarMenuItem label="Billing" />
//!             </SidebarMenuItem>
//!         </SidebarGroup>
//!     </Sidebar>
//!     <SidebarInset>
//!         <SidebarTrigger sidebar="app" />
//!         {page}
//!     </SidebarInset>
//! </div>
//! ```
//!
//! gpuicn's builder parts map to components: `SidebarGroup::new().label(..).item(..)`
//! is `<SidebarGroup label=..>` with `<SidebarMenuItem>` children (any other
//! child is `SidebarGroupContent`), and `.sub_item(..)` is a nested
//! `<SidebarMenuItem>`.

use std::cell::RefCell;
use std::collections::HashMap;

use rsx::prelude::*;
use rsx::{Scope, Style};

use super::icon::{Icon, IconName};
use super::scroll_area::{ScrollArea, ScrollHandle};
use super::theme::{mix, theme, ColorExt};

pub const SIDEBAR_WIDTH: f32 = 256.;
pub const SIDEBAR_WIDTH_ICON: f32 = 48.;

thread_local! {
    /// Collapsed state per sidebar id (`SidebarProvider`). The signals are
    /// owned by a root scope that is never disposed.
    static STORE: (Scope, RefCell<HashMap<String, Signal<bool>>>) = (Scope::root(), RefCell::new(HashMap::new()));
}

fn collapsed_signal(id: &str) -> Signal<bool> {
    STORE.with(|(scope, map)| *map.borrow_mut().entry(id.to_string()).or_insert_with(|| scope.run(|| signal(false))))
}

/// Whether the sidebar with this id is collapsed (tracked).
pub fn is_sidebar_collapsed(id: &str) -> bool {
    collapsed_signal(id).get()
}

pub fn set_sidebar_collapsed(id: impl AsRef<str>, collapsed: bool) {
    collapsed_signal(id.as_ref()).set(collapsed);
}

pub fn toggle_sidebar(id: impl AsRef<str>) {
    collapsed_signal(id.as_ref()).update(|c| *c = !*c);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarCollapsible {
    /// Collapses to a strip of icons.
    #[default]
    Icon,
    /// Slides away completely.
    Offcanvas,
    /// Never collapses.
    None,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

/// What a sidebar tells its groups and items (`useSidebar`).
#[derive(Clone, Copy)]
pub struct SidebarContext {
    /// Collapsed to the icon strip (tracked).
    pub icon_mode: Memo<bool>,
}

/// Set by a menu item for its nested items (`SidebarMenuSub`).
#[derive(Clone, Copy)]
struct SubMenu {
    open: Signal<bool>,
}

fn icon_mode() -> bool {
    use_context::<SidebarContext>().is_some_and(|c| c.icon_mode.get())
}

#[component]
pub fn Sidebar(
    /// Shares the collapsed state with `SidebarTrigger sidebar={id}`.
    id: String,
    #[prop(optional)] collapsible: SidebarCollapsible,
    #[prop(optional)] variant: SidebarVariant,
    #[prop(optional)] side: SidebarSide,
    /// Control the collapsed state yourself instead of using the shared store.
    collapsed: Option<Accessor<bool>>,
    header: Option<Children>,
    footer: Option<Children>,
    /// The scrolling content area's handle, e.g. to scroll it yourself
    /// (the active item scrolls into view by itself).
    track_scroll: Option<ScrollHandle>,
    /// Groups (`SidebarGroup`) and any other content.
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let store = collapsed_signal(&id);
    let collapsed = memo(move || collapsed.map(|c| c.get()).unwrap_or_else(|| store.get()) && collapsible != SidebarCollapsible::None);
    let icon_mode = memo(move || collapsed.get() && collapsible == SidebarCollapsible::Icon);
    provide_context(SidebarContext { icon_mode });
    let scroll = track_scroll.unwrap_or_default();
    let t = theme();

    move || {
        if collapsed.get() && collapsible == SidebarCollapsible::Offcanvas {
            return <div />;
        }
        let theme = t.get();
        let icon = icon_mode.get();
        let width = if icon { SIDEBAR_WIDTH_ICON } else { SIDEBAR_WIDTH };
        let right = side == SidebarSide::Right;
        <div
            class="flex flex-col flex-none h-full"
            class:border-r={variant == SidebarVariant::Sidebar && !right}
            class:border-l={variant == SidebarVariant::Sidebar && right}
            class:m-2={variant == SidebarVariant::Floating}
            class:h-auto={variant == SidebarVariant::Floating}
            class:border={variant == SidebarVariant::Floating}
            class:shadow-sm={variant == SidebarVariant::Floating && theme.shadows}
            corner_radius={if variant == SidebarVariant::Floating { theme.radius_lg() } else { px(0.0) }}
            border_color={theme.sidebar_border}
            w={px(width)}
            bg={theme.sidebar}
            text_color={theme.sidebar_foreground}
            {...&attrs}
        >
            {header.map(|h| <div class="flex flex-col gap-2 p-2">{h}</div>)}
            {/* A themed scrollbar shows there's more below when the menu overflows. */}
            <ScrollArea class="flex-1 min-h-0" track_scroll={scroll}>
                <div class="flex flex-col gap-2" class:items-center={icon}>{children}</div>
            </ScrollArea>
            {footer.map(|f| <div class="flex flex-col gap-2 p-2">{f}</div>)}
        </div>
    }
}

/// `SidebarGroup` with an optional `SidebarGroupLabel` and a `SidebarMenu`
/// holding its children (menu items, or any content).
#[component]
pub fn SidebarGroup(label: Option<Accessor<String>>, children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();
        let label_color = theme.sidebar_foreground.opacity(0.7);
        // `pr-3` leaves room for the scrollbar thumb.
        <div class="flex flex-col w-full min-w-0 p-2 pr-3" {...&attrs}>
            {label.filter(|_| !icon_mode()).map(|label| <div
                class="flex flex-none h-8 items-center px-2 text-xs leading-none font-medium"
                text_color={label_color}
            >{label}</div>)}
            <div class="flex flex-col w-full gap-1">{children}</div>
        </div>
    }
}

/// A `SidebarMenuItem` + `SidebarMenuButton`. Nested `SidebarMenuItem`
/// children become its `SidebarMenuSub`, and the item toggles them.
#[component]
pub fn SidebarMenuItem(
    label: Accessor<String>,
    icon: Option<IconName>,
    /// `isActive`
    active: Option<Accessor<bool>>,
    /// `SidebarMenuBadge`
    badge: Option<Accessor<String>>,
    on_click: Option<Callback<ClickEvent>>,
    /// Show the nested items initially (they also open when one is active).
    default_open: bool,
    /// Nested items (`SidebarMenuSubItem`).
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let parent = use_context::<SubMenu>();
    let sub = parent.is_some();
    let active = move || active.map(|a| a.get()).unwrap_or(false);
    // An active nested item opens its parent.
    if let Some(parent) = parent {
        effect(move || {
            if active() {
                parent.open.set(true);
            }
        });
    }
    let open = signal(default_open);
    provide_context(SubMenu { open });
    let has_sub = children.is_some();
    // The active item scrolls into view in the sidebar (gpuicn's gallery
    // `reveal_active`), e.g. when it becomes active from elsewhere.
    let row = NodeRef::new();
    effect(move || {
        if active() {
            row.scroll_into_view();
        }
    });
    let ctx = use_context::<SidebarContext>();
    let icon_mode = move || ctx.is_some_and(|c| c.icon_mode.get());

    let button = move || {
        let theme = t.get();
        let icon_only = icon_mode() && !sub;
        let active = active();
        let (accent, accent_fg) = (theme.sidebar_accent, theme.sidebar_accent_foreground);
        let size = if icon_only {
            "size-8 justify-center"
        } else if sub {
            "h-7 px-2 w-full"
        } else {
            "h-8 p-2 w-full"
        };
        let is_open = open.get();
        <div
            ref={row}
            class={format!("flex flex-none items-center gap-2 overflow-hidden text-sm leading-[1.25rem] cursor-pointer {size}")}
            class:font-medium={active}
            corner_radius={theme.radius_md()}
            // Active is a notch stronger than hover so the current page stays clear.
            bg={if active { mix(accent, theme.sidebar_foreground, 0.05) } else { Color::TRANSPARENT }}
            text_color={if active { accent_fg } else { theme.sidebar_foreground }}
            // Hover is a lighter tint than the active item so you never lose
            // track of where you are while pointing at another row.
            hover={move |s: Style| if active { s } else { s.bg(accent).text_color(accent_fg) }}
            onClick={move |e: &ClickEvent| {
                if has_sub {
                    open.update(|o| *o = !*o);
                }
                if let Some(f) = on_click {
                    f.call(e);
                }
            }}
            {...&attrs}
        >
            {icon.map(|i| <Icon name={i} />)}
            {(!icon_only).then(|| <>
                <div class="flex-1 min-w-0 overflow-hidden whitespace-nowrap">{label}</div>
                {badge.map(|b| <div class="ml-auto text-xs leading-none font-medium">{b}</div>)}
                {has_sub.then(|| <Icon name={if is_open { IconName::ChevronDown } else { IconName::ChevronRight }} class="size-4 flex-none ml-auto" />)}
            </>)}
        </div>
    };

    // The nested items are always built (hidden while closed), so an active
    // one can open its parent.
    <div class="flex flex-col gap-1">
        {button}
        {has_sub.then(|| <div
            class="flex flex-col gap-1 mx-3.5 px-2.5 py-0.5 border-l"
            class:hidden={!open.get() || icon_mode()}
            border_color={t.get().sidebar_border}
        >{children}</div>)}
    </div>
}

/// The main content area next to a sidebar.
#[component]
pub fn SidebarInset(
    /// Rounded, inset look for `SidebarVariant::Inset`.
    inset: bool,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();
        <div
            class="relative flex flex-col flex-1 min-w-0"
            class:shadow-sm={inset && theme.shadows}
            class:m-2={inset}
            class:ml-0={inset}
            corner_radius={if inset { theme.radius_xl() } else { px(0.0) }}
            bg={theme.background}
            {...&attrs}
        >
            {children}
        </div>
    }
}

/// Toggles the sidebar with the given id.
#[component]
pub fn SidebarTrigger(sidebar: String, attrs: Attributes) -> impl IntoElement {
    let t = theme();
    let sidebar = StoredValue::new(sidebar);
    move || {
        let theme = t.get();
        let (accent, accent_fg) = (theme.accent, theme.accent_foreground);
        <div
            class="flex flex-none size-7 items-center justify-center cursor-pointer"
            corner_radius={theme.radius_md()}
            hover={move |s: Style| s.bg(accent).text_color(accent_fg)}
            onClick={move |_| sidebar.with_value(|s| toggle_sidebar(s))}
            {...&attrs}
        >
            <Icon name={IconName::PanelLeft} />
        </div>
    }
}

/// `SidebarSeparator`
#[component]
pub fn SidebarSeparator(attrs: Attributes) -> impl IntoElement {
    let t = theme();
    <div class="mx-2 h-px flex-none" bg={t.get().sidebar_border} {...attrs} />
}

/// `SidebarSeparator`, as a function (gpuicn's `sidebar_separator(cx)`).
pub fn sidebar_separator() -> Element {
    SidebarSeparator::new().into_element()
}

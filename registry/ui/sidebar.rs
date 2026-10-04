//! A composable, themeable and customizable sidebar component.
//!
//! The collapsed state is shared by id (like `SidebarProvider`), so a
//! [`SidebarTrigger`] anywhere can toggle it:
//!
//! ```ignore
//! div().flex().size_full()
//!     .child(
//!         Sidebar::new("app")
//!             .collapsible(SidebarCollapsible::Icon)
//!             .header(div().child("Acme Inc."))
//!             .group(
//!                 SidebarGroup::new()
//!                     .label("Application")
//!                     .item(SidebarMenuItem::new("Home").icon(IconName::House).active(true))
//!                     .item(SidebarMenuItem::new("Inbox").icon(IconName::Inbox).badge("24"))
//!                     .item(SidebarMenuItem::new("Settings").icon(IconName::Settings)
//!                         .sub_item(SidebarMenuItem::new("General"))
//!                         .sub_item(SidebarMenuItem::new("Billing"))),
//!             ),
//!     )
//!     .child(SidebarInset::new().child(SidebarTrigger::new("app")).child(page))
//! ```

use gpui::{
    AnyElement, App, ClickEvent, FontWeight, Global, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Window, div, prelude::*, px,
};
use std::collections::HashMap;
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

pub const SIDEBAR_WIDTH: f32 = 256.;
pub const SIDEBAR_WIDTH_ICON: f32 = 48.;

#[derive(Default)]
struct SidebarStore {
    collapsed: HashMap<SharedString, bool>,
}

impl Global for SidebarStore {}

pub fn is_sidebar_collapsed(id: &str, cx: &App) -> bool {
    cx.try_global::<SidebarStore>().and_then(|s| s.collapsed.get(id).copied()).unwrap_or(false)
}

pub fn set_sidebar_collapsed(id: impl Into<SharedString>, collapsed: bool, cx: &mut App) {
    cx.default_global::<SidebarStore>().collapsed.insert(id.into(), collapsed);
    cx.refresh_windows();
}

pub fn toggle_sidebar(id: impl Into<SharedString>, cx: &mut App) {
    let id = id.into();
    let collapsed = is_sidebar_collapsed(&id, cx);
    set_sidebar_collapsed(id, !collapsed, cx);
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

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A `SidebarMenuItem` + `SidebarMenuButton` (and optional `SidebarMenuSub`).
pub struct SidebarMenuItem {
    label: SharedString,
    icon: Option<IconName>,
    active: bool,
    badge: Option<SharedString>,
    on_click: Option<ClickHandler>,
    sub_items: Vec<SidebarMenuItem>,
    default_open: bool,
}

impl SidebarMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), icon: None, active: false, badge: None, on_click: None, sub_items: Vec::new(), default_open: false }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// `isActive`
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// `SidebarMenuBadge`
    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Nested item (`SidebarMenuSubItem`); the parent becomes collapsible.
    pub fn sub_item(mut self, item: SidebarMenuItem) -> Self {
        self.sub_items.push(item);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
}

enum GroupEntry {
    Item(SidebarMenuItem),
    Element(AnyElement),
}

/// `SidebarGroup` with an optional `SidebarGroupLabel` and a `SidebarMenu`.
pub struct SidebarGroup {
    label: Option<SharedString>,
    entries: Vec<GroupEntry>,
}

impl SidebarGroup {
    pub fn new() -> Self {
        Self { label: None, entries: Vec::new() }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn item(mut self, item: SidebarMenuItem) -> Self {
        self.entries.push(GroupEntry::Item(item));
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = SidebarMenuItem>) -> Self {
        self.entries.extend(items.into_iter().map(GroupEntry::Item));
        self
    }

    /// Arbitrary content (`SidebarGroupContent`).
    pub fn child(mut self, element: impl IntoElement) -> Self {
        self.entries.push(GroupEntry::Element(element.into_any_element()));
        self
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(IntoElement)]
pub struct Sidebar {
    id: SharedString,
    collapsible: SidebarCollapsible,
    variant: SidebarVariant,
    side: SidebarSide,
    collapsed: Option<bool>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    groups: Vec<SidebarGroup>,
    style: StyleRefinement,
}

crate::impl_styled!(Sidebar);

impl Sidebar {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            collapsible: SidebarCollapsible::Icon,
            variant: SidebarVariant::Sidebar,
            side: SidebarSide::Left,
            collapsed: None,
            header: None,
            footer: None,
            groups: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn collapsible(mut self, collapsible: SidebarCollapsible) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    /// Control the collapsed state yourself instead of using the shared store.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = Some(collapsed);
        self
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn group(mut self, group: SidebarGroup) -> Self {
        self.groups.push(group);
        self
    }
}

fn menu_item(item: SidebarMenuItem, key: SharedString, icon_mode: bool, sub: bool, window: &mut Window, cx: &mut App) -> AnyElement {
    let has_sub = !item.sub_items.is_empty();
    let default_open = item.default_open || item.sub_items.iter().any(|i| i.active);
    let open_state = window.use_keyed_state(gpui::ElementId::Name(format!("{key}-open").into()), cx, |_, _| default_open);
    let theme = cx.theme().clone();
    let (accent, accent_fg) = (theme.sidebar_accent, theme.sidebar_accent_foreground);
    let open = *open_state.read(cx);
    let toggle = open_state.clone();
    let on_click = item.on_click.clone();

    let button = div()
        .id(gpui::ElementId::Name(key.clone()))
        .flex()
        .items_center()
        .gap_2()
        .overflow_hidden()
        .rounded(theme.radius_md())
        .text_sm()
        .cursor_pointer()
        .map(|this| {
            if icon_mode && !sub {
                this.size_8().justify_center()
            } else if sub {
                this.h_7().px_2().w_full()
            } else {
                this.h_8().p_2().w_full()
            }
        })
        .when(item.active, |this| this.bg(accent).text_color(accent_fg).font_weight(FontWeight::MEDIUM))
        .hover(move |s| s.bg(accent).text_color(accent_fg))
        .children(item.icon.map(Icon::new))
        .when(!icon_mode || sub, |this| {
            this.child(div().flex_1().overflow_hidden().whitespace_nowrap().child(item.label.clone()))
                .children(item.badge.clone().map(|badge| div().ml_auto().text_xs().font_weight(FontWeight::MEDIUM).child(badge)))
                .when(has_sub, |this| {
                    this.child(Icon::new(if open { IconName::ChevronDown } else { IconName::ChevronRight }).ml_auto())
                })
        })
        .on_click(move |event, window, cx| {
            if has_sub {
                toggle.update(cx, |open, cx| {
                    *open = !*open;
                    cx.notify();
                });
            }
            if let Some(handler) = &on_click {
                handler(event, window, cx);
            }
        });

    let subs = (has_sub && open && !icon_mode).then(|| {
        let sub_items: Vec<AnyElement> = item
            .sub_items
            .into_iter()
            .enumerate()
            .map(|(ix, sub_item)| menu_item(sub_item, format!("{key}-{ix}").into(), icon_mode, true, window, cx))
            .collect();
        let border = cx.theme().sidebar_border;
        div()
            .flex()
            .flex_col()
            .gap_1()
            .mx_3p5()
            .px_2p5()
            .py_0p5()
            .border_l_1()
            .border_color(border)
            .children(sub_items)
    });

    div().flex().flex_col().gap_1().child(button).children(subs).into_any_element()
}

impl RenderOnce for Sidebar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let collapsed = self.collapsed.unwrap_or_else(|| is_sidebar_collapsed(&self.id, cx))
            && self.collapsible != SidebarCollapsible::None;
        let icon_mode = collapsed && self.collapsible == SidebarCollapsible::Icon;
        if collapsed && self.collapsible == SidebarCollapsible::Offcanvas {
            return div().into_any_element();
        }

        let mut groups = Vec::new();
        for (group_ix, group) in self.groups.into_iter().enumerate() {
            let mut entries = Vec::new();
            for (ix, entry) in group.entries.into_iter().enumerate() {
                entries.push(match entry {
                    GroupEntry::Item(item) => {
                        menu_item(item, format!("{}-{group_ix}-{ix}", self.id).into(), icon_mode, false, window, cx)
                    }
                    GroupEntry::Element(element) => element,
                });
            }
            let label_color = cx.theme().sidebar_foreground.opacity(0.7);
            groups.push(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .min_w_0()
                    .p_2()
                    .children(group.label.filter(|_| !icon_mode).map(|label| {
                        div()
                            .flex()
                            .flex_none()
                            .h_8()
                            .items_center()
                            .px_2()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(label_color)
                            .child(label)
                    }))
                    .child(div().flex().flex_col().w_full().gap_1().children(entries)),
            );
        }

        let theme = cx.theme();
        let width = px(if icon_mode { SIDEBAR_WIDTH_ICON } else { SIDEBAR_WIDTH });
        let right = self.side == SidebarSide::Right;
        div()
            .flex()
            .flex_col()
            .flex_none()
            .h_full()
            .w(width)
            .bg(theme.sidebar)
            .text_color(theme.sidebar_foreground)
            .map(|this| match self.variant {
                SidebarVariant::Sidebar => this
                    .border_color(theme.sidebar_border)
                    .map(|this| if right { this.border_l_1() } else { this.border_r_1() }),
                SidebarVariant::Floating => this
                    .m_2()
                    .h_auto()
                    .rounded(theme.radius_lg())
                    .border_1()
                    .border_color(theme.sidebar_border)
                    .when(theme.shadows, |this| this.shadow_sm()),
                SidebarVariant::Inset => this,
            })
            .refine_style(&self.style)
            .children(self.header.map(|h| div().flex().flex_col().gap_2().p_2().child(h)))
            .child(
                div()
                    .id("sidebar-content")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    .gap_2()
                    .overflow_y_scroll()
                    .when(icon_mode, |this| this.items_center())
                    .children(groups),
            )
            .children(self.footer.map(|f| div().flex().flex_col().gap_2().p_2().child(f)))
            .into_any_element()
    }
}

/// The main content area next to a sidebar.
#[derive(IntoElement)]
pub struct SidebarInset {
    inset: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(SidebarInset);

impl SidebarInset {
    pub fn new() -> Self {
        Self { inset: false, children: Vec::new(), style: StyleRefinement::default() }
    }

    /// Rounded, inset look for `SidebarVariant::Inset`.
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }
}

impl Default for SidebarInset {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SidebarInset {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .relative()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .bg(theme.background)
            .when(self.inset, |this| {
                this.m_2().ml_0().rounded(theme.radius_xl()).when(theme.shadows, |this| this.shadow_sm())
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Toggles the sidebar with the given id.
#[derive(IntoElement)]
pub struct SidebarTrigger {
    sidebar: SharedString,
    style: StyleRefinement,
}

crate::impl_styled!(SidebarTrigger);

impl SidebarTrigger {
    pub fn new(sidebar: impl Into<SharedString>) -> Self {
        Self { sidebar: sidebar.into(), style: StyleRefinement::default() }
    }
}

impl RenderOnce for SidebarTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (accent, accent_fg) = (theme.accent, theme.accent_foreground);
        let sidebar = self.sidebar;
        div()
            .id(gpui::ElementId::Name(format!("{sidebar}-trigger").into()))
            .flex()
            .size_7()
            .items_center()
            .justify_center()
            .rounded(theme.radius_md())
            .cursor_pointer()
            .hover(move |s| s.bg(accent).text_color(accent_fg))
            .refine_style(&self.style)
            .child(Icon::new(IconName::PanelLeft))
            .on_click(move |_, _, cx| toggle_sidebar(sidebar.clone(), cx))
    }
}

/// `SidebarSeparator`
pub fn sidebar_separator(cx: &App) -> impl IntoElement {
    div().mx_2().h_px().bg(cx.theme().sidebar_border)
}

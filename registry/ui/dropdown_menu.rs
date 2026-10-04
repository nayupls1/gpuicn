//! Displays a menu to the user — such as a set of actions or functions —
//! triggered by a button.
//!
//! The menu model ([`MenuEntry`], [`MenuItem`]) and renderer ([`MenuList`])
//! are shared with `context_menu` and `menubar`.
//!
//! ```ignore
//! DropdownMenu::new("account")
//!     .trigger(Button::new("open").outline().label("Open"))
//!     .label("My Account")
//!     .separator()
//!     .item(MenuItem::new("Profile").shortcut("⇧⌘P").on_select(|_, _| println!("profile")))
//!     .item(MenuItem::new("Billing").icon(IconName::CreditCard))
//!     .submenu("Invite users", vec![MenuItem::new("Email").into(), MenuItem::new("Message").into()])
//!     .separator()
//!     .item(MenuItem::new("Log out").icon(IconName::LogOut).destructive())
//! ```

use gpui::{
    AnyElement, App, Bounds, ElementId, FocusHandle, FontWeight, IntoElement, Pixels, RenderOnce,
    SharedString, StyleRefinement, Window, anchored, canvas, div, point, prelude::*, px,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::popover::{Popover, PopoverHandle};
use super::theme::ActiveTheme;
use super::utils::{Align, Side, StyledExt};

type SelectHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// What a menu item shows in its leading indicator slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuIndicator {
    /// `DropdownMenuCheckboxItem`
    Checkbox(bool),
    /// `DropdownMenuRadioItem`
    Radio(bool),
}

#[derive(Clone)]
pub struct MenuItem {
    label: SharedString,
    icon: Option<IconName>,
    shortcut: Option<SharedString>,
    indicator: Option<MenuIndicator>,
    disabled: bool,
    destructive: bool,
    inset: bool,
    close_on_select: bool,
    on_select: Option<SelectHandler>,
}

impl MenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: None,
            indicator: None,
            disabled: false,
            destructive: false,
            inset: false,
            close_on_select: true,
            on_select: None,
        }
    }

    /// A checkbox item (`DropdownMenuCheckboxItem`).
    pub fn checkbox(label: impl Into<SharedString>, checked: bool) -> Self {
        let mut item = Self::new(label);
        item.indicator = Some(MenuIndicator::Checkbox(checked));
        item
    }

    /// A radio item (`DropdownMenuRadioItem`).
    pub fn radio(label: impl Into<SharedString>, selected: bool) -> Self {
        let mut item = Self::new(label);
        item.indicator = Some(MenuIndicator::Radio(selected));
        item
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// `variant="destructive"`.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Indent to line up with items that have an indicator.
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }

    /// Keep the menu open after selecting (handy for checkbox items).
    pub fn keep_open(mut self) -> Self {
        self.close_on_select = false;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

#[derive(Clone)]
pub enum MenuEntry {
    Item(MenuItem),
    Label(SharedString),
    Separator,
    Submenu { label: SharedString, icon: Option<IconName>, entries: Rc<Vec<MenuEntry>> },
}

impl From<MenuItem> for MenuEntry {
    fn from(item: MenuItem) -> Self {
        MenuEntry::Item(item)
    }
}

impl MenuEntry {
    fn is_selectable(&self) -> bool {
        match self {
            MenuEntry::Item(item) => !item.disabled,
            MenuEntry::Submenu { .. } => true,
            _ => false,
        }
    }
}

/// Builder methods shared by every component that holds a menu.
pub trait MenuBuilder: Sized {
    fn entries_mut(&mut self) -> &mut Vec<MenuEntry>;

    fn item(mut self, item: MenuItem) -> Self {
        self.entries_mut().push(MenuEntry::Item(item));
        self
    }

    fn items(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        self.entries_mut().extend(items.into_iter().map(MenuEntry::Item));
        self
    }

    /// `DropdownMenuLabel`
    fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.entries_mut().push(MenuEntry::Label(label.into()));
        self
    }

    /// `DropdownMenuSeparator`
    fn separator(mut self) -> Self {
        self.entries_mut().push(MenuEntry::Separator);
        self
    }

    /// `DropdownMenuSub`
    fn submenu(mut self, label: impl Into<SharedString>, entries: Vec<MenuEntry>) -> Self {
        self.entries_mut().push(MenuEntry::Submenu { label: label.into(), icon: None, entries: Rc::new(entries) });
        self
    }

    fn entry(mut self, entry: MenuEntry) -> Self {
        self.entries_mut().push(entry);
        self
    }
}

struct MenuState {
    focus_handle: FocusHandle,
    highlighted: Option<usize>,
    open_submenu: Option<usize>,
    item_bounds: Vec<Bounds<Pixels>>,
    focused_once: bool,
}

/// Renders a list of [`MenuEntry`]s as a menu panel with keyboard navigation
/// (↑ ↓ Enter →  ←).
#[derive(IntoElement)]
pub struct MenuList {
    id: ElementId,
    entries: Rc<Vec<MenuEntry>>,
    /// Closes the whole menu (and all submenus).
    on_close: Rc<dyn Fn(&mut Window, &mut App)>,
    /// Called on ← inside a submenu.
    on_back: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(MenuList);

impl MenuList {
    pub fn new(
        id: impl Into<ElementId>,
        entries: Rc<Vec<MenuEntry>>,
        on_close: Rc<dyn Fn(&mut Window, &mut App)>,
    ) -> Self {
        Self { id: id.into(), entries, on_close, on_back: None, style: StyleRefinement::default() }
    }

    /// A [`MenuList`] that closes `popover` when an item is chosen.
    pub fn for_popover(id: impl Into<ElementId>, entries: Rc<Vec<MenuEntry>>, popover: &PopoverHandle) -> Self {
        let popover = popover.clone();
        Self::new(id, entries, Rc::new(move |window, cx| popover.close(window, cx)))
    }

    fn on_back(mut self, on_back: Rc<dyn Fn(&mut Window, &mut App)>) -> Self {
        self.on_back = Some(on_back);
        self
    }
}

fn select_entry(entry: &MenuEntry, on_close: &Rc<dyn Fn(&mut Window, &mut App)>, window: &mut Window, cx: &mut App) {
    if let MenuEntry::Item(item) = entry {
        if item.disabled {
            return;
        }
        if item.close_on_select {
            on_close(window, cx);
        }
        if let Some(handler) = &item.on_select {
            handler(window, cx);
        }
    }
}

impl RenderOnce for MenuList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| MenuState {
            focus_handle: cx.focus_handle(),
            highlighted: None,
            open_submenu: None,
            item_bounds: Vec::new(),
            focused_once: false,
        });
        let (focus_handle, highlighted, open_submenu, item_bounds) = {
            let s = state.read(cx);
            (s.focus_handle.clone(), s.highlighted, s.open_submenu, s.item_bounds.clone())
        };
        if !state.read(cx).focused_once {
            window.focus(&focus_handle);
            state.update(cx, |s, _| s.focused_once = true);
        }

        let theme = cx.theme();
        let entries = self.entries.clone();
        let on_close = self.on_close.clone();
        let accent = theme.accent;
        let accent_fg = theme.accent_foreground;
        let destructive = theme.destructive;
        let muted_fg = theme.muted_foreground;
        let any_indicator = entries.iter().any(|e| matches!(e, MenuEntry::Item(MenuItem { indicator: Some(_), .. })));

        let mut rows: Vec<AnyElement> = Vec::new();
        for (ix, entry) in entries.iter().enumerate() {
            let is_highlighted = highlighted == Some(ix) || open_submenu == Some(ix);
            let bounds_recorder = {
                let state = state.clone();
                canvas(
                    move |bounds, _, cx| {
                        state.update(cx, |s, _| {
                            if s.item_bounds.len() <= ix {
                                s.item_bounds.resize(ix + 1, Bounds::default());
                            }
                            s.item_bounds[ix] = bounds;
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full()
            };
            let row = match entry {
                MenuEntry::Label(label) => div()
                    .px_2()
                    .py_1p5()
                    .when(any_indicator, |this| this.pl_8())
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .child(label.clone())
                    .into_any_element(),
                MenuEntry::Separator => div().h_px().mx_neg_1().my_1().bg(theme.border).into_any_element(),
                MenuEntry::Item(item) => {
                    let inset = item.inset || item.indicator.is_some() || (any_indicator && item.icon.is_none());
                    let fg = if item.destructive { destructive } else { theme.popover_foreground };
                    let hover_bg = if item.destructive { destructive.opacity(0.1) } else { accent };
                    let hover_fg = if item.destructive { destructive } else { accent_fg };
                    div()
                        .id(("item", ix))
                        .relative()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .py_1p5()
                        .when(inset, |this| this.pl_8())
                        .rounded(theme.radius_sm())
                        .text_sm()
                        .text_color(fg)
                        .when(item.disabled, |this| this.opacity(0.5))
                        .when(is_highlighted && !item.disabled, |this| this.bg(hover_bg).text_color(hover_fg))
                        .children(item.indicator.map(|indicator| {
                            div()
                                .absolute()
                                .left_2()
                                .flex()
                                .size_3p5()
                                .items_center()
                                .justify_center()
                                .map(|this| match indicator {
                                    MenuIndicator::Checkbox(true) => this.child(Icon::new(IconName::Check)),
                                    MenuIndicator::Radio(true) => this.child(div().size_2().rounded_full().bg(fg)),
                                    _ => this,
                                })
                        }))
                        .children(item.icon.map(|icon| Icon::new(icon).text_color(if item.destructive { destructive } else { muted_fg })))
                        .child(div().flex_1().child(item.label.clone()))
                        .children(item.shortcut.clone().map(|shortcut| {
                            div().ml_auto().pl_4().text_xs().text_color(muted_fg).child(shortcut)
                        }))
                        .when(!item.disabled, |this| {
                            let state = state.clone();
                            let entries = entries.clone();
                            let on_close = on_close.clone();
                            this.on_mouse_move(move |_, window, cx| {
                                state.update(cx, |s, cx| {
                                    if s.highlighted != Some(ix) || s.open_submenu.is_some() {
                                        s.highlighted = Some(ix);
                                        s.open_submenu = None;
                                        window.focus(&s.focus_handle);
                                        cx.notify();
                                    }
                                })
                            })
                            .on_click(move |_, window, cx| select_entry(&entries[ix], &on_close, window, cx))
                        })
                        .into_any_element()
                }
                MenuEntry::Submenu { label, icon, .. } => {
                    let state = state.clone();
                    div()
                        .id(("submenu", ix))
                        .relative()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .py_1p5()
                        .when(any_indicator && icon.is_none(), |this| this.pl_8())
                        .rounded(theme.radius_sm())
                        .text_sm()
                        .when(is_highlighted, |this| this.bg(accent).text_color(accent_fg))
                        .children(icon.map(|icon| Icon::new(icon).text_color(muted_fg)))
                        .child(div().flex_1().child(label.clone()))
                        .child(Icon::new(IconName::ChevronRight).ml_auto())
                        .on_mouse_move(move |_, _, cx| {
                            state.update(cx, |s, cx| {
                                if s.open_submenu != Some(ix) {
                                    s.highlighted = Some(ix);
                                    s.open_submenu = Some(ix);
                                    cx.notify();
                                }
                            })
                        })
                        .into_any_element()
                }
            };
            rows.push(div().relative().child(row).child(bounds_recorder).into_any_element());
        }

        // An open submenu floats to the right of its trigger row.
        let submenu = open_submenu.and_then(|ix| {
            let MenuEntry::Submenu { entries, .. } = &self.entries[ix] else { return None };
            let bounds = item_bounds.get(ix).copied()?;
            let state = state.clone();
            let back: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(move |window, cx| {
                state.update(cx, |s, cx| {
                    s.open_submenu = None;
                    cx.notify();
                    window.focus(&s.focus_handle);
                })
            });
            Some(
                anchored()
                    .position(point(bounds.right() + px(8.), bounds.top() - px(5.)))
                    .snap_to_window_with_margin(px(8.))
                    .child(
                        MenuList::new(("sub", ix), entries.clone(), self.on_close.clone())
                            .on_back(back),
                    ),
            )
        });

        let selectable: Vec<usize> = (0..entries.len()).filter(|ix| entries[*ix].is_selectable()).collect();
        let key_state = state.clone();
        let key_entries = entries.clone();
        let key_close = self.on_close.clone();
        let on_back = self.on_back.clone();

        div()
            .id(self.id)
            .track_focus(&focus_handle)
            .occlude()
            .min_w_32()
            .p_1()
            .flex()
            .flex_col()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.border)
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .when(theme.shadows, |this| this.shadow_md())
            .refine_style(&self.style)
            .on_hover({
                let state = state.clone();
                move |hovered, _, cx| {
                    if !hovered {
                        state.update(cx, |s, cx| {
                            if s.open_submenu.is_none() {
                                s.highlighted = None;
                                cx.notify();
                            }
                        })
                    }
                }
            })
            .on_key_down(move |event, window, cx| {
                let key = event.keystroke.key.as_str();
                let current = key_state.read(cx).highlighted;
                let pos = current.and_then(|c| selectable.iter().position(|s| *s == c));
                let next = match key {
                    "down" => Some(match pos {
                        Some(p) => selectable[(p + 1) % selectable.len()],
                        None => *selectable.first().unwrap_or(&0),
                    }),
                    "up" => Some(match pos {
                        Some(p) => selectable[(p + selectable.len() - 1) % selectable.len()],
                        None => *selectable.last().unwrap_or(&0),
                    }),
                    _ => None,
                };
                if let Some(next) = next.filter(|_| !selectable.is_empty()) {
                    cx.stop_propagation();
                    key_state.update(cx, |s, cx| {
                        s.highlighted = Some(next);
                        cx.notify();
                    });
                    return;
                }
                match key {
                    "enter" | "space" => {
                        if let Some(ix) = current {
                            cx.stop_propagation();
                            if matches!(key_entries[ix], MenuEntry::Submenu { .. }) {
                                key_state.update(cx, |s, cx| {
                                    s.open_submenu = Some(ix);
                                    cx.notify();
                                });
                            } else {
                                select_entry(&key_entries[ix], &key_close, window, cx);
                            }
                        }
                    }
                    "right" => {
                        if let Some(ix) = current.filter(|ix| matches!(key_entries[*ix], MenuEntry::Submenu { .. })) {
                            cx.stop_propagation();
                            key_state.update(cx, |s, cx| {
                                s.open_submenu = Some(ix);
                                cx.notify();
                            });
                        }
                    }
                    "left" => {
                        if let Some(on_back) = &on_back {
                            cx.stop_propagation();
                            on_back(window, cx);
                        }
                    }
                    _ => {}
                }
            })
            .children(rows)
            .children(submenu)
    }
}

#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    trigger: Option<AnyElement>,
    entries: Vec<MenuEntry>,
    side: Side,
    align: Align,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(DropdownMenu);

impl MenuBuilder for DropdownMenu {
    fn entries_mut(&mut self) -> &mut Vec<MenuEntry> {
        &mut self.entries
    }
}

impl DropdownMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            entries: Vec::new(),
            side: Side::Bottom,
            align: Align::Start,
            open: None,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let entries = Rc::new(self.entries);
        let style = self.style;
        let mut popover = Popover::new(self.id)
            .unstyled()
            .side(self.side)
            .align(self.align)
            .content_with_handle(move |popover, _, _| {
                MenuList::for_popover("menu", entries.clone(), popover).refine_style(&style)
            });
        if let Some(trigger) = self.trigger {
            popover = popover.trigger(trigger);
        }
        if let Some(open) = self.open {
            popover = popover.open(open);
        }
        if let Some(handler) = self.on_open_change {
            popover = popover.on_open_change(move |open, window, cx| handler(open, window, cx));
        }
        popover
    }
}

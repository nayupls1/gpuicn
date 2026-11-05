//! Displays a menu to the user — such as a set of actions or functions —
//! triggered by a button.
//!
//! The menu model ([`Menu`], [`MenuEntry`], [`MenuItem`]) and renderer
//! ([`MenuList`]) are shared with `context_menu` and `menubar`.
//!
//! ```ignore
//! <DropdownMenu
//!     trigger={<Button outline label="Open" />}
//!     menu={Menu::new()
//!         .label("My Account")
//!         .separator()
//!         .item(MenuItem::new("Profile").shortcut("⇧⌘P").on_select(|| println!("profile")))
//!         .item(MenuItem::new("Billing").icon(IconName::CreditCard))
//!         .submenu("Invite users", vec![MenuItem::new("Email").into(), MenuItem::new("Message").into()])
//!         .separator()
//!         .item(MenuItem::new("Log out").icon(IconName::LogOut).destructive())}
//! />
//!
//! // Items that read signals: pass a closure, the menu follows them while open.
//! <DropdownMenu
//!     trigger={<Button outline label="View" />}
//!     menu={move || Menu::new().item(
//!         MenuItem::checkbox("Status Bar", status.get()).keep_open().on_select(move || status.update(|s| *s = !*s)),
//!     )}
//! />
//! ```
//!
//! gpuicn's components *are* the menu builders (`DropdownMenu::new(id).item(..)`);
//! an rsx component's builder already has methods for its props and its
//! forwarded attributes (`label` among them), so here the entries are built
//! with a [`Menu`] value and passed as `menu={..}`. [`MenuBuilder`] is the
//! same trait, so the chains read the same.

use std::rc::Rc;

use rsx::prelude::*;
use rsx::{window_size, Bounds, Point, Size};

use super::icon::{Icon, IconName};
use super::popover::{Popover, PopoverHandle};
use super::theme::{theme, ColorExt};
use super::utils::{Align, Side};

type SelectHandler = Rc<dyn Fn()>;

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

    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self {
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

/// The entries of a menu, built with [`MenuBuilder`]'s methods and passed
/// to [`DropdownMenu`], `ContextMenu`, `MenubarMenu` or [`MenuList`].
#[derive(Clone, Default)]
pub struct Menu {
    pub entries: Vec<MenuEntry>,
}

impl Menu {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MenuBuilder for Menu {
    fn entries_mut(&mut self) -> &mut Vec<MenuEntry> {
        &mut self.entries
    }
}

impl From<Vec<MenuEntry>> for Menu {
    fn from(entries: Vec<MenuEntry>) -> Self {
        Menu { entries }
    }
}

fn select_entry(entry: &MenuEntry, close: &dyn Fn()) {
    if let MenuEntry::Item(item) = entry {
        if item.disabled {
            return;
        }
        if item.close_on_select {
            close();
        }
        if let Some(handler) = &item.on_select {
            handler();
        }
    }
}

/// Whether `point` is inside `bounds`.
pub fn bounds_contain(bounds: &Bounds, point: Point) -> bool {
    point.x >= bounds.origin.x
        && point.y >= bounds.origin.y
        && point.x < bounds.origin.x + bounds.size.width
        && point.y < bounds.origin.y + bounds.size.height
}

/// Renders a list of [`MenuEntry`]s as a menu panel with keyboard navigation
/// (↑ ↓ Enter → ←).
#[component]
pub fn MenuList(
    /// The entries: a [`Menu`], a `Vec<MenuEntry>`, or a closure returning
    /// one (it's re-run when the signals it reads change).
    #[prop(optional)]
    entries: Accessor<Menu>,
    /// Closes the whole menu (and all submenus). Defaults to closing `popover`.
    on_close: Option<Callback<()>>,
    /// Called on ← inside a submenu.
    on_back: Option<Callback<()>>,
    /// The popover it lives in, for closing it and capping its height to the room there.
    popover: Option<PopoverHandle>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let root = NodeRef::new();
    let highlighted = signal(None::<usize>);
    let open_submenu = signal(None::<usize>);
    let nested = on_back.is_some();
    let close = move || match on_close {
        Some(f) => f.run(),
        None => {
            if let Some(p) = popover {
                p.close()
            }
        }
    };
    // Take focus so the arrow keys reach this list (gpui: focus on first
    // render). The popover's panel asks too, but the innermost request wins.
    root.focus();

    // One handle per row, to scroll the highlighted row into view and to
    // place an open submenu next to its row.
    let row_refs = memo(move || {
        let n = entries.with(|m| m.entries.len());
        (0..n).map(|_| NodeRef::new()).collect::<Vec<_>>()
    });

    let rows = move || {
        let menu = entries.get();
        let refs = row_refs.get();
        let any_indicator = menu.entries.iter().any(|e| matches!(e, MenuEntry::Item(MenuItem { indicator: Some(_), .. })));
        menu.entries
            .into_iter()
            .enumerate()
            .map(|(ix, entry)| {
                let is_highlighted = move || highlighted.get() == Some(ix) || open_submenu.get() == Some(ix);
                let row_ref = refs.get(ix).copied().unwrap_or_default();
                match entry {
                    MenuEntry::Label(label) => <div class="relative flex flex-col">
                        <div class="px-2 py-1.5 text-sm leading-[1.25rem] font-medium" class:pl-8={any_indicator}>{label}</div>
                    </div>,
                    MenuEntry::Separator => <div class="relative flex flex-col">
                        <div class="h-px -mx-1 my-1" bg={t.get().border} />
                    </div>,
                    MenuEntry::Item(item) => {
                        let item = Rc::new(item);
                        let inset = item.inset || item.indicator.is_some() || (any_indicator && item.icon.is_none());
                        let (disabled, destructive) = (item.disabled, item.destructive);
                        let row = move || {
                            let theme = t.get();
                            let fg = if destructive { theme.destructive } else { theme.popover_foreground };
                            let hover_bg = if destructive { theme.destructive.opacity(0.1) } else { theme.accent };
                            let hover_fg = if destructive { theme.destructive } else { theme.accent_foreground };
                            let lit = is_highlighted() && !disabled;
                            let muted = if destructive { theme.destructive } else { theme.muted_foreground };
                            let select = item.clone();
                            <div
                                class="relative flex items-center gap-2 px-2 py-1.5 text-sm leading-[1.25rem]"
                                class:pl-8={inset}
                                corner_radius={theme.radius_sm()}
                                bg={if lit { hover_bg } else { Color::TRANSPARENT }}
                                text_color={if lit { hover_fg } else { fg }}
                                class:opacity-50={disabled}
                                onMouseMove={move |_| {
                                    if !disabled && (highlighted.get_untracked() != Some(ix) || open_submenu.get_untracked().is_some()) {
                                        highlighted.set(Some(ix));
                                        open_submenu.set(None);
                                        root.focus();
                                    }
                                }}
                                onClick={move |_| select_entry(&MenuEntry::Item((*select).clone()), &close)}
                            >
                                {item.indicator.map(|indicator| <div class="absolute left-2 flex size-3.5 items-center justify-center">
                                    {match indicator {
                                        MenuIndicator::Checkbox(true) => <Icon name={IconName::Check} />,
                                        MenuIndicator::Radio(true) => <div class="size-2 rounded-full" bg={fg} />,
                                        _ => ().into_element(),
                                    }}
                                </div>)}
                                {item.icon.map(|icon| icon.element().text_color(muted))}
                                <div class="flex-1 min-w-0">{item.label.clone()}</div>
                                {item.shortcut.clone().map(|shortcut| <div
                                    class="ml-auto pl-4 text-xs leading-[1rem]"
                                    text_color={theme.muted_foreground}
                                >{shortcut}</div>)}
                            </div>
                        };
                        <div ref={row_ref} class="relative flex flex-col">{row}</div>
                    }
                    MenuEntry::Submenu { label, icon, .. } => {
                        let row = move || {
                            let theme = t.get();
                            let lit = is_highlighted();
                            <div
                                class="relative flex items-center gap-2 px-2 py-1.5 text-sm leading-[1.25rem]"
                                class:pl-8={any_indicator && icon.is_none()}
                                corner_radius={theme.radius_sm()}
                                bg={if lit { theme.accent } else { Color::TRANSPARENT }}
                                text_color={if lit { theme.accent_foreground } else { theme.popover_foreground }}
                                onMouseMove={move |_| {
                                    if open_submenu.get_untracked() != Some(ix) {
                                        highlighted.set(Some(ix));
                                        open_submenu.set(Some(ix));
                                    }
                                }}
                            >
                                {icon.map(|icon| icon.element().text_color(theme.muted_foreground))}
                                <div class="flex-1 min-w-0">{label.clone()}</div>
                                <Icon name={IconName::ChevronRight} class="ml-auto" />
                            </div>
                        };
                        <div ref={row_ref} class="relative flex flex-col">{row}</div>
                    }
                }
            })
            .collect::<Vec<_>>()
    };

    // An open submenu floats to the right of its trigger row (kept 8px
    // inside the window, like gpui's `snap_to_window_with_margin`).
    let submenu = move || {
        let ix = open_submenu.get()?;
        let row = row_refs.get().get(ix).copied()?;
        let sub = NodeRef::new();
        let place = move |el: rsx::Div| {
            let (Some(row), Some(list)) = (row.bounds(), root.bounds()) else { return el.opacity(0.0) };
            let size = sub.bounds().map(|b| b.size).unwrap_or(Size { width: 0.0, height: 0.0 });
            let vp = window_size();
            let x = (row.origin.x + row.size.width + 8.0).min(vp.width - 8.0 - size.width).max(8.0);
            let y = (row.origin.y - 5.0).min(vp.height - 8.0 - size.height).max(8.0);
            // Absolute children are placed from the list's padding box (inside its 1px border).
            el.left(px(x - list.origin.x - 1.0))
                .top(px(y - list.origin.y - 1.0))
                .opacity(if size.width > 0.0 { 1.0 } else { 0.0 })
        };
        let sub_entries = move || {
            entries.with(|m| match m.entries.get(ix) {
                Some(MenuEntry::Submenu { entries, .. }) => Menu { entries: (**entries).clone() },
                _ => Menu::default(),
            })
        };
        Some(<div ref={sub} class="absolute flex" bind={place}>
            <MenuList
                entries={sub_entries}
                on_close={move |_| close()}
                on_back={move |_| {
                    open_submenu.set(None);
                    root.focus();
                }}
                popover={popover}
            />
        </div>)
    };

    let on_key = move |e: &KeyboardEvent| {
        let list = entries.get_untracked().entries;
        let selectable: Vec<usize> = (0..list.len()).filter(|ix| list[*ix].is_selectable()).collect();
        let current = highlighted.get_untracked();
        let pos = current.and_then(|c| selectable.iter().position(|s| *s == c));
        let next = match e.key.as_str() {
            "ArrowDown" => Some(match pos {
                Some(p) => selectable[(p + 1) % selectable.len()],
                None => *selectable.first().unwrap_or(&0),
            }),
            "ArrowUp" => Some(match pos {
                Some(p) => selectable[(p + selectable.len() - 1) % selectable.len()],
                None => *selectable.last().unwrap_or(&0),
            }),
            _ => None,
        };
        if let Some(next) = next.filter(|_| !selectable.is_empty()) {
            e.stop_propagation();
            highlighted.set(Some(next));
            if let Some(row) = row_refs.get_untracked().get(next) {
                row.scroll_into_view();
            }
            return;
        }
        match e.key.as_str() {
            "Enter" | " " => {
                if let Some(ix) = current.filter(|ix| *ix < list.len()) {
                    e.stop_propagation();
                    e.prevent_default();
                    if matches!(list[ix], MenuEntry::Submenu { .. }) {
                        open_submenu.set(Some(ix));
                    } else {
                        select_entry(&list[ix], &close);
                    }
                }
            }
            "ArrowRight" => {
                if let Some(ix) = current.filter(|ix| matches!(list.get(*ix), Some(MenuEntry::Submenu { .. }))) {
                    e.stop_propagation();
                    open_submenu.set(Some(ix));
                }
            }
            "ArrowLeft" => {
                if let Some(on_back) = on_back {
                    e.stop_propagation();
                    on_back.run();
                }
            }
            _ => {}
        }
    };

    // Too tall for the room beside the trigger: scroll rather than let the
    // window edge push the menu over its trigger. Only once the cap is hit,
    // since scrolling clips the submenus floating beside the list.
    let cap = move |el: rsx::Div| {
        let max = popover.filter(|_| !nested).and_then(|p| p.available_height());
        let height = root.bounds().map(|b| b.size.height).unwrap_or(0.0);
        match max {
            Some(max) if height > 0.0 && height + 1.0 >= max => el.max_h(px(max)).class("overflow-y-scroll"),
            Some(max) => el.max_h(px(max)),
            None => el,
        }
    };

    // Taffy doesn't stretch the rows of a column whose width comes from
    // `min-width` while it's sized to its content (as floating panels
    // are), so separators collapse and shortcuts don't right-align. A
    // zero-height strut as wide as the min width fixes that.
    let min_width = {
        let probe = rsx::element("div").spread(attrs.clone());
        probe.style.min_size.width.unwrap_or(px(128.0))
    };
    let attrs = StoredValue::new(attrs);

    move || {
        let theme = t.get();
        <div
            ref={root}
            class="relative flex flex-col p-1 border"
            class:shadow-md={theme.shadows}
            corner_radius={theme.radius_md()}
            border_color={theme.border}
            bg={theme.popover}
            text_color={theme.popover_foreground}
            bind={cap}
            onKeyDown={on_key}
            onHover={move |h: &bool| {
                if !*h && open_submenu.get_untracked().is_none() {
                    highlighted.set(None);
                }
            }}
            {...attrs.with_value(|a| a.clone())}
        >
            <div class="h-0" min_w={min_width} />
            {rows}
            {submenu}
        </div>
    }
}

impl MenuList {
    /// A [`MenuList`] that closes `popover` when an item is chosen.
    pub fn for_popover(entries: Accessor<Menu>, popover: PopoverHandle) -> Self {
        MenuList::new().entries(entries).popover(popover)
    }
}

#[component]
pub fn DropdownMenu(
    /// The element that opens the menu.
    trigger: Option<Children>,
    /// The entries (see [`Menu`]); a closure keeps them following signals.
    #[prop(optional)]
    menu: Accessor<Menu>,
    #[prop(default = Side::Bottom)] side: Side,
    #[prop(default = Align::Start)] align: Align,
    /// Control the open state from outside.
    open: Option<Accessor<bool>>,
    on_open_change: Option<Callback<bool>>,
    /// Share state with the caller (open or close it from code).
    handle: Option<PopoverHandle>,
    /// Styles for the menu panel (`class="w-[224px]"`).
    attrs: Attributes,
) -> impl IntoElement {
    let attrs = StoredValue::new(attrs);
    <Popover
        unstyled
        side={side}
        align={align}
        open={open}
        on_open_change={move |o: &bool| if let Some(f) = on_open_change { f.call(o) }}
        handle={handle}
        trigger={trigger}
        content={move |popover: PopoverHandle| <MenuList entries={menu} popover={popover} {...attrs.with_value(|a| a.clone())} />}
    />
}

//! Fast, composable, unstyled command menu (shadcn's `cmdk` wrapper).
//!
//! ```ignore
//! Command::new("palette")
//!     .placeholder("Type a command or search...")
//!     .group(CommandGroup::new("Suggestions")
//!         .item(CommandItem::new("Calendar").icon(IconName::Calendar))
//!         .item(CommandItem::new("Search Emoji").icon(IconName::Search)))
//!     .group(CommandGroup::new("Settings")
//!         .item(CommandItem::new("Profile").icon(IconName::User).shortcut("⌘P").on_select(|_, _| {})))
//!
//! // ⌘K palette:
//! CommandDialog::new("k").open(self.open).on_open_change(..).group(..)
//! ```

use gpui::{
    App, ElementId, FontWeight, IntoElement, RenderOnce, ScrollHandle, SharedString,
    StyleRefinement, Window, div, prelude::*, px,
};
use std::rc::Rc;

use super::dialog::{ModalPlacement, render_modal, use_modal};
use super::icon::{Icon, IconName};
use super::input::{self, Input};
use super::theme::ActiveTheme;
use super::utils::StyledExt;

type SelectHandler = Rc<dyn Fn(&mut Window, &mut App)>;

#[derive(Clone)]
pub struct CommandItem {
    label: SharedString,
    value: SharedString,
    keywords: Vec<SharedString>,
    icon: Option<IconName>,
    shortcut: Option<SharedString>,
    checked: Option<bool>,
    disabled: bool,
    on_select: Option<SelectHandler>,
}

impl CommandItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            keywords: Vec::new(),
            icon: None,
            shortcut: None,
            checked: None,
            disabled: false,
            on_select: None,
        }
    }

    /// Identifier passed to `Command::on_select` (defaults to the label).
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    /// Extra search terms.
    pub fn keywords(mut self, keywords: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Show a trailing check mark (visible when `true`), as in comboboxes.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    fn matches(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query = query.to_lowercase();
        std::iter::once(&self.label)
            .chain(&self.keywords)
            .any(|text| fuzzy_match(&text.to_lowercase(), &query))
    }
}

/// Case-insensitive subsequence match (`"cal"` matches `"Calendar"`).
pub fn fuzzy_match(haystack: &str, needle: &str) -> bool {
    let mut chars = haystack.chars();
    needle.chars().all(|n| chars.any(|h| h == n))
}

#[derive(Clone)]
pub struct CommandGroup {
    heading: Option<SharedString>,
    items: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(heading: impl Into<SharedString>) -> Self {
        Self { heading: Some(heading.into()), items: Vec::new() }
    }

    /// A group without a heading.
    pub fn untitled() -> Self {
        Self { heading: None, items: Vec::new() }
    }

    pub fn item(mut self, item: CommandItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = CommandItem>) -> Self {
        self.items.extend(items);
        self
    }
}

struct CommandState {
    highlighted: usize,
    last_query: SharedString,
    scroll: ScrollHandle,
}

type ValueHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Command {
    id: ElementId,
    placeholder: SharedString,
    empty: SharedString,
    groups: Vec<CommandGroup>,
    on_select: Option<ValueHandler>,
    max_height: gpui::Pixels,
    style: StyleRefinement,
}

crate::impl_styled!(Command);

impl Command {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            placeholder: "Type a command or search...".into(),
            empty: "No results found.".into(),
            groups: Vec::new(),
            on_select: None,
            max_height: px(300.),
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// `CommandEmpty` text.
    pub fn empty(mut self, empty: impl Into<SharedString>) -> Self {
        self.empty = empty.into();
        self
    }

    pub fn group(mut self, group: CommandGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Called with the item's value whenever any item is selected.
    pub fn on_select(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    /// Height of the scrollable list (default 300px).
    pub fn max_height(mut self, height: impl Into<gpui::Pixels>) -> Self {
        self.max_height = height.into();
        self
    }
}

fn run(item: &CommandItem, on_select: &Option<ValueHandler>, window: &mut Window, cx: &mut App) {
    if item.disabled {
        return;
    }
    if let Some(handler) = on_select {
        handler(&item.value, window, cx);
    }
    if let Some(handler) = &item.on_select {
        handler(window, cx);
    }
}

impl RenderOnce for Command {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| CommandState {
            highlighted: 0,
            last_query: SharedString::default(),
            scroll: ScrollHandle::new(),
        });
        let input = Input::new("command-input").placeholder(self.placeholder.clone()).autofocus().unstyled();
        let input_state = input.state(window, cx);
        let query = input_state.read(cx).text().clone();
        if state.read(cx).last_query != query {
            state.update(cx, |s, _| {
                s.last_query = query.clone();
                s.highlighted = 0;
            });
        }

        // Flatten the visible items (keeping group boundaries for headings).
        let groups: Vec<(Option<SharedString>, Vec<CommandItem>)> = self
            .groups
            .iter()
            .map(|g| (g.heading.clone(), g.items.iter().filter(|i| i.matches(&query)).cloned().collect::<Vec<_>>()))
            .filter(|(_, items)| !items.is_empty())
            .collect();
        let items: Rc<Vec<CommandItem>> = Rc::new(groups.iter().flat_map(|(_, items)| items.clone()).collect());
        let count = items.len();
        let (highlighted, scroll) = {
            let s = state.read(cx);
            (s.highlighted.min(count.saturating_sub(1)), s.scroll.clone())
        };

        let theme = cx.theme();
        let (accent, accent_fg, muted_fg) = (theme.accent, theme.accent_foreground, theme.muted_foreground);
        let radius = theme.radius_sm();
        let mut rows = Vec::new();
        let mut item_ix = 0;
        for (group_ix, (heading, group_items)) in groups.iter().enumerate() {
            if group_ix > 0 {
                rows.push(div().h_px().mx_neg_1().bg(theme.border).into_any_element());
            }
            let mut group = div().flex().flex_col().p_1().text_color(theme.foreground);
            if let Some(heading) = heading {
                group = group.child(
                    div().px_2().py_1p5().text_xs().font_weight(FontWeight::MEDIUM).text_color(muted_fg).child(heading.clone()),
                );
            }
            for item in group_items {
                let ix = item_ix;
                item_ix += 1;
                let selected = ix == highlighted;
                let state = state.clone();
                let items = items.clone();
                let on_select = self.on_select.clone();
                group = group.child(
                    div()
                        .id(("command-item", ix))
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .py_1p5()
                        .rounded(radius)
                        .text_sm()
                        .when(item.disabled, |this| this.opacity(0.5))
                        .when(selected && !item.disabled, |this| this.bg(accent).text_color(accent_fg))
                        .children(item.icon.map(|icon| Icon::new(icon).text_color(muted_fg)))
                        .child(div().flex_1().min_w_0().child(item.label.clone()))
                        .children(item.shortcut.clone().map(|s| div().ml_auto().text_xs().text_color(muted_fg).child(s)))
                        .children(item.checked.map(|checked| {
                            Icon::new(IconName::Check).ml_auto().opacity(if checked { 1. } else { 0. })
                        }))
                        .on_mouse_move(move |_, _, cx| {
                            state.update(cx, |s, cx| {
                                if s.highlighted != ix {
                                    s.highlighted = ix;
                                    cx.notify();
                                }
                            })
                        })
                        .on_click(move |_, window, cx| run(&items[ix], &on_select, window, cx)),
                );
            }
            rows.push(group.into_any_element());
        }

        let move_by = {
            let state = state.clone();
            move |delta: isize, cx: &mut App| {
                if count == 0 {
                    return;
                }
                state.update(cx, |s, cx| {
                    s.highlighted = (s.highlighted as isize + delta).rem_euclid(count as isize) as usize;
                    cx.notify();
                });
            }
        };
        let (move_up, move_down) = (move_by.clone(), move_by);
        let enter_items = items.clone();
        let enter_select = self.on_select.clone();
        let enter_state = state.clone();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w_full()
            .overflow_hidden()
            .rounded(theme.radius_md())
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .refine_style(&self.style)
            .capture_action(move |_: &input::Up, _, cx| {
                cx.stop_propagation();
                move_up(-1, cx)
            })
            .capture_action(move |_: &input::Down, _, cx| {
                cx.stop_propagation();
                move_down(1, cx)
            })
            .capture_action(move |_: &input::Enter, window, cx| {
                cx.stop_propagation();
                let ix = enter_state.read(cx).highlighted;
                if let Some(item) = enter_items.get(ix) {
                    run(item, &enter_select, window, cx);
                }
            })
            .child(
                div()
                    .flex()
                    .h_9()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(Icon::new(IconName::Search).opacity(0.5))
                    .child(input),
            )
            .child(
                div()
                    .id("command-list")
                    .max_h(self.max_height)
                    .overflow_y_scroll()
                    .track_scroll(&scroll)
                    .when(count == 0, |this| {
                        this.child(div().py_6().text_center().text_sm().child(self.empty.clone()))
                    })
                    .children(rows),
            )
    }
}

/// [`Command`] inside a [`Dialog`](super::dialog::Dialog) — the ⌘K palette.
#[derive(IntoElement)]
pub struct CommandDialog {
    id: ElementId,
    command: Option<Command>,
    open: Option<bool>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
}

impl CommandDialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self { command: Some(Command::new(("command", 0usize))), id, open: None, on_open_change: None }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    /// Configure the inner [`Command`].
    pub fn command(mut self, f: impl FnOnce(Command) -> Command) -> Self {
        self.command = self.command.map(f);
        self
    }

    pub fn placeholder(self, placeholder: impl Into<SharedString>) -> Self {
        self.command(|c| c.placeholder(placeholder))
    }

    pub fn group(self, group: CommandGroup) -> Self {
        self.command(|c| c.group(group))
    }
}

impl RenderOnce for CommandDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let modal = use_modal(&self.id, self.open, self.on_open_change, window, cx);
        let width = px(512.).min(window.viewport_size().width - px(32.));
        if let Some(command) = self.command {
            let groups = command.groups.clone();
            let (placeholder, empty, on_select, max_height) =
                (command.placeholder.clone(), command.empty.clone(), command.on_select.clone(), command.max_height);
            render_modal(&modal, ModalPlacement::Center, true, window, cx, move |handle, _, cx| {
                let theme = cx.theme();
                let close = handle.clone();
                let mut command = Command::new("command-dialog").placeholder(placeholder).empty(empty).max_height(max_height);
                command.groups = groups;
                command.on_select = Some(Rc::new(move |value: &SharedString, window: &mut Window, cx: &mut App| {
                    close.close(window, cx);
                    if let Some(handler) = &on_select {
                        handler(value, window, cx);
                    }
                }));
                div()
                    .w(width)
                    .overflow_hidden()
                    .rounded(theme.radius_lg())
                    .border_1()
                    .border_color(theme.border)
                    .when(theme.shadows, |this| this.shadow_lg())
                    .child(command)
                    .into_any_element()
            });
        }
        div().id(self.id)
    }
}

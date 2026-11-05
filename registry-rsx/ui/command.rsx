//! Fast, composable, unstyled command menu (shadcn's `cmdk` wrapper).
//!
//! ```ignore
//! <Command
//!     placeholder="Type a command or search..."
//!     groups={vec![
//!         CommandGroup::new("Suggestions")
//!             .item(CommandItem::new("Calendar").icon(IconName::Calendar))
//!             .item(CommandItem::new("Search Emoji").icon(IconName::Search)),
//!         CommandGroup::new("Settings")
//!             .item(CommandItem::new("Profile").icon(IconName::User).shortcut("⌘P").on_select(|| {})),
//!     ]}
//!     onSelect={|value: &SharedString| println!("{value}")}
//! />
//!
//! // ⌘K palette:
//! let open = signal(false);
//! <CommandDialog open={open} onOpenChange={move |o: &bool| open.set(*o)} groups={..} />
//! ```
//!
//! gpuicn's `.group(..)` builder becomes the `groups` prop, and
//! `CommandDialog::command(|c| ..)` becomes the Command props repeated on
//! `CommandDialog` (`placeholder`, `empty`, `groups`, `on_select`, `max_height`).

use std::rc::Rc;

use rsx::prelude::*;
use rsx::Div;

use super::dialog::{use_modal, DialogHandle, Modal, ModalPlacement};
use super::icon::{Icon, IconName};
use super::input::Input;
use super::theme::theme;

type SelectHandler = Rc<dyn Fn()>;

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

    /// Identifier passed to `Command`'s `on_select` (defaults to the label).
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

    pub fn on_select(mut self, handler: impl Fn() + 'static) -> Self {
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

#[component]
pub fn Command(
    /// Default "Type a command or search...".
    placeholder: Option<Accessor<String>>,
    /// `CommandEmpty` text (default "No results found.").
    empty: Option<Accessor<String>>,
    /// The groups of items (gpuicn's `.group(..)`).
    #[prop(optional)]
    groups: Vec<CommandGroup>,
    /// Called with the item's value whenever any item is selected.
    on_select: Option<Callback<SharedString>>,
    /// Height of the scrollable list (default 300px).
    #[prop(default = 300.0)]
    max_height: f32,
    /// Connected to the search field, to focus it (`node.focus()`).
    input_ref: Option<NodeRef>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let placeholder = placeholder.unwrap_or_else(|| Accessor::value("Type a command or search...".to_string()));
    let empty = empty.unwrap_or_else(|| Accessor::value("No results found.".to_string()));
    let groups = StoredValue::new(groups);
    let query = signal(String::new());
    let input = input_ref.unwrap_or_default();
    let highlighted = signal(0usize);

    // The visible items as (group, item) indices, in display order.
    let visible = memo(move || {
        query.with(|q| {
            groups.with_value(|groups| {
                groups
                    .iter()
                    .enumerate()
                    .flat_map(|(g, group)| {
                        group.items.iter().enumerate().filter(|(_, item)| item.matches(q)).map(move |(i, _)| (g, i))
                    })
                    .collect::<Vec<_>>()
            })
        })
    });
    // Typing starts again from the first item.
    effect(move || {
        query.with(|_| ());
        highlighted.set(0);
    });
    let current = move || {
        let count = visible.with(|v| v.len());
        highlighted.get().min(count.saturating_sub(1))
    };

    let run = move |pos: usize| {
        let Some((g, i)) = visible.with_untracked(|v| v.get(pos).copied()) else { return };
        let item = groups.with_value(|groups| groups[g].items[i].clone());
        if item.disabled {
            return;
        }
        if let Some(f) = on_select {
            f.call(&item.value);
        }
        if let Some(handler) = &item.on_select {
            handler();
        }
    };
    let move_by = move |delta: isize| {
        let count = visible.with_untracked(|v| v.len());
        if count == 0 {
            return;
        }
        let at = highlighted.get_untracked().min(count - 1) as isize;
        highlighted.set((at + delta).rem_euclid(count as isize) as usize);
    };
    // Focus the search field once it has been laid out. A surrounding
    // popover or dialog focuses its own panel when it opens, and rsx-gpui
    // applies focus requests made in the same frame outermost last, so the
    // field's `autofocus` alone loses to it.
    let focused_once = StoredValue::new(false);
    effect(move || {
        if input.bounds().is_some() && !focused_once.get_value() {
            focused_once.set_value(true);
            input.focus();
        }
    });

    // One row per visible item. Rebuilt only when the filter changes; the
    // highlight is a binding so hovering never rebuilds a row (rsx#27).
    let rows = move || {
        let visible = visible.get();
        let mut rows: Vec<Element> = Vec::new();
        let mut pos = 0;
        let mut first_group = true;
        groups.with_value(|groups| {
            for (g, group) in groups.iter().enumerate() {
                let items: Vec<CommandItem> =
                    visible.iter().filter(|(vg, _)| *vg == g).map(|&(_, i)| group.items[i].clone()).collect();
                if items.is_empty() {
                    continue;
                }
                if !std::mem::take(&mut first_group) {
                    // rsx#9: no `-mx-1`.
                    rows.push(<div class="h-px" mx={px(-4.0)} bg={t.get().border} />);
                }
                let mut children: Vec<Element> = Vec::new();
                if let Some(heading) = group.heading.clone() {
                    children.push(
                        <div class="px-2 py-1.5 text-xs leading-[1rem] font-medium" text_color={t.get().muted_foreground}>
                            {heading}
                        </div>,
                    );
                }
                for item in items {
                    let ix = pos;
                    pos += 1;
                    let disabled = item.disabled;
                    let lit = move || current() == ix && !disabled;
                    let colors = move |el: Div| {
                        let theme = t.get();
                        if lit() {
                            el.bg(theme.accent).text_color(theme.accent_foreground)
                        } else {
                            el.bg(Color::TRANSPARENT).text_color(theme.foreground)
                        }
                    };
                    children.push(
                        <div
                            class="flex items-center gap-2 px-2 py-1.5 text-sm leading-[1.25rem] cursor-default"
                            corner_radius={t.get().radius_sm()}
                            opacity={if disabled { 0.5f32 } else { 1.0f32 }}
                            bind={colors}
                            onMouseMove={move |_| {
                                if highlighted.get_untracked() != ix {
                                    highlighted.set(ix);
                                }
                            }}
                            onClick={move |e: &ClickEvent| {
                                // Keep focus in the search field (the root
                                // would take it, rsx#25).
                                e.stop_propagation();
                                input.focus();
                                run(ix);
                            }}
                        >
                            {item.icon.map(|icon| icon.element().text_color(t.get().muted_foreground))}
                            <div class="flex-1 min-w-0">{item.label.clone()}</div>
                            {item.shortcut.clone().map(|s| <div class="ml-auto text-xs leading-[1rem]" text_color={t.get().muted_foreground}>{s}</div>)}
                            {item.checked.map(|checked| IconName::Check.element().class("ml-auto").opacity(if checked { 1.0 } else { 0.0 }))}
                        </div>,
                    );
                }
                rows.push(<div class="flex flex-col p-1" text_color={t.get().foreground}>{children}</div>);
            }
        });
        if rows.is_empty() {
            rows.push(<div class="py-6 text-center text-sm leading-[1.25rem]">{empty}</div>);
        }
        rows
    };

    let attrs = StoredValue::new(attrs);
    // Theme colors as a binding, followed by the caller's overrides so a
    // static `class="rounded-lg"` still wins (rsx#10). The root itself is
    // built once, so the search field keeps its focus and text.
    let style = move |el: Div| {
        let theme = t.get();
        el.corner_radius(theme.radius_md())
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .spread(attrs.with_value(|a| a.clone()))
    };

    // rsx#25: the key handler makes the root focusable, and a click on it
    // (header padding, headings) would take the focus from the search field.
    <div
        class="flex flex-col w-full overflow-hidden"
        bind={style}
        onKeyDown={move |e: &KeyboardEvent| match e.key.as_str() {
            "ArrowUp" => {
                e.stop_propagation();
                e.prevent_default();
                move_by(-1);
            }
            "ArrowDown" => {
                e.stop_propagation();
                e.prevent_default();
                move_by(1);
            }
            "Enter" => {
                e.stop_propagation();
                run(untrack(current));
            }
            _ => {}
        }}
        onClick={move |_| input.focus()}
        {...attrs.with_value(|a| a.clone())}
    >
        <div class="flex h-9 items-center gap-2 px-3 border-b" border_color={t.get().border}>
            <Icon name={IconName::Search} class="opacity-50" />
            <Input
                unstyled
                autofocus
                node_ref={input}
                text={query}
                placeholder={placeholder}
                // Enter doesn't bubble out of a text field.
                onSubmit={move |_: &String| run(untrack(current))}
            />
        </div>
        {/* rsx#22: no scroll position API, so the highlighted row isn't scrolled into view. */}
        <div class="flex flex-col overflow-y-scroll" max_h={px(max_height)}>
            {rows}
        </div>
    </div>
}

/// [`Command`] inside a [`Dialog`](super::dialog::Dialog) — the ⌘K palette.
/// Selecting an item closes it.
#[component]
pub fn CommandDialog(
    /// Control the open state from outside.
    open: Option<Accessor<bool>>,
    on_open_change: Option<Callback<bool>>,
    /// Share state with the caller: `handle={dialog}` with `let dialog = DialogHandle::new();`.
    handle: Option<DialogHandle>,
    // The inner Command's options (gpuicn's `.command(|c| ..)`).
    placeholder: Option<Accessor<String>>,
    empty: Option<Accessor<String>>,
    #[prop(optional)]
    groups: Vec<CommandGroup>,
    on_select: Option<Callback<SharedString>>,
    #[prop(default = 300.0)]
    max_height: f32,
) -> impl IntoElement {
    let handle = use_modal(open, on_open_change, handle);
    let t = theme();
    let groups = StoredValue::new(groups);
    // #26: the Command is drawn in the modal's own portal; nothing inside it
    // opens another Portal.
    let panel = move |handle: DialogHandle| {
        let select = move |value: &SharedString| {
            handle.close();
            if let Some(f) = on_select {
                f.call(value);
            }
        };
        // rsx#7: no `shadow-lg`.
        <div
            class="w-[512px] max-w-full overflow-hidden border"
            corner_radius={t.get().radius_lg()}
            border_color={t.get().border}
        >
            <Command
                placeholder={placeholder}
                empty={empty}
                groups={groups.get_value()}
                max_height={max_height}
                onSelect={select}
            />
        </div>
    };
    <div class="flex">
        <Modal handle={handle} placement={ModalPlacement::Center} panel={panel} />
    </div>
}

//! A set of layered sections of content — known as tab panels — that are
//! displayed one at a time.
//!
//! ```ignore
//! <Tabs default_value="account" class="w-[400px]">
//!     <Tab value="account" label="Account">{account_card}</Tab>
//!     <Tab value="password" label="Password">{password_card}</Tab>
//! </Tabs>
//! ```
//!
//! Keyboard: the arrow keys (and Home / End) move to the previous / next
//! tab and select it, like Radix's automatic activation.

use rsx::prelude::*;
use rsx::{Cursor, Div};

use super::icon::{Icon, IconName};
use super::theme::{theme, ColorExt};
use super::utils::{is_focus_visible, keyboard_focus, on_tab_key, pointer_pressed, ring, Orientation, Siblings};

#[derive(Clone, Copy)]
struct TabInfo {
    id: u64,
    value: StoredValue<SharedString>,
    label: Accessor<String>,
    icon: Option<IconName>,
    disabled: Option<Accessor<bool>>,
    content: Option<Children>,
    trigger: NodeRef,
}

impl TabInfo {
    fn disabled(&self) -> bool {
        self.disabled.map(|d| d.get()).unwrap_or(false)
    }
}

#[derive(Clone, Copy)]
struct TabsContext {
    tabs: StoredValue<Vec<TabInfo>>,
    order: Siblings,
}

impl TabsContext {
    fn info(&self, id: u64) -> Option<TabInfo> {
        self.tabs.with_value(|t| t.iter().find(|t| t.id == id).copied())
    }

    /// The registered tabs, in order (tracked).
    fn ordered(&self) -> Vec<TabInfo> {
        let len = self.order.len();
        let mut tabs = self.tabs.get_value();
        tabs.sort_by_key(|t| self.order.index(t.id).unwrap_or(len));
        tabs
    }
}

#[component]
pub fn Tabs(
    /// The selected tab (controlled).
    value: Option<Accessor<SharedString>>,
    /// The tab selected at first (uncontrolled); defaults to the first tab.
    default_value: Option<SharedString>,
    #[prop(optional)] orientation: Orientation,
    /// Tab list on the left, content on the right.
    vertical: bool,
    /// Stretch the tab list to the full width.
    full_width: bool,
    on_change: Option<Callback<SharedString>>,
    /// [`Tab`]s.
    children: Option<ChildrenFn>,
    attrs: Attributes,
) -> impl IntoElement {
    let vertical = vertical || orientation == Orientation::Vertical;
    let context = TabsContext { tabs: StoredValue::new(Vec::new()), order: Siblings::new() };
    let state = signal(default_value);
    let t = theme();
    let active = memo(move || {
        value.map(|v| v.get()).or_else(|| state.get()).or_else(|| context.ordered().first().map(|t| t.value.get_value()))
    });
    let select = move |tab: TabInfo| {
        let value = tab.value.get_value();
        state.set(Some(value.clone()));
        if let Some(f) = on_change {
            f.call(&value);
        }
    };
    // Arrow keys: the next enabled tab in `step` direction (or the first / last), wrapping.
    let step = move |from: u64, key: &str| -> Option<TabInfo> {
        let tabs: Vec<TabInfo> = untrack(|| context.ordered().into_iter().filter(|t| !t.disabled() || t.id == from).collect());
        let ix = tabs.iter().position(|t| t.id == from)?;
        let n = tabs.len();
        let (prev, next) = if vertical { ("ArrowUp", "ArrowDown") } else { ("ArrowLeft", "ArrowRight") };
        let target = match key {
            k if k == next => (ix + 1) % n,
            k if k == prev => (ix + n - 1) % n,
            "Home" => 0,
            "End" => n - 1,
            _ => return None,
        };
        Some(tabs[target])
    };

    let trigger = move |id: u64| {
        let Some(tab) = context.info(id) else { return Element::Empty };
        let selected = move || active.with(|a| a.as_ref() == Some(&tab.value.get_value()));
        let node = tab.trigger;
        // One element whose styles are bindings: re-creating it on selection or
        // focus changes would lose GPUI's element state (the click in progress, focus).
        <div
            ref={node}
            class="relative flex items-center justify-center gap-1.5 h-full px-2 py-1 border text-sm leading-[1.25rem] font-medium"
            class:flex-1={!vertical}
            bind={move |el: Div| {
                let theme = t.get();
                let is_selected = selected();
                let disabled = tab.disabled();
                let fg = if theme.is_dark() && !is_selected { theme.muted_foreground } else { theme.foreground };
                let bg = match (is_selected, theme.is_dark()) {
                    (false, _) => Color::TRANSPARENT,
                    (true, true) => theme.input.opacity(0.3),
                    (true, false) => theme.background,
                };
                let border = if is_selected && theme.is_dark() { theme.input } else { Color::TRANSPARENT };
                // rsx#7: no `shadow-sm` on the selected tab. rsx#9: no `whitespace-nowrap`.
                let el = el
                    .corner_radius(theme.radius_md())
                    .border_color(border)
                    .bg(bg)
                    .text_color(fg)
                    .opacity(if disabled { 0.5 } else { 1.0 });
                if disabled { el } else { el.cursor(Cursor::Pointer) }
            }}
            onMouseDown={move |_| pointer_pressed()}
            onClick={move |_| {
                if !tab.disabled() && !untrack(selected) {
                    select(tab);
                }
            }}
            onKeyDown={move |e: &KeyboardEvent| {
                on_tab_key(e);
                if let Some(target) = step(tab.id, &e.key) {
                    e.prevent_default();
                    keyboard_focus();
                    target.trigger.focus();
                    if !untrack(|| active.get()).is_some_and(|a| a == target.value.get_value()) {
                        select(target);
                    }
                }
            }}
        >
            {tab.icon.map(|i| <Icon name={i} />)}
            {tab.label}
            {move || is_focus_visible(node).then(|| ring(t.get().ring.opacity(0.5), 3.0, t.get().radius - 2.0, 1.0))}
        </div>
    };

    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div class="flex gap-2" class:flex-col={!vertical} class:flex-row={vertical} {...attrs}>
        <div class="flex" class:flex-none={!vertical && !full_width}>
            <div
                class="flex items-center p-[3px]"
                class:flex-col={vertical}
                class:items-stretch={vertical}
                class:h-9={!vertical}
                class:w-full={!vertical && full_width}
                corner_radius={t.get().radius_lg()}
                bg={t.get().muted}
                text_color={t.get().muted_foreground}
            >
                <For each={move || context.ordered().iter().map(|t| t.id).collect::<Vec<u64>>()}>
                    {move |id| trigger(id)}
                </For>
            </div>
        </div>
        {move || {
            let active = active.get()?;
            let tab = context.ordered().into_iter().find(|t| t.value.get_value() == active)?;
            tab.content.map(|content| <div class="flex-1 min-w-0">{content}</div>)
        }}
        <Provider value={context}>{children}</Provider>
    </div>
}

/// One tab of [`Tabs`]: its trigger, and the content shown while it's
/// selected (the children; omit them to use the tabs as a segmented control).
#[component]
pub fn Tab(
    value: SharedString,
    label: Accessor<String>,
    icon: Option<IconName>,
    disabled: Option<Accessor<bool>>,
    /// `TabsContent` for this tab (or pass it as children).
    content: Option<Children>,
    children: Option<Children>,
) -> impl IntoElement {
    let context = expect_context::<TabsContext>();
    // One batch, so the tab list never sees an id without its tab.
    let id = batch(|| {
        let id = context.order.register();
        let info = TabInfo {
            id,
            value: StoredValue::new(value),
            label,
            icon,
            disabled,
            content: content.or(children),
            trigger: NodeRef::new(),
        };
        context.tabs.update_value(|t| t.push(info));
        id
    });
    on_cleanup(move || {
        if context.tabs.is_alive() {
            context.tabs.update_value(|t| t.retain(|t| t.id != id));
        }
    });
}

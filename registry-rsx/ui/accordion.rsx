//! A vertically stacked set of interactive headings that each reveal a
//! section of content.
//!
//! ```ignore
//! <Accordion default_value={vec!["item-1".into()]} class="w-[480px]">
//!     <AccordionItem value="item-1" title="Product Information">
//!         {p("Our flagship product…")}
//!     </AccordionItem>
//!     <AccordionItem value="item-2" title="Shipping Details">
//!         {p("We offer worldwide shipping…")}
//!     </AccordionItem>
//! </Accordion>
//! ```
//!
//! Keyboard: Enter / Space toggle the focused item, arrow keys (and Home /
//! End) move between headings.

use rsx::prelude::*;
use rsx::Div;

use super::icon::{Icon, IconName};
use super::theme::{theme, ColorExt};
use super::utils::{is_focus_visible, keyboard_focus, on_tab_key, pointer_pressed, ring, Siblings};

#[derive(Clone, Copy)]
struct AccordionContext {
    state: Signal<Vec<SharedString>>,
    value: Option<Accessor<Vec<SharedString>>>,
    on_change: Option<Callback<Vec<SharedString>>>,
    multiple: bool,
    collapsible: bool,
    items: Siblings,
    triggers: StoredValue<Vec<(u64, NodeRef)>>,
}

impl AccordionContext {
    /// The open items (tracked).
    fn open_values(&self) -> Vec<SharedString> {
        match self.value {
            Some(value) => value.get(),
            None => self.state.get(),
        }
    }

    fn toggle(&self, value: &SharedString) {
        let open_values = untrack(|| self.open_values());
        let open = open_values.contains(value);
        let next: Vec<SharedString> = match (open, self.multiple) {
            (true, true) => open_values.iter().filter(|v| *v != value).cloned().collect(),
            (true, false) if self.collapsible => Vec::new(),
            (true, false) => open_values.clone(),
            (false, true) => {
                let mut next = open_values.clone();
                next.push(value.clone());
                next
            }
            (false, false) => vec![value.clone()],
        };
        self.state.set(next.clone());
        if let Some(f) = self.on_change {
            f.call(&next);
        }
    }

    /// Moves focus to the heading `step` places away (wrapping), or to the
    /// first / last one.
    fn focus_sibling(&self, id: u64, key: &str) {
        let triggers = self.triggers.get_value();
        let Some(ix) = triggers.iter().position(|(i, _)| *i == id) else { return };
        let n = triggers.len();
        let target = match key {
            "ArrowDown" => (ix + 1) % n,
            "ArrowUp" => (ix + n - 1) % n,
            "Home" => 0,
            "End" => n - 1,
            _ => return,
        };
        keyboard_focus();
        triggers[target].1.focus();
    }
}

#[component]
pub fn Accordion(
    /// Allow several items open at once (`type="multiple"`).
    multiple: bool,
    /// In single mode, whether the open item can be closed again.
    #[prop(default = true)]
    collapsible: bool,
    /// The open items (controlled).
    value: Option<Accessor<Vec<SharedString>>>,
    /// The items open at first (uncontrolled).
    #[prop(optional)]
    default_value: Vec<SharedString>,
    on_change: Option<Callback<Vec<SharedString>>>,
    /// [`AccordionItem`]s.
    children: Option<ChildrenFn>,
    attrs: Attributes,
) -> impl IntoElement {
    let context = AccordionContext {
        state: signal(default_value),
        value,
        on_change,
        multiple,
        collapsible,
        items: Siblings::new(),
        triggers: StoredValue::new(Vec::new()),
    };
    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div class="flex flex-col w-full" {...attrs}>
        <Provider value={context}>{children}</Provider>
    </div>
}

#[component]
pub fn AccordionItem(
    value: SharedString,
    title: Accessor<String>,
    disabled: Option<Accessor<bool>>,
    /// The section revealed when open (or pass it as children).
    content: Option<Children>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let context = expect_context::<AccordionContext>();
    let id = context.items.register();
    let trigger = NodeRef::new();
    context.triggers.update_value(|t| t.push((id, trigger)));
    on_cleanup(move || {
        if context.triggers.is_alive() {
            context.triggers.update_value(|t| t.retain(|(i, _)| *i != id));
        }
    });
    let t = theme();
    let hovered = signal(false);
    let value = StoredValue::new(value);
    let disabled = move || disabled.map(|d| d.get()).unwrap_or(false);
    let open = move || value.with_value(|v| context.open_values().contains(v));
    let toggle = move || {
        if !disabled() {
            value.with_value(|v| context.toggle(v));
        }
    };

    <div
        class="flex flex-col"
        class:border-b={move || !context.items.is_last(id)}
        border_color={t.get().border}
        {...attrs}
    >
        <div
            ref={trigger}
            class="relative flex items-start justify-between gap-4 py-4 text-sm leading-[1.25rem] font-medium"
            class:cursor-pointer={move || !disabled()}
            class:underline={!disabled() && hovered.get()}
            bind={move |el: Div| el.opacity(if disabled() { 0.5 } else { 1.0 })}
            onHover={move |h: &bool| hovered.set(*h)}
            onMouseDown={move |_| pointer_pressed()}
            onClick={move |_| toggle()}
            onKeyDown={move |e: &KeyboardEvent| {
                on_tab_key(e);
                match e.key.as_str() {
                    "Enter" | " " | "Space" => {
                        e.prevent_default();
                        toggle();
                    }
                    "ArrowDown" | "ArrowUp" | "Home" | "End" => {
                        e.prevent_default();
                        context.focus_sibling(id, &e.key);
                    }
                    _ => {}
                }
            }}
        >
            <div class="flex-1 min-w-0">{title}</div>
            <div class="flex-none mt-0.5" text_color={t.get().muted_foreground}>
                <Icon
                    name={IconName::ChevronDown}
                    rotate={Some(Accessor::derive(move || if open() { 180.0f32 } else { 0.0 }))}
                />
            </div>
            {move || is_focus_visible(trigger).then(|| ring(t.get().ring.opacity(0.5), 3.0, t.get().radius - 2.0, 0.0))}
        </div>
        {move || open().then(|| <div class="pb-4 text-sm leading-[1.25rem]">
            {content}
            {children}
        </div>)}
    </div>
}

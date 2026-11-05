//! A collection of links for navigating websites — here, app sections.
//! Items with content open a panel on hover.
//!
//! ```ignore
//! <NavigationMenu items={vec![
//!     NavigationMenuItem::new("Getting started").content(|| <div class="w-[400px] flex flex-col">
//!         <NavigationMenuLink title="Introduction" description="Re-usable components…" />
//!         <NavigationMenuLink title="Installation" description="How to install…" />
//!     </div>),
//!     NavigationMenuItem::new("Docs").on_click(|_| {}),
//! ]} />
//! ```
//!
//! gpuicn's `NavigationMenu::new(id).item(..).item(..)` becomes the `items` prop,
//! and `NavigationMenuLink::new(title)` the `title` prop.

use std::rc::Rc;

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::popover::{Popover, PopoverTrigger};
use super::theme::{theme, ColorExt};
use super::utils::Align;

type ClickHandler = Rc<dyn Fn(&ClickEvent)>;

pub struct NavigationMenuItem {
    label: SharedString,
    content: Option<Rc<dyn Fn() -> Element>>,
    on_click: Option<ClickHandler>,
    active: bool,
}

impl NavigationMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), content: None, on_click: None, active: false }
    }

    /// Panel shown on hover (`NavigationMenuContent`).
    pub fn content<E: IntoElement>(mut self, content: impl Fn() -> E + 'static) -> Self {
        self.content = Some(Rc::new(move || content().into_element()));
        self
    }

    /// Make this a plain link (`NavigationMenuLink`) instead of a trigger.
    pub fn on_click(mut self, handler: impl Fn(&ClickEvent) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[component]
pub fn NavigationMenu(
    /// The items, left to right (gpuicn's `.item(..)`).
    #[prop(optional)]
    items: Vec<NavigationMenuItem>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    // Which item's panel is open, to flip its chevron and keep it highlighted.
    let open_ix = signal(None::<usize>);

    let items = items.into_iter().enumerate().map(|(ix, item)| {
        let hovered = signal(false);
        let has_content = item.content.is_some();
        let on_click = StoredValue::new(item.on_click);
        let (label, active) = (item.label, item.active);
        let trigger = move || {
            let theme = t.get();
            let is_open = open_ix.get() == Some(ix);
            let (accent, accent_fg) = (theme.accent, theme.accent_foreground);
            let bg = if hovered.get() {
                accent
            } else if active || is_open {
                accent.opacity(0.5)
            } else {
                Color::TRANSPARENT
            };
            <div
                class="flex items-center h-9 px-4 py-2 text-sm leading-[1.25rem] font-medium cursor-pointer"
                corner_radius={theme.radius_md()}
                bg={bg}
                text_color={if hovered.get() { accent_fg } else { theme.foreground }}
                onHover={move |h: &bool| hovered.set(*h)}
                onClick={move |e: &ClickEvent| on_click.with_value(|f| if let Some(f) = f { f(e) })}
            >
                {label.clone()}
                {has_content.then(|| <Icon
                    name={if is_open { IconName::ChevronUp } else { IconName::ChevronDown }}
                    class="size-3.5 ml-1"
                />)}
            </div>
        };
        match item.content {
            Some(content) => {
                let content = StoredValue::new(content);
                <Popover
                    mode={PopoverTrigger::Hover { open_delay_ms: 150, close_delay_ms: 250 }}
                    align={Align::Start}
                    side_offset={6.0f32}
                    on_open_change={move |open: &bool| {
                        if *open {
                            open_ix.set(Some(ix));
                        } else if open_ix.get_untracked() == Some(ix) {
                            open_ix.set(None);
                        }
                    }}
                    class="w-auto p-2"
                    trigger={trigger}
                    content={move |_| content.with_value(|c| c())}
                />
            }
            None => trigger.into_element(),
        }
    });
    let items: Vec<Element> = items.collect();

    <div class="flex items-center gap-1" {...attrs}>{items}</div>
}

/// A title + description link block for navigation menu panels.
#[component]
pub fn NavigationMenuLink(
    title: Accessor<String>,
    description: Option<Accessor<String>>,
    on_click: Option<Callback<ClickEvent>>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let hovered = signal(false);
    let attrs = StoredValue::new(attrs);
    move || {
        let theme = t.get();
        let lit = hovered.get();
        <div
            class="flex flex-col gap-1 p-2 text-sm leading-[1.25rem] cursor-pointer"
            corner_radius={theme.radius_sm()}
            bg={if lit { theme.accent } else { Color::TRANSPARENT }}
            text_color={if lit { theme.accent_foreground } else { theme.popover_foreground }}
            onHover={move |h: &bool| hovered.set(*h)}
            onClick={move |e: &ClickEvent| if let Some(f) = on_click { f.call(e) }}
            {...attrs.with_value(|a| a.clone())}
        >
            <div class="font-medium leading-none">{title}</div>
            {/* rsx#9: no `line-clamp-2` / `text-ellipsis`; long descriptions wrap. */}
            {description.map(|d| <div class="leading-[19.25px]" text_color={theme.muted_foreground}>{d}</div>)}
        </div>
    }
}

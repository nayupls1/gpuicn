//! A collection of links for navigating websites — here, app sections.
//! Items with content open a panel on hover.
//!
//! ```ignore
//! NavigationMenu::new("nav")
//!     .item(NavigationMenuItem::new("Getting started").content(|_, _| {
//!         div().w(px(400.)).flex().flex_col()
//!             .child(NavigationMenuLink::new("Introduction").description("Re-usable components…"))
//!             .child(NavigationMenuLink::new("Installation").description("How to install…"))
//!     }))
//!     .item(NavigationMenuItem::new("Docs").on_click(|_, _, _| {}))
//! ```

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, relative,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::popover::{Popover, PopoverTrigger};
use super::theme::ActiveTheme;
use super::utils::{Align, StyledExt};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

pub struct NavigationMenuItem {
    label: SharedString,
    content: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>>,
    on_click: Option<ClickHandler>,
    active: bool,
}

impl NavigationMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), content: None, on_click: None, active: false }
    }

    /// Panel shown on hover (`NavigationMenuContent`).
    pub fn content<E: IntoElement>(mut self, content: impl Fn(&mut Window, &mut App) -> E + 'static) -> Self {
        self.content = Some(Rc::new(move |window, cx| content(window, cx).into_any_element()));
        self
    }

    /// Make this a plain link (`NavigationMenuLink`) instead of a trigger.
    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[derive(IntoElement)]
pub struct NavigationMenu {
    id: ElementId,
    items: Vec<NavigationMenuItem>,
    style: StyleRefinement,
}

crate::impl_styled!(NavigationMenu);

impl NavigationMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into(), items: Vec::new(), style: StyleRefinement::default() }
    }

    pub fn item(mut self, item: NavigationMenuItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for NavigationMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (accent, accent_fg) = (theme.accent, theme.accent_foreground);
        let radius = theme.radius_md();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_1()
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map(|(ix, item)| {
                let trigger = div()
                    .id(("nav-trigger", ix))
                    .flex()
                    .items_center()
                    .h_9()
                    .px_4()
                    .py_2()
                    .rounded(radius)
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .when(item.active, |this| this.bg(accent.opacity(0.5)))
                    .hover(move |s| s.bg(accent).text_color(accent_fg))
                    .child(item.label)
                    .when(item.content.is_some(), |this| {
                        this.child(Icon::new(IconName::ChevronDown).size_3().ml_1())
                    })
                    .when_some(item.on_click, |this, handler| {
                        this.on_click(move |event, window, cx| handler(event, window, cx))
                    });

                match item.content {
                    Some(content) => Popover::new(("nav-content", ix))
                        .mode(PopoverTrigger::Hover { open_delay_ms: 150, close_delay_ms: 250 })
                        .align(Align::Start)
                        .side_offset(gpui::px(6.))
                        .w_auto()
                        .p_2()
                        .trigger(trigger)
                        .content(move |window, cx| content(window, cx))
                        .into_any_element(),
                    None => trigger.into_any_element(),
                }
            }))
    }
}

/// A title + description link block for navigation menu panels.
#[derive(IntoElement)]
pub struct NavigationMenuLink {
    id: ElementId,
    title: SharedString,
    description: Option<SharedString>,
    on_click: Option<ClickHandler>,
    style: StyleRefinement,
}

crate::impl_styled!(NavigationMenuLink);

impl NavigationMenuLink {
    pub fn new(title: impl Into<SharedString>) -> Self {
        let title = title.into();
        Self { id: ElementId::Name(title.clone()), title, description: None, on_click: None, style: StyleRefinement::default() }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for NavigationMenuLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (accent, accent_fg) = (theme.accent, theme.accent_foreground);
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_1()
            .p_2()
            .rounded(theme.radius_sm())
            .text_sm()
            .cursor_pointer()
            .hover(move |s| s.bg(accent).text_color(accent_fg))
            .refine_style(&self.style)
            .child(div().font_weight(FontWeight::MEDIUM).line_height(relative(1.)).child(self.title))
            .children(self.description.map(|description| {
                div()
                    .text_color(theme.muted_foreground)
                    .line_clamp(2)
                    .line_height(relative(1.375))
                    .child(description)
            }))
            .when_some(self.on_click, |this, handler| {
                this.on_click(move |event, window, cx| handler(event, window, cx))
            })
    }
}

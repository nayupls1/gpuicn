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
    AnyElement, App, ClickEvent, ElementId, Entity, FontWeight, IntoElement, RenderOnce,
    SharedString, StyleRefinement, Task, Window, div, prelude::*, relative,
};
use std::rc::Rc;
use std::time::Duration;

use super::icon::{Icon, IconName};
use super::popover::{Popover, PopoverTrigger};
use super::theme::ActiveTheme;
use super::utils::{Align, StyledExt, on_mouse_leave_window};

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

/// Which panel is open and what the pointer is over. Shared by all items so
/// moving between triggers switches panels instantly, like Radix.
#[derive(Default)]
struct NavState {
    open: Option<usize>,
    hovered: Option<usize>,
    task: Option<Task<()>>,
}

const OPEN_DELAY: Duration = Duration::from_millis(150);
const CLOSE_DELAY: Duration = Duration::from_millis(250);

fn set_hovered(state: &Entity<NavState>, ix: usize, hovered: bool, window: &mut Window, cx: &mut App) {
    let target = state.update(cx, |s, _| {
        if hovered {
            s.hovered = Some(ix);
        } else if s.hovered == Some(ix) {
            s.hovered = None;
        }
        s.hovered
    });
    let open = state.read(cx).open;
    if target.is_some() && open.is_some() {
        // Already showing a panel: switch (or stay) right away.
        state.update(cx, |s, cx| {
            s.open = target;
            s.task = None;
            cx.notify();
        });
        return;
    }
    if target == open {
        state.update(cx, |s, _| s.task = None);
        return;
    }
    let delay = if target.is_some() { OPEN_DELAY } else { CLOSE_DELAY };
    let timer_state = state.clone();
    let task = window.spawn(cx, async move |cx| {
        cx.background_executor().timer(delay).await;
        cx.update(|_, cx| {
            timer_state.update(cx, |s, cx| {
                if s.open != s.hovered {
                    s.open = s.hovered;
                    cx.notify();
                }
            })
        })
        .ok();
    });
    state.update(cx, |s, _| s.task = Some(task));
}

impl RenderOnce for NavigationMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| NavState::default());
        let open = state.read(cx).open;
        let theme = cx.theme();
        let (accent, accent_fg) = (theme.accent, theme.accent_foreground);
        let radius = theme.radius_md();
        let leave_state = state.clone();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_1()
            .refine_style(&self.style)
            .child(on_mouse_leave_window(move |_, cx| {
                leave_state.update(cx, |s, cx| {
                    *s = NavState::default();
                    cx.notify();
                })
            }))
            .children(self.items.into_iter().enumerate().map(|(ix, item)| {
                let is_open = open == Some(ix);
                let has_content = item.content.is_some();
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
                    .when(is_open, |this| this.bg(accent.opacity(0.5)))
                    .hover(move |s| s.bg(accent).text_color(accent_fg))
                    .child(item.label)
                    .when(has_content, |this| {
                        this.child(Icon::new(IconName::ChevronDown).size_3().ml_1().when(is_open, |icon| icon.rotate(std::f32::consts::PI)))
                    })
                    .on_hover({
                        let state = state.clone();
                        move |hovered, window, cx| set_hovered(&state, ix, *hovered, window, cx)
                    })
                    .when(has_content, |this| {
                        let state = state.clone();
                        this.on_click(move |_, _, cx| {
                            state.update(cx, |s, cx| {
                                s.open = if s.open == Some(ix) { None } else { Some(ix) };
                                s.task = None;
                                cx.notify();
                            })
                        })
                    })
                    .when_some(item.on_click, |this, handler| {
                        this.on_click(move |event, window, cx| handler(event, window, cx))
                    });

                match item.content {
                    Some(content) => {
                        let content_state = state.clone();
                        let close_state = state.clone();
                        Popover::new(("nav-content", ix))
                            .mode(PopoverTrigger::Manual)
                            .modal(false)
                            .open(is_open)
                            .on_open_change(move |open, _, cx| {
                                if !open {
                                    close_state.update(cx, |s, cx| {
                                        if s.open == Some(ix) {
                                            s.open = None;
                                            s.task = None;
                                            cx.notify();
                                        }
                                    })
                                }
                            })
                            .align(Align::Start)
                            .side_offset(gpui::px(6.))
                            .w_auto()
                            .p_2()
                            .trigger(trigger)
                            .content(move |window, cx| {
                                let state = content_state.clone();
                                div()
                                    .id(("nav-panel", ix))
                                    .on_hover(move |hovered, window, cx| set_hovered(&state, ix, *hovered, window, cx))
                                    .child(content(window, cx))
                            })
                            .into_any_element()
                    }
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

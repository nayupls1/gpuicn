//! A set of layered sections of content — known as tab panels — that are
//! displayed one at a time.
//!
//! ```ignore
//! Tabs::new("settings")
//!     .default_value("account")
//!     .tab(Tab::new("account", "Account").content(account_card))
//!     .tab(Tab::new("password", "Password").content(password_card))
//! ```

use gpui::{
    AnyElement, App, ElementId, FontWeight, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, px,
};
use std::rc::Rc;

use super::icon::Icon;
use super::theme::ActiveTheme;
use super::utils::{Orientation, StyledExt};

pub struct Tab {
    value: SharedString,
    label: SharedString,
    icon: Option<Icon>,
    disabled: bool,
    content: Option<AnyElement>,
}

impl Tab {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self { value: value.into(), label: label.into(), icon: None, disabled: false, content: None }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// `TabsContent` for this tab; omit it to use the tabs as a segmented control.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    tabs: Vec<Tab>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    orientation: Orientation,
    full_width: bool,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Tabs);

impl Tabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            value: None,
            default_value: None,
            orientation: Orientation::Horizontal,
            full_width: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn tab(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Tab list on the left, content on the right.
    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    /// Stretch the tab list to the full width.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Tabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self
            .default_value
            .clone()
            .or_else(|| self.tabs.first().map(|t| t.value.clone()))
            .unwrap_or_default();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let active = self.value.clone().unwrap_or_else(|| state.read(cx).clone());
        let theme = cx.theme();
        let vertical = self.orientation == Orientation::Vertical;

        let mut contents = Vec::new();
        let mut triggers = Vec::new();
        for (ix, tab) in self.tabs.into_iter().enumerate() {
            let selected = tab.value == active;
            if selected {
                contents.extend(tab.content);
            }
            let state = state.clone();
            let on_change = self.on_change.clone();
            let value = tab.value.clone();
            let disabled = tab.disabled;
            triggers.push(
                div()
                    .id(ix)
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_1p5()
                    .when(!vertical, |this| this.flex_1())
                    .h_full()
                    .px_2()
                    .py_1()
                    .rounded(theme.radius_md())
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .whitespace_nowrap()
                    .text_color(if theme.is_dark() && !selected { theme.muted_foreground } else { theme.foreground })
                    .when(selected, |this| {
                        this.bg(if theme.is_dark() { theme.input.opacity(0.3) } else { theme.background })
                            .when(theme.is_dark(), |this| this.border_color(theme.input))
                            .when(theme.shadows, |this| this.shadow_sm())
                    })
                    .when(disabled, |this| this.opacity(0.5))
                    .when(!disabled, |this| this.cursor_pointer())
                    .children(tab.icon)
                    .child(tab.label)
                    .when(!disabled && !selected, |this| {
                        this.on_click(move |_, window, cx| {
                            state.update(cx, |s, cx| {
                                *s = value.clone();
                                cx.notify();
                            });
                            if let Some(handler) = &on_change {
                                handler(&value, window, cx);
                            }
                        })
                    }),
            );
        }

        let list = div()
            .flex()
            .items_center()
            .p(px(3.))
            .rounded(theme.radius_lg())
            .bg(theme.muted)
            .text_color(theme.muted_foreground)
            .map(|this| {
                if vertical {
                    let mut this = this.flex_col();
                    this.style().align_items = Some(gpui::AlignItems::Stretch);
                    this
                } else {
                    this.h_9().when(self.full_width, |this| this.w_full())
                }
            })
            .children(triggers);

        div()
            .id(self.id)
            .flex()
            .gap_2()
            .map(|this| if vertical { this.flex_row() } else { this.flex_col() })
            .refine_style(&self.style)
            .child(div().flex().when(!vertical && !self.full_width, |this| this.flex_none()).child(list))
            .children(contents.into_iter().map(|content| div().flex_1().child(content)))
    }
}

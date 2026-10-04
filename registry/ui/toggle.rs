//! A two-state button that can be either on or off.
//!
//! ```ignore
//! Toggle::new("bold").icon(IconName::Bold)
//! Toggle::new("italic").outline().icon(IconName::Italic).label("Italic")
//! ```

use gpui::{
    AnyElement, App, ElementId, FontWeight, Hsla, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Window, div, prelude::*,
};
use std::rc::Rc;

use super::icon::Icon;
use super::theme::{ActiveTheme, Theme};
use super::utils::StyledExt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

/// Shared look of `Toggle` and `ToggleGroup` items.
pub(super) fn toggle_base(
    id: ElementId,
    pressed: bool,
    disabled: bool,
    variant: ToggleVariant,
    size: ToggleSize,
    theme: &Theme,
) -> gpui::Stateful<gpui::Div> {
    let (hover_bg, hover_fg): (Hsla, Hsla) = match variant {
        ToggleVariant::Default => (theme.muted, theme.muted_foreground),
        ToggleVariant::Outline => (theme.accent, theme.accent_foreground),
    };
    div()
        .id(id)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .gap_2()
        .rounded(theme.radius_md())
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .whitespace_nowrap()
        .map(|this| match size {
            ToggleSize::Default => this.h_9().px_2().min_w_9(),
            ToggleSize::Sm => this.h_8().px_1p5().min_w_8(),
            ToggleSize::Lg => this.h_10().px_2p5().min_w_10(),
        })
        .when(variant == ToggleVariant::Outline, |this| {
            this.border_1().border_color(theme.input).when(theme.shadows, |this| this.shadow_xs())
        })
        .when(pressed, |this| this.bg(theme.accent).text_color(theme.accent_foreground))
        .when(disabled, |this| this.opacity(0.5))
        .when(!disabled && !pressed, |this| {
            this.cursor_pointer().hover(move |s| s.bg(hover_bg).text_color(hover_fg))
        })
        .when(!disabled && pressed, |this| this.cursor_pointer())
}

#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    pressed: Option<bool>,
    default_pressed: bool,
    disabled: bool,
    variant: ToggleVariant,
    size: ToggleSize,
    icon: Option<Icon>,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    on_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Toggle);

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pressed: None,
            default_pressed: false,
            disabled: false,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            icon: None,
            label: None,
            children: Vec::new(),
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    pub fn default_pressed(mut self, pressed: bool) -> Self {
        self.default_pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outline(self) -> Self {
        self.variant(ToggleVariant::Outline)
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Toggle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_pressed;
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let pressed = self.pressed.unwrap_or(*state.read(cx));
        let disabled = self.disabled;
        let on_change = self.on_change;

        toggle_base(self.id, pressed, disabled, self.variant, self.size, cx.theme())
            .refine_style(&self.style)
            .children(self.icon)
            .children(self.label)
            .children(self.children)
            .when(!disabled, |this| {
                this.on_click(move |_, window, cx| {
                    let next = !pressed;
                    state.update(cx, |s, cx| {
                        *s = next;
                        cx.notify();
                    });
                    if let Some(handler) = &on_change {
                        handler(&next, window, cx);
                    }
                })
            })
    }
}

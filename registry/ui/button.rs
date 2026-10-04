//! Displays a button or a component that looks like a button.
//!
//! ```ignore
//! Button::new("save").label("Save changes")
//! Button::new("delete").destructive().icon(IconName::Trash).label("Delete")
//! Button::new("more").ghost().size(ButtonSize::Icon).icon(IconName::Ellipsis)
//! ```

use gpui::{
    AnyElement, App, ClickEvent, ElementId, Hsla, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, prelude::*, px,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::spinner::Spinner;
use super::theme::{ActiveTheme, Theme};
use super::utils::StyledExt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn is_icon(self) -> bool {
        matches!(self, ButtonSize::Icon | ButtonSize::IconSm | ButtonSize::IconLg)
    }
}

/// Colors for a button variant: `(background, hover background, text, border)`.
pub fn variant_colors(variant: ButtonVariant, theme: &Theme) -> (Hsla, Hsla, Hsla, Option<Hsla>) {
    let clear = gpui::transparent_black();
    match variant {
        ButtonVariant::Default => (
            theme.primary,
            theme.primary.opacity(0.9),
            theme.primary_foreground,
            None,
        ),
        ButtonVariant::Destructive => {
            let bg = if theme.is_dark() {
                theme.destructive.opacity(0.6)
            } else {
                theme.destructive
            };
            (bg, bg.opacity(0.9), gpui::white(), None)
        }
        ButtonVariant::Outline => {
            if theme.is_dark() {
                (
                    theme.input.opacity(0.3),
                    theme.input.opacity(0.5),
                    theme.foreground,
                    Some(theme.input),
                )
            } else {
                (
                    theme.background,
                    theme.accent,
                    theme.foreground,
                    Some(theme.border),
                )
            }
        }
        ButtonVariant::Secondary => (
            theme.secondary,
            theme.secondary.opacity(0.8),
            theme.secondary_foreground,
            None,
        ),
        ButtonVariant::Ghost => {
            let hover = if theme.is_dark() {
                theme.accent.opacity(0.5)
            } else {
                theme.accent
            };
            (clear, hover, theme.foreground, None)
        }
        ButtonVariant::Link => (clear, clear, theme.primary, None),
    }
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: Option<SharedString>,
    icon: Option<Icon>,
    icon_end: Option<Icon>,
    children: Vec<AnyElement>,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
    selected: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    tooltip: Option<SharedString>,
    style: StyleRefinement,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            icon: None,
            icon_end: None,
            children: Vec::new(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
            selected: false,
            on_click: None,
            tooltip: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Icon shown before the label.
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Icon shown after the label.
    pub fn icon_end(mut self, icon: impl Into<Icon>) -> Self {
        self.icon_end = Some(icon.into());
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn destructive(self) -> Self {
        self.variant(ButtonVariant::Destructive)
    }

    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }

    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }

    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }

    pub fn link(self) -> Self {
        self.variant(ButtonVariant::Link)
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show a spinner in place of the icon and ignore clicks.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Render in the hovered (accent) state, e.g. for an open menu trigger.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (bg, hover_bg, fg, border) = variant_colors(self.variant, theme);
        let accent_fg = match self.variant {
            ButtonVariant::Outline | ButtonVariant::Ghost => theme.accent_foreground,
            _ => fg,
        };
        let has_icon = self.icon.is_some() || self.loading;
        let interactive = !self.disabled && !self.loading;
        let radius = theme.radius_md();
        let is_link = self.variant == ButtonVariant::Link;
        let shadow = theme.shadows && self.variant == ButtonVariant::Outline;

        let base = div()
            .id(self.id)
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap_2()
            .whitespace_nowrap()
            .rounded(radius)
            .text_sm()
            .font_weight(gpui::FontWeight::MEDIUM)
            .bg(if self.selected { hover_bg } else { bg })
            .text_color(if self.selected { accent_fg } else { fg })
            .when_some(border, |this, color| this.border_1().border_color(color))
            .when(shadow, |this| this.shadow_xs())
            .map(|this| match self.size {
                ButtonSize::Default => this.h_9().px(if has_icon { px(12.) } else { px(16.) }),
                ButtonSize::Sm => this.h_8().gap_1p5().px(if has_icon { px(10.) } else { px(12.) }),
                ButtonSize::Lg => this.h_10().px(if has_icon { px(16.) } else { px(24.) }),
                ButtonSize::Icon => this.size_9(),
                ButtonSize::IconSm => this.size_8(),
                ButtonSize::IconLg => this.size_10(),
            })
            .when(!interactive, |this| this.opacity(0.5).cursor_default())
            .when(interactive, |this| {
                this.cursor_pointer().hover(move |style| {
                    let style = style.bg(hover_bg).text_color(accent_fg);
                    if is_link { style.underline() } else { style }
                })
            })
            .refine_style(&self.style);

        base.when(self.loading, |this| this.child(Spinner::new()))
            .when(!self.loading, |this| this.children(self.icon))
            .children(self.label)
            .children(self.children)
            .children(self.icon_end)
            .when_some(self.tooltip, |this, text| {
                this.tooltip(move |_, cx| cx.new(|_| SimpleTooltip(text.clone())).into())
            })
            .when_some(self.on_click.filter(|_| interactive), |this, handler| {
                this.on_click(move |event, window, cx| {
                    cx.stop_propagation();
                    handler(event, window, cx)
                })
            })
    }
}

/// Minimal tooltip view used by [`Button::tooltip`]; the `tooltip` component
/// offers richer content.
struct SimpleTooltip(SharedString);

impl gpui::Render for SimpleTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .mt_1()
            .px_3()
            .py_1p5()
            .rounded(theme.radius_md())
            .bg(theme.foreground)
            .text_color(theme.background)
            .text_xs()
            .child(self.0.clone())
    }
}

/// Convenience for `IconName` → icon-only button.
pub fn icon_button(id: impl Into<ElementId>, icon: IconName) -> Button {
    Button::new(id).ghost().size(ButtonSize::Icon).icon(icon)
}

//! A set of two-state buttons that can be toggled on or off.
//!
//! ```ignore
//! ToggleGroup::new("format")
//!     .multiple()
//!     .outline()
//!     .item(ToggleGroupItem::new("bold").icon(IconName::Bold))
//!     .item(ToggleGroupItem::new("italic").icon(IconName::Italic))
//!     .item(ToggleGroupItem::new("underline").icon(IconName::Underline))
//! ```

use gpui::{
    App, ElementId, IntoElement, RenderOnce, SharedString, StyleRefinement, Window, div,
    prelude::*,
};
use std::rc::Rc;

use super::icon::Icon;
use super::theme::ActiveTheme;
use super::toggle::{ToggleSize, ToggleVariant, toggle_base};
use super::utils::StyledExt;

pub struct ToggleGroupItem {
    value: SharedString,
    icon: Option<Icon>,
    label: Option<SharedString>,
    disabled: bool,
}

impl ToggleGroupItem {
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self { value: value.into(), icon: None, label: None, disabled: false }
    }

    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(IntoElement)]
pub struct ToggleGroup {
    id: ElementId,
    items: Vec<ToggleGroupItem>,
    multiple: bool,
    value: Option<Vec<SharedString>>,
    default_value: Vec<SharedString>,
    variant: ToggleVariant,
    size: ToggleSize,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&Vec<SharedString>, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(ToggleGroup);

impl ToggleGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            multiple: false,
            value: None,
            default_value: Vec::new(),
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            disabled: false,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: ToggleGroupItem) -> Self {
        self.items.push(item);
        self
    }

    /// `type="multiple"` (default is `"single"`).
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// Controlled pressed values.
    pub fn value(mut self, value: Vec<SharedString>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.default_value = value.into_iter().map(Into::into).collect();
        self
    }

    pub fn outline(mut self) -> Self {
        self.variant = ToggleVariant::Outline;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&Vec<SharedString>, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_value.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let values = self.value.clone().unwrap_or_else(|| state.read(cx).clone());
        let theme = cx.theme();
        let count = self.items.len();
        let outline = self.variant == ToggleVariant::Outline;
        let radius = theme.radius_md();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .rounded(radius)
            .when(outline && theme.shadows, |this| this.shadow_xs())
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map(|(ix, item)| {
                let pressed = values.contains(&item.value);
                let disabled = self.disabled || item.disabled;
                let (first, last) = (ix == 0, ix + 1 == count);
                let state = state.clone();
                let values = values.clone();
                let multiple = self.multiple;
                let on_change = self.on_change.clone();
                let value = item.value.clone();
                toggle_base(ix.into(), pressed, disabled, self.variant, self.size, theme)
                    .flex_1()
                    .rounded_none()
                    .when(first, |this| this.rounded_l(radius))
                    .when(last, |this| this.rounded_r(radius))
                    .when(outline, |this| this.shadow_none().when(!first, |this| this.border_l_0()))
                    .children(item.icon)
                    .children(item.label)
                    .when(!disabled, |this| {
                        this.on_click(move |_, window, cx| {
                            let next = if multiple {
                                let mut next = values.clone();
                                if pressed {
                                    next.retain(|v| v != &value);
                                } else {
                                    next.push(value.clone());
                                }
                                next
                            } else if pressed {
                                Vec::new()
                            } else {
                                vec![value.clone()]
                            };
                            state.update(cx, |s, cx| {
                                *s = next.clone();
                                cx.notify();
                            });
                            if let Some(handler) = &on_change {
                                handler(&next, window, cx);
                            }
                        })
                    })
            }))
    }
}

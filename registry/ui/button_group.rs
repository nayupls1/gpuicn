//! A container that groups related buttons together with consistent styling.
//!
//! ```ignore
//! ButtonGroup::new()
//!     .button(Button::new("archive").outline().label("Archive"))
//!     .button(Button::new("report").outline().label("Report"))
//!     .separator()
//!     .button(Button::new("snooze").outline().icon(IconName::Ellipsis).size(ButtonSize::Icon))
//! ```

use gpui::{
    AnyElement, App, IntoElement, RenderOnce, SharedString, StyleRefinement, Window, div,
    prelude::*,
};

use super::button::Button;
use super::theme::ActiveTheme;
use super::utils::{Orientation, StyledExt};

enum GroupItem {
    Button(Button),
    Text(SharedString),
    Separator,
    Element(AnyElement),
}

#[derive(IntoElement)]
pub struct ButtonGroup {
    items: Vec<GroupItem>,
    orientation: Orientation,
    style: StyleRefinement,
}

crate::impl_styled!(ButtonGroup);

impl ButtonGroup {
    pub fn new() -> Self {
        Self { items: Vec::new(), orientation: Orientation::Horizontal, style: StyleRefinement::default() }
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    pub fn button(mut self, button: Button) -> Self {
        self.items.push(GroupItem::Button(button));
        self
    }

    pub fn buttons(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        self.items.extend(buttons.into_iter().map(GroupItem::Button));
        self
    }

    /// `ButtonGroupText` — a static label segment.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.items.push(GroupItem::Text(text.into()));
        self
    }

    /// `ButtonGroupSeparator`.
    pub fn separator(mut self) -> Self {
        self.items.push(GroupItem::Separator);
        self
    }

    /// Any other element (an input, a select…), inserted as-is.
    pub fn element(mut self, element: impl IntoElement) -> Self {
        self.items.push(GroupItem::Element(element.into_any_element()));
        self
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let radius = theme.radius_md();
        let vertical = self.orientation == Orientation::Vertical;
        let count = self.items.len();

        div()
            .flex()
            .when(vertical, |this| this.flex_col())
            .items_stretch_if(vertical)
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map(|(ix, item)| {
                let (first, last) = (ix == 0, ix + 1 == count);
                match item {
                    GroupItem::Button(button) => {
                        let button = button.rounded_none();
                        let button = match (vertical, first, last) {
                            (false, true, true) | (true, true, true) => button.rounded(radius),
                            (false, true, false) => button.rounded_l(radius),
                            (false, false, true) => button.rounded_r(radius),
                            (true, true, false) => button.rounded_t(radius),
                            (true, false, true) => button.rounded_b(radius),
                            _ => button,
                        };
                        // Collapse double borders between neighbours.
                        let button = match (vertical, first) {
                            (false, false) => button.border_l_0(),
                            (true, false) => button.border_t_0(),
                            _ => button,
                        };
                        button.into_any_element()
                    }
                    GroupItem::Text(text) => div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_4()
                        .text_sm()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .bg(theme.muted)
                        .border_1()
                        .border_color(theme.border)
                        .when(!first, |this| if vertical { this.border_t_0() } else { this.border_l_0() })
                        .map(|this| match (vertical, first, last) {
                            (false, true, _) => this.rounded_l(radius),
                            (false, _, true) => this.rounded_r(radius),
                            (true, true, _) => this.rounded_t(radius),
                            (true, _, true) => this.rounded_b(radius),
                            _ => this,
                        })
                        .when(theme.shadows, |this| this.shadow_xs())
                        .child(text)
                        .into_any_element(),
                    GroupItem::Separator => div()
                        .flex_none()
                        .bg(theme.input)
                        .map(|this| if vertical { this.h_px().w_full() } else { this.w_px() })
                        .into_any_element(),
                    GroupItem::Element(element) => element,
                }
            }))
    }
}

trait ItemsStretch {
    fn items_stretch_if(self, condition: bool) -> Self;
}

impl ItemsStretch for gpui::Div {
    fn items_stretch_if(mut self, condition: bool) -> Self {
        if condition {
            self.style().align_items = Some(gpui::AlignItems::Stretch);
        }
        self
    }
}

//! Display additional information or actions next to an input or textarea.
//!
//! ```ignore
//! InputGroup::new(Input::new("search").placeholder("Search..."))
//!     .start(InputGroupAddon::icon(IconName::Search))
//!     .end(InputGroupAddon::text("12 results"))
//!
//! InputGroup::new(Input::new("url").placeholder("example.com"))
//!     .start(InputGroupAddon::text("https://"))
//!     .end(InputGroupAddon::button(Button::new("copy").ghost().size(ButtonSize::IconSm).icon(IconName::Copy)))
//! ```

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px,
};

use super::button::Button;
use super::icon::{Icon, IconName};
use super::input::Input;
use super::theme::ActiveTheme;
use super::utils::{StyledExt, ring_shadow};

/// Content placed beside the input (`InputGroupAddon`).
pub struct InputGroupAddon(AnyElement);

impl InputGroupAddon {
    pub fn icon(icon: IconName) -> Self {
        Self(Icon::new(icon).into_any_element())
    }

    /// `InputGroupText`
    pub fn text(text: impl Into<SharedString>) -> Self {
        Self(div().whitespace_nowrap().child(text.into()).into_any_element())
    }

    /// `InputGroupButton`
    pub fn button(button: Button) -> Self {
        Self(button.into_any_element())
    }

    pub fn element(element: impl IntoElement) -> Self {
        Self(element.into_any_element())
    }
}

#[derive(IntoElement)]
pub struct InputGroup {
    input: Input,
    start: Vec<AnyElement>,
    end: Vec<AnyElement>,
    block_start: Vec<AnyElement>,
    block_end: Vec<AnyElement>,
    invalid: bool,
    style: StyleRefinement,
}

crate::impl_styled!(InputGroup);

impl InputGroup {
    pub fn new(input: Input) -> Self {
        Self {
            input,
            start: Vec::new(),
            end: Vec::new(),
            block_start: Vec::new(),
            block_end: Vec::new(),
            invalid: false,
            style: StyleRefinement::default(),
        }
    }

    /// `align="inline-start"`
    pub fn start(mut self, addon: InputGroupAddon) -> Self {
        self.start.push(addon.0);
        self
    }

    /// `align="inline-end"`
    pub fn end(mut self, addon: InputGroupAddon) -> Self {
        self.end.push(addon.0);
        self
    }

    /// `align="block-start"` — a row above the input (for textareas).
    pub fn block_start(mut self, addon: InputGroupAddon) -> Self {
        self.block_start.push(addon.0);
        self
    }

    /// `align="block-end"` — a row below the input.
    pub fn block_end(mut self, addon: InputGroupAddon) -> Self {
        self.block_end.push(addon.0);
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}

impl RenderOnce for InputGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Resolve the input's state here so the focus ring can live on the group.
        let state = self.input.state(window, cx);
        let focused = state.read(cx).is_focused(window);
        let input = self.input.with_state(&state).unstyled();
        let theme = cx.theme();
        let muted_fg = theme.muted_foreground;
        let addon_row = |items: Vec<AnyElement>| {
            div().flex().items_center().gap_2().px_3().text_sm().text_color(muted_fg).children(items)
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.input)
            .when(theme.is_dark(), |this| this.bg(theme.input.opacity(0.3)))
            .when(theme.shadows, |this| this.shadow_xs())
            .when(focused && !self.invalid, |this| {
                this.border_color(theme.ring).shadow(vec![ring_shadow(theme.ring.opacity(0.5), px(3.))])
            })
            .when(self.invalid, |this| this.border_color(theme.destructive))
            .refine_style(&self.style)
            .when(!self.block_start.is_empty(), |this| this.child(addon_row(self.block_start).pt_3()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .min_h_9()
                    .when(!self.start.is_empty(), |this| this.child(addon_row(self.start).pr_0()))
                    .child(div().flex_1().min_w_0().px_3().py_1().child(input))
                    .when(!self.end.is_empty(), |this| this.child(addon_row(self.end).pl_0().pr_1p5())),
            )
            .when(!self.block_end.is_empty(), |this| this.child(addon_row(self.block_end).pb_3()))
    }
}

//! Use the Empty component to display an empty state.
//!
//! ```ignore
//! Empty::new()
//!     .icon(IconName::Folder)
//!     .title("No Projects Yet")
//!     .description("You haven't created any projects yet.")
//!     .child(Button::new("create").label("Create Project"))
//! ```

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, px,
};

use super::icon::Icon;
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Empty {
    media: Option<AnyElement>,
    icon: Option<Icon>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    outline: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Empty);

impl Empty {
    pub fn new() -> Self {
        Self {
            media: None,
            icon: None,
            title: None,
            description: None,
            outline: false,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    /// Icon in a muted tile (`EmptyMedia variant="icon"`).
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Arbitrary media, e.g. an avatar group.
    pub fn media(mut self, media: impl IntoElement) -> Self {
        self.media = Some(media.into_any_element());
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Dashed border around the empty state.
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }
}

impl RenderOnce for Empty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let icon_tile = self.icon.map(|icon| {
            div()
                .flex()
                .flex_none()
                .size_10()
                .items_center()
                .justify_center()
                .rounded(theme.radius_lg())
                .bg(theme.muted)
                .text_color(theme.foreground)
                .child(icon.size_6())
                .into_any_element()
        });

        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .items_center()
            .justify_center()
            .gap_6()
            .p_12()
            .rounded(theme.radius_lg())
            .when(self.outline, |this| this.border_1().border_dashed().border_color(theme.border))
            .refine_style(&self.style)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .max_w(px(384.))
                    .items_center()
                    .gap_2()
                    .text_center()
                    .children(self.media.or(icon_tile).map(|media| div().mb_2().child(media)))
                    .children(self.title.map(|title| {
                        div().text_lg().font_weight(FontWeight::MEDIUM).child(title)
                    }))
                    .children(self.description.map(|description| {
                        div()
                            .text_sm()
                            .line_height(gpui::relative(1.625))
                            .text_color(theme.muted_foreground)
                            .child(description)
                    })),
            )
            .when(!self.children.is_empty(), |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .max_w(px(384.))
                        .items_center()
                        .gap_4()
                        .text_sm()
                        .children(self.children),
                )
            })
    }
}

//! A versatile component that you can use to display any content.
//!
//! ```ignore
//! Item::new("security")
//!     .outline()
//!     .icon(IconName::CircleCheck)
//!     .title("Your profile has been verified.")
//!     .description("Two-factor authentication is enabled.")
//!     .action(Button::new("review").outline().size(ButtonSize::Sm).label("Review"))
//! ```

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Window, div, prelude::*,
};
use std::rc::Rc;

use super::icon::Icon;
use super::separator::Separator;
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

#[derive(IntoElement)]
pub struct Item {
    id: ElementId,
    variant: ItemVariant,
    size: ItemSize,
    media: Option<AnyElement>,
    title: Option<SharedString>,
    description: Option<SharedString>,
    actions: Vec<AnyElement>,
    children: Vec<AnyElement>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled_parent!(Item, ItemGroup);

impl Item {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: ItemVariant::Default,
            size: ItemSize::Default,
            media: None,
            title: None,
            description: None,
            actions: Vec::new(),
            children: Vec::new(),
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outline(self) -> Self {
        self.variant(ItemVariant::Outline)
    }

    pub fn muted(self) -> Self {
        self.variant(ItemVariant::Muted)
    }

    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    /// `ItemMedia variant="icon"`.
    pub fn icon(self, icon: impl Into<Icon>) -> Self {
        let icon = icon.into();
        self.media(ItemMediaIcon(icon))
    }

    /// `ItemMedia` with arbitrary content (avatar, image, …).
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

    /// Element in the trailing `ItemActions` slot.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// Make the whole item clickable (like `<Item asChild><a/></Item>`).
    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

#[derive(IntoElement)]
struct ItemMediaIcon(Icon);

impl RenderOnce for ItemMediaIcon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .flex()
            .flex_none()
            .size_8()
            .items_center()
            .justify_center()
            .rounded(theme.radius_sm())
            .border_1()
            .border_color(theme.border)
            .bg(theme.muted)
            .child(self.0)
    }
}

impl RenderOnce for Item {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let hover = theme.accent.opacity(0.5);
        let has_content = self.title.is_some() || self.description.is_some() || !self.children.is_empty();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(gpui::transparent_black())
            .text_sm()
            .map(|this| match self.variant {
                ItemVariant::Default => this,
                ItemVariant::Outline => this.border_color(theme.border),
                ItemVariant::Muted => this.bg(theme.muted.opacity(0.5)),
            })
            .map(|this| match self.size {
                ItemSize::Default => this.p_4().gap_4(),
                ItemSize::Sm => this.py_3().px_4().gap_2p5(),
            })
            .refine_style(&self.style)
            .children(self.media)
            .when(has_content, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .gap_1()
                        .children(self.title.map(|title| {
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .font_weight(FontWeight::MEDIUM)
                                .line_height(gpui::relative(1.375))
                                .child(title)
                        }))
                        .children(self.description.map(|description| {
                            div()
                                .line_clamp(2)
                                .text_color(theme.muted_foreground)
                                .child(description)
                        }))
                        .children(self.children),
                )
            })
            .when(!self.actions.is_empty(), |this| {
                this.child(div().flex().items_center().gap_2().children(self.actions))
            })
            .when_some(self.on_click, |this, handler| {
                this.cursor_pointer()
                    .hover(move |style| style.bg(hover))
                    .on_click(move |event, window, cx| handler(event, window, cx))
            })
    }
}

/// A vertical list of items, optionally separated.
#[derive(IntoElement)]
pub struct ItemGroup {
    separated: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl ItemGroup {
    pub fn new() -> Self {
        Self { separated: false, children: Vec::new(), style: StyleRefinement::default() }
    }

    /// Insert an `ItemSeparator` between items.
    pub fn separated(mut self) -> Self {
        self.separated = true;
        self
    }
}

impl RenderOnce for ItemGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let separated = self.separated;
        let mut children = Vec::new();
        for (ix, child) in self.children.into_iter().enumerate() {
            if separated && ix > 0 {
                children.push(Separator::horizontal().into_any_element());
            }
            children.push(child);
        }
        div().flex().flex_col().refine_style(&self.style).children(children)
    }
}

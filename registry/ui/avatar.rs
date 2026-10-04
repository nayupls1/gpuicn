//! An image element with a fallback for representing the user.
//!
//! ```ignore
//! Avatar::new().src("https://github.com/shadcn.png").fallback("CN")
//! ```

use gpui::{
    App, ImageSource, IntoElement, ObjectFit, RenderOnce, SharedString, StyleRefinement, Window,
    div, img, prelude::*,
};

use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Avatar {
    src: Option<ImageSource>,
    fallback: SharedString,
    style: StyleRefinement,
}

crate::impl_styled!(Avatar);

impl Avatar {
    pub fn new() -> Self {
        Self { src: None, fallback: SharedString::default(), style: StyleRefinement::default() }
    }

    /// URL, file path or embedded asset. Remote URLs need an HTTP client on
    /// the gpui `Application`; otherwise the fallback is shown.
    pub fn src(mut self, src: impl Into<ImageSource>) -> Self {
        self.src = Some(src.into());
        self
    }

    /// Shown while loading or when there is no image (usually initials).
    pub fn fallback(mut self, fallback: impl Into<SharedString>) -> Self {
        self.fallback = fallback.into();
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let muted = theme.muted;
        let fallback = self.fallback;
        let fallback_el = move || {
            div()
                .flex()
                .size_full()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(muted)
                .text_sm()
                .child(fallback.clone())
                .into_any_element()
        };

        div()
            .relative()
            .flex()
            .flex_none()
            .size_8()
            .overflow_hidden()
            .rounded_full()
            .refine_style(&self.style)
            .map(|this| match self.src {
                Some(src) => this.child(
                    img(src)
                        .size_full()
                        .object_fit(ObjectFit::Cover)
                        .with_loading({
                            let fallback_el = fallback_el.clone();
                            move || fallback_el()
                        })
                        .with_fallback(fallback_el),
                ),
                None => this.child(fallback_el()),
            })
    }
}

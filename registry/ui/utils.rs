//! Small helpers shared by every component — the gpui counterpart of
//! shadcn's `lib/utils.ts`.

use gpui::{BoxShadow, Div, Hsla, Pixels, Refineable, StyleRefinement, Styled, div, point, prelude::*, px};

use super::theme::Theme;

/// Extension methods available on every [`Styled`] element.
pub trait StyledExt: Styled + Sized {
    /// Merge user-provided style overrides on top of a component's defaults.
    ///
    /// This is what `cn(defaults, className)` does in shadcn: anything the
    /// caller set through the component's `Styled` impl wins over the defaults.
    fn refine_style(mut self, style: &StyleRefinement) -> Self {
        self.style().refine(style);
        self
    }

    /// The surface of a form control (`bg-transparent dark:bg-input/30 shadow-xs`).
    ///
    /// gpui paints box shadows as a filled shape *under* the element (CSS clips
    /// them to the outside), so a shadow only looks right on an opaque
    /// background. In light mode controls get the page background; in dark
    /// mode they stay translucent and skip the (invisible anyway) shadow.
    fn control_surface(self, theme: &Theme) -> Self {
        if theme.is_dark() {
            self.bg(theme.input.opacity(0.3))
        } else {
            let this = self.bg(theme.background);
            if theme.shadows { this.shadow_xs() } else { this }
        }
    }
}

impl<E: Styled> StyledExt for E {}

/// A tailwind `ring-[width]` drawn *outside* an element, as an overlay child.
///
/// Add it as the last child of a `.relative()` element whose border is
/// `border` wide and whose corner radius is `radius`:
///
/// ```ignore
/// div().relative().border_1().rounded(r).when(focused, |this| this.child(ring(theme.ring.opacity(0.5), px(3.), r, px(1.))))
/// ```
///
/// Unlike a spread [`BoxShadow`], it doesn't tint the element's interior and
/// its corners follow the element's radius.
pub fn ring(color: Hsla, width: Pixels, radius: Pixels, border: Pixels) -> Div {
    let inset = -(width + border);
    div()
        .absolute()
        .top(inset)
        .left(inset)
        .right(inset)
        .bottom(inset)
        .rounded(radius + width)
        .border(width)
        .border_color(color)
}

/// The standard `focus-visible:ring-[3px] ring-ring/50` focus ring (see [`ring`]).
pub fn focus_ring(theme: &Theme, radius: Pixels) -> Div {
    ring(theme.ring.opacity(0.5), px(3.0), radius, px(1.0))
}

/// The `aria-invalid:ring-destructive/20` ring (see [`ring`]).
pub fn invalid_ring(theme: &Theme, radius: Pixels) -> Div {
    let alpha = if theme.is_dark() { 0.4 } else { 0.2 };
    ring(theme.destructive.opacity(alpha), px(3.0), radius, px(1.0))
}

/// A solid, unblurred "ring" around an element, like tailwind's `ring-*`.
///
/// Prefer [`ring`]: shadows fill the element's interior in gpui, so this only
/// looks right on elements with an opaque background.
pub fn ring_shadow(color: Hsla, width: Pixels) -> BoxShadow {
    BoxShadow {
        color,
        offset: point(px(0.0), px(0.0)),
        blur_radius: px(0.0),
        spread_radius: width,
    }
}

/// Which side of the trigger floating content appears on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

/// How floating content is aligned along the trigger's edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    Start,
    #[default]
    Center,
    End,
}

/// Horizontal or vertical layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Implements [`Styled`] and [`gpui::ParentElement`] for a component struct
/// that has `style: StyleRefinement` and `children: Vec<AnyElement>` fields.
#[macro_export]
macro_rules! impl_styled_parent {
    ($($ty:ty),* $(,)?) => {
        $(
            impl gpui::Styled for $ty {
                fn style(&mut self) -> &mut gpui::StyleRefinement {
                    &mut self.style
                }
            }

            impl gpui::ParentElement for $ty {
                fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
                    self.children.extend(elements)
                }
            }
        )*
    };
}

/// Implements [`Styled`] for a component struct with a `style` field.
#[macro_export]
macro_rules! impl_styled {
    ($($ty:ty),* $(,)?) => {
        $(
            impl gpui::Styled for $ty {
                fn style(&mut self) -> &mut gpui::StyleRefinement {
                    &mut self.style
                }
            }
        )*
    };
}

/// An invisible view used as the drag "ghost" for resize/slider drags.
pub struct InvisibleDrag;

impl gpui::Render for InvisibleDrag {
    fn render(&mut self, _: &mut gpui::Window, _: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        gpui::Empty
    }
}

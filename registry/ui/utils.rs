//! Small helpers shared by every component — the gpui counterpart of
//! shadcn's `lib/utils.ts`.

use gpui::{BoxShadow, Hsla, Pixels, Refineable, StyleRefinement, Styled, point, px};

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

    /// The `focus-visible:ring-[3px] ring-ring/50` focus ring.
    fn focus_ring(self, ring: Hsla) -> Self {
        self.shadow(vec![ring_shadow(ring.opacity(0.5), px(3.0))])
    }
}

impl<E: Styled> StyledExt for E {}

/// A solid, unblurred "ring" around an element, like tailwind's `ring-*`.
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

/// Runs `handler` when the mouse leaves the window.
///
/// gpui keeps the last mouse position when the pointer leaves the window, so
/// `on_hover(false)` never fires. Add this (it's invisible) next to anything
/// that tracks hover itself. Note that gpui's built-in `.hover(..)` styles
/// have the same limitation and stay applied until the mouse comes back.
pub fn on_mouse_leave_window(handler: impl Fn(&mut gpui::Window, &mut gpui::App) + 'static) -> impl gpui::IntoElement {
    use gpui::Styled as _;
    gpui::canvas(
        |_, _, _| {},
        move |_, _, window, _| {
            window.on_mouse_event(move |_: &gpui::MouseExitEvent, phase, window, cx| {
                if phase == gpui::DispatchPhase::Bubble {
                    handler(window, cx);
                }
            });
        },
    )
    .absolute()
    .size_0()
}

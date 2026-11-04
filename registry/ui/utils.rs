//! Small helpers shared by every component — the gpui counterpart of
//! shadcn's `lib/utils.ts`.

use gpui::{
    App, BoxShadow, Div, ElementId, FocusHandle, Global, Hsla, KeyBinding, Pixels, Refineable, StyleRefinement,
    Styled, Window, actions, div, point, prelude::*, px,
};

use super::theme::Theme;

actions!(gpuicn, [FocusNext, FocusPrevious]);

/// Whether focus last moved by keyboard, i.e. whether `focus-visible` rings
/// should show. Tab sets it; a pointer press on a component clears it.
#[derive(Default)]
struct KeyboardFocus(bool);

impl Global for KeyboardFocus {}

/// Bind Tab / Shift-Tab to move focus between tab stops (buttons, inputs,
/// checkboxes, switches...). Called by `ui::init`.
pub fn init(cx: &mut App) {
    cx.set_global(KeyboardFocus(false));
    cx.bind_keys([KeyBinding::new("tab", FocusNext, None), KeyBinding::new("shift-tab", FocusPrevious, None)]);
    cx.on_action(|_: &FocusNext, cx| move_focus(cx, true));
    cx.on_action(|_: &FocusPrevious, cx| move_focus(cx, false));
}

fn move_focus(cx: &mut App, forward: bool) {
    cx.set_global(KeyboardFocus(true));
    // Fall back to the only window when none reports itself active (some
    // window managers, headless X servers).
    let window = cx.active_window().or_else(|| match cx.windows().as_slice() {
        [only] => Some(*only),
        _ => None,
    });
    if let Some(window) = window {
        // Actions are dispatched while the window is checked out, so update it
        // once dispatch is done.
        cx.defer(move |cx| {
            window
                .update(cx, |_, window, _| if forward { window.focus_next() } else { window.focus_prev() })
                .ok();
        });
    }
}

/// `:focus-visible` for `handle`: focused, and focus arrived by keyboard.
pub fn is_focus_visible(handle: &FocusHandle, window: &Window, cx: &App) -> bool {
    handle.is_focused(window) && cx.try_global::<KeyboardFocus>().is_some_and(|k| k.0)
}

/// Forget keyboard focus mode (call on pointer presses).
pub fn pointer_pressed(cx: &mut App) {
    if cx.try_global::<KeyboardFocus>().is_some_and(|k| k.0) {
        cx.set_global(KeyboardFocus(false));
    }
}

/// Hover state kept across frames, for hover effects on *text*.
///
/// gpui applies `.hover(..)` styles after text has been laid out, so a hover
/// text color or underline never shows (background changes do). Track the
/// hover yourself and style the element from it:
///
/// ```ignore
/// let hover = use_hover(("link", ix), window, cx);
/// div().id(..).on_hover(hover.listener()).when(hover.get(cx), |this| this.underline())
/// ```
#[derive(Clone)]
pub struct HoverState(gpui::Entity<bool>);

impl HoverState {
    pub fn get(&self, cx: &App) -> bool {
        *self.0.read(cx)
    }

    /// Pass to `.on_hover(..)`.
    pub fn listener(&self) -> impl Fn(&bool, &mut Window, &mut App) + 'static {
        let state = self.0.clone();
        move |hovered, _, cx| {
            state.update(cx, |value, cx| {
                if *value != *hovered {
                    *value = *hovered;
                    cx.notify();
                }
            })
        }
    }
}

pub fn use_hover(id: impl Into<ElementId>, window: &mut Window, cx: &mut App) -> HoverState {
    HoverState(window.use_keyed_state(id, cx, |_, _| false))
}

/// A focus handle that is a tab stop, kept alive across frames under `id`.
pub fn use_tab_stop(id: impl Into<ElementId>, window: &mut Window, cx: &mut App) -> FocusHandle {
    window.use_keyed_state(id, cx, |_, cx| cx.focus_handle().tab_stop(true)).read(cx).clone()
}

/// Extension methods available on every [`Styled`] element.
pub trait StyledExt: Styled + Sized {
    /// Merge user-provided style overrides on top of a component's defaults.
    ///
    /// This is what `cn(defaults, className)` does in shadcn: anything the
    /// caller set through the component's `Styled` impl wins over the defaults.
    fn refine_style(mut self, style: &StyleRefinement) -> Self {
        // gpui replaces the whole text block when refining (it isn't marked
        // `#[refineable]`), so `.text_center()` from a caller would drop the
        // component's font size, weight and color. Merge it field by field,
        // the way CSS classes combine.
        let base_text = self.style().text.take();
        self.style().refine(style);
        if let Some(mut text) = base_text {
            if let Some(overrides) = &style.text {
                text.refine(overrides);
            }
            self.style().text = Some(text);
        }
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

/// Border for small toggles (checkbox, radio). `border-input` plus the CSS
/// `shadow-xs` reads clearly on the web; gpui's lighter shadow doesn't, so
/// light mode nudges the border toward the foreground.
pub fn control_border(theme: &Theme) -> Hsla {
    if theme.is_dark() {
        theme.input
    } else {
        super::theme::mix(theme.input, theme.foreground, 0.12)
    }
}

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

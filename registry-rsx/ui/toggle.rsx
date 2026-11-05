//! A two-state button that can be either on or off.
//!
//! ```ignore
//! <Toggle icon={IconName::Bold} />
//! <Toggle outline icon={IconName::Italic} label="Italic" />
//! // controlled
//! <Toggle pressed={bold} onChange={move |on: &bool| bold.set(*on)} icon={IconName::Bold} />
//! ```

use rsx::prelude::*;
use rsx::Div;

use super::icon::{Icon, IconName};
use super::theme::{mix, theme, ColorExt, Theme};
use super::utils::{is_focus_visible, on_tab_key, pointer_pressed, ring};


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ToggleSize {
    /// `(height, horizontal padding)` in px (`h-9 px-2 min-w-9`, ...).
    pub fn metrics(self) -> (f32, f32) {
        match self {
            ToggleSize::Default => (36.0, 8.0),
            ToggleSize::Sm => (32.0, 6.0),
            ToggleSize::Lg => (40.0, 10.0),
        }
    }
}

/// Colors of a toggle in a given state: the shared look of `Toggle` and
/// `ToggleGroup` items.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToggleLook {
    pub bg: Color,
    /// `None`: inherit the surrounding text color.
    pub fg: Option<Color>,
    pub border: Option<Color>,
}

/// The shared look of `Toggle` and `ToggleGroup` items (gpui's `toggle_base`).
pub fn toggle_look(pressed: bool, hovered: bool, disabled: bool, variant: ToggleVariant, theme: &Theme) -> ToggleLook {
    let outline = variant == ToggleVariant::Outline;
    // Opaque in light mode so the shadow can't tint the interior.
    let mut look = ToggleLook {
        bg: if outline && !theme.is_dark() { theme.background } else { Color::TRANSPARENT },
        fg: None,
        border: outline.then_some(theme.input),
    };
    if pressed {
        // `data-[state=on]:bg-accent`, nudged a little darker than the hover
        // fill so "on" and "hovered" stay distinguishable.
        look.bg = mix(theme.accent, theme.foreground, 0.06);
        look.fg = Some(theme.accent_foreground);
        if outline {
            // Keep the outline variant's edge visible on the darker fill.
            look.border = Some(mix(theme.input, theme.foreground, 0.12));
        }
    } else if hovered && !disabled {
        let (bg, fg) = match variant {
            ToggleVariant::Default => (theme.muted, theme.muted_foreground),
            ToggleVariant::Outline => (theme.accent, theme.accent_foreground),
        };
        look.bg = bg;
        look.fg = Some(fg);
    }
    look
}

/// Where a toggle sits in a `ToggleGroup`: which corners stay rounded and
/// whether its left border collapses into its neighbour's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GroupPosition {
    pub first: bool,
    pub last: bool,
}

/// The button behind `Toggle` and each `ToggleGroup` item. The element stays
/// the same while its state changes (only its colors are bound), so it keeps
/// keyboard focus: Space / Enter toggle it while focused.
#[doc(hidden)]
#[component]
pub fn ToggleButton(
    pressed: Accessor<bool>,
    disabled: Accessor<bool>,
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional)] size: ToggleSize,
    /// Set inside a `ToggleGroup`.
    group: Option<GroupPosition>,
    on_toggle: Callback<()>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let node = NodeRef::new();
    let hovered = signal(false);
    let look = memo(move || toggle_look(pressed.get(), hovered.get(), disabled.get(), variant, &t.get()));
    let (h, pad) = size.metrics();
    let outline = variant == ToggleVariant::Outline;
    let border_left = if group.is_some_and(|g| !g.first) { 0.0 } else { 1.0 };
    let border = if outline { 1.0 } else { 0.0 };

    <div
        ref={node}
        class="relative flex flex-none items-center justify-center gap-2 text-sm leading-[1.25rem] font-medium"
        class:flex-1={group.is_some()}
        class:cursor-pointer={!disabled.get()}
        h={px(h)}
        min_w={px(h)}
        px={px(pad)}
        bind={move |el: Div| radii(el, group, t.get().radius - 2.0)}
        bind={move |el: Div| border_sides(el, border, border_left)}
        bg={look.get().bg}
        border_color={look.get().border.unwrap_or(Color::TRANSPARENT)}
        bind={move |el: Div| match look.get().fg { Some(fg) => el.text_color(fg), None => el }}
        opacity={if disabled.get() { 0.5 } else { 1.0 }}
        onMouseDown={move |_| pointer_pressed()}
        onKeyDown={on_tab_key}
        onHover={move |h: &bool| hovered.set(*h)}
        onClick={move |e: &ClickEvent| {
            if !disabled.get_untracked() {
                e.stop_propagation();
                on_toggle.run();
            }
        }}
        {...&attrs}
    >
        {children}
        {move || {
            let theme = t.get();
            (is_focus_visible(node) && !disabled.get()).then(|| {
                // rsx#9: no `z-*`, so a group item's ring may sit under its right neighbour.
                ring(theme.ring.opacity(0.5), 3.0, theme.radius - 2.0, border)
            })
        }}
    </div>
}

/// `rounded-md`, or only the outer corners inside a group.
fn radii(mut el: Div, group: Option<GroupPosition>, r: f32) -> Div {
    let r = px(r.max(0.0));
    let zero = px(0.0);
    let (left, right) = match group {
        None => (r, r),
        Some(g) => (if g.first { r } else { zero }, if g.last { r } else { zero }),
    };
    let c = &mut el.style().corner_radii;
    c.top_left = Some(left);
    c.bottom_left = Some(left);
    c.top_right = Some(right);
    c.bottom_right = Some(right);
    el
}

/// `border` (or `border border-l-0` for a group item after the first).
fn border_sides(mut el: Div, width: f32, left: f32) -> Div {
    let b = &mut el.style().border_widths;
    b.top = Some(px(width));
    b.right = Some(px(width));
    b.bottom = Some(px(width));
    b.left = Some(px(width * left));
    el
}

#[component]
pub fn Toggle(
    /// Controlled pressed state.
    pressed: Option<Accessor<bool>>,
    default_pressed: bool,
    disabled: Option<Accessor<bool>>,
    #[prop(optional)] variant: ToggleVariant,
    /// Shorthand for `variant={ToggleVariant::Outline}`.
    outline: bool,
    #[prop(optional)] size: ToggleSize,
    icon: Option<IconName>,
    label: Option<Accessor<String>>,
    on_change: Option<Callback<bool>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let variant = if outline { ToggleVariant::Outline } else { variant };
    let state = signal(default_pressed);
    let is_pressed = memo(move || pressed.map(|p| p.get()).unwrap_or_else(|| state.get()));
    let is_disabled = memo(move || disabled.map(|d| d.get()).unwrap_or(false));

    <ToggleButton
        pressed={is_pressed}
        disabled={is_disabled}
        variant={variant}
        size={size}
        onToggle={move |_| {
            let next = !is_pressed.get_untracked();
            state.set(next);
            if let Some(f) = on_change {
                f.call(&next);
            }
        }}
        {...attrs}
    >
        {icon.map(|i| <Icon name={i} />)}
        {label}
        {children.map(|c| c.render())}
    </ToggleButton>
}

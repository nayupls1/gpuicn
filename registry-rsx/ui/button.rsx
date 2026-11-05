//! Displays a button or a component that looks like a button.
//!
//! ```ignore
//! <Button label="Save changes" />
//! <Button destructive icon={IconName::Trash} label="Delete" />
//! <Button ghost size={ButtonSize::Icon} icon={IconName::Ellipsis} />
//! <Button outline onClick={move |_| count.update(|n| *n += 1)}>Clicked {count} times</Button>
//! ```

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::spinner::Spinner;
use super::theme::{theme, ColorExt, Theme};
use super::tooltip::Tooltip;
use super::utils::{is_focus_visible, on_tab_key, pointer_pressed, ring};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub fn is_icon(self) -> bool {
        matches!(self, ButtonSize::Icon | ButtonSize::IconSm | ButtonSize::IconLg)
    }
}

/// Colors for a button variant: `(background, hover background, text, border)`.
pub fn variant_colors(variant: ButtonVariant, theme: &Theme) -> (Color, Color, Color, Option<Color>) {
    let clear = Color::TRANSPARENT;
    match variant {
        ButtonVariant::Default => (theme.primary, theme.primary.opacity(0.9), theme.primary_foreground, None),
        ButtonVariant::Destructive => {
            let bg = if theme.is_dark() { theme.destructive.opacity(0.6) } else { theme.destructive };
            (bg, bg.opacity(0.9), Color::WHITE, None)
        }
        ButtonVariant::Outline => {
            if theme.is_dark() {
                // Stronger than `input/50` so the hover reads on dark backgrounds.
                (theme.input.opacity(0.3), theme.accent, theme.foreground, Some(theme.input))
            } else {
                (theme.background, theme.accent, theme.foreground, Some(theme.border))
            }
        }
        ButtonVariant::Secondary => (theme.secondary, theme.secondary.opacity(0.8), theme.secondary_foreground, None),
        ButtonVariant::Ghost => {
            let hover = if theme.is_dark() { theme.accent.opacity(0.5) } else { theme.accent };
            (clear, hover, theme.foreground, None)
        }
        ButtonVariant::Link => (clear, clear, theme.primary, None),
    }
}

/// Context for containers that join buttons into one control (`ButtonGroup`):
/// each `Button` created inside calls it once, and applies the attributes it
/// returns to its root, after its own defaults and before the caller's.
#[derive(Clone)]
pub struct ButtonJoin(pub std::rc::Rc<dyn Fn() -> Attributes>);

#[component]
pub fn Button(
    /// Text shown after the icon.
    label: Option<Accessor<String>>,
    /// Icon shown before the label.
    icon: Option<IconName>,
    /// Icon shown after the label.
    icon_end: Option<IconName>,
    #[prop(optional)] variant: ButtonVariant,
    /// Shorthands for `variant`: `<Button outline>`.
    destructive: bool,
    outline: bool,
    secondary: bool,
    ghost: bool,
    link: bool,
    #[prop(optional)] size: ButtonSize,
    disabled: Option<Accessor<bool>>,
    /// Show a spinner in place of the icon and ignore clicks.
    loading: Option<Accessor<bool>>,
    /// Render in the hovered (accent) state, e.g. for an open menu trigger.
    selected: Option<Accessor<bool>>,
    /// A tooltip centered above the button.
    tooltip: Option<String>,
    on_click: Option<Callback<ClickEvent>>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let variant = if destructive {
        ButtonVariant::Destructive
    } else if outline {
        ButtonVariant::Outline
    } else if secondary {
        ButtonVariant::Secondary
    } else if ghost {
        ButtonVariant::Ghost
    } else if link {
        ButtonVariant::Link
    } else {
        variant
    };
    let t = theme();
    let join = use_context::<ButtonJoin>().map(|j| (j.0)());
    let node = NodeRef::new();
    let hovered = signal(false);
    let flag = |a: Option<Accessor<bool>>| move || a.map(|a| a.get()).unwrap_or(false);
    let (disabled, loading, selected) = (flag(disabled), flag(loading), flag(selected));
    let interactive = move || !disabled() && !loading();

    // Everything that changes is a binding, so the button keeps its focus.
    let style = move |el: rsx::Div| {
        let theme = t.get();
        let (bg, hover_bg, fg, border) = variant_colors(variant, &theme);
        let accent_fg = match variant {
            ButtonVariant::Outline | ButtonVariant::Ghost => theme.accent_foreground,
            _ => fg,
        };
        let interactive = interactive();
        let has_icon = icon.is_some() || loading();
        // `disabled:opacity-50`, applied to the colors (see `Theme::faded`).
        let (bg, fg, border) = if interactive {
            (bg, fg, border)
        } else {
            (theme.faded(bg, 0.5), theme.faded(fg, 0.5), border.map(|b| theme.faded(b, 0.5)))
        };
        let hover = interactive && (hovered.get() || selected());
        let (h, pad) = match size {
            ButtonSize::Default => (9.0, if has_icon { 12.0 } else { 16.0 }),
            ButtonSize::Sm => (8.0, if has_icon { 10.0 } else { 12.0 }),
            ButtonSize::Lg => (10.0, if has_icon { 16.0 } else { 24.0 }),
            ButtonSize::Icon => (9.0, 0.0),
            ButtonSize::IconSm => (8.0, 0.0),
            ButtonSize::IconLg => (10.0, 0.0),
        };
        let height = rsx::rems(h * 0.25);
        let el = el
            .h(height)
            .w(if size.is_icon() { height } else { rsx::auto() })
            .px(px(pad))
            .corner_radius(theme.radius_md())
            .bg(if hover { hover_bg } else { bg })
            .text_color(if hover { accent_fg } else { fg })
            .border_width(px(if border.is_some() { 1.0 } else { 0.0 }))
            .border_color(border.unwrap_or(Color::TRANSPARENT))
            .class(if interactive { "cursor-pointer" } else { "cursor-default" });
        // `shadow-xs` on outline buttons, light mode only (gpuicn's `theme.shadows`).
        let el = if variant == ButtonVariant::Outline && theme.shadows && !theme.is_dark() { el.shadow_xs() } else { el };
        if hover && variant == ButtonVariant::Link { el.class("underline") } else { el }
    };
    let ring = move || {
        let theme = t.get();
        let border = variant_colors(variant, &theme).3;
        (interactive() && is_focus_visible(node))
            .then(|| ring(theme.ring.opacity(0.5), 3.0, theme.radius - 2.0, if border.is_some() { 1.0 } else { 0.0 }))
    };

    // A group's joins, then the caller's attributes, go over the defaults.
    let button = move || <button
        ref={node}
        class="relative flex shrink-0 items-center justify-center gap-2 whitespace-nowrap text-sm leading-[1.25rem] font-medium"
        class:gap-1.5={size == ButtonSize::Sm}
        type="button"
        disabled={move || !interactive()}
        bind={style}
        {...join}
        onMouseDown={move |_| pointer_pressed()}
        onKeyDown={on_tab_key}
        onHover={move |h: &bool| hovered.set(*h)}
        onClick={move |e: &ClickEvent| {
            // Like gpuicn: only a button with its own handler keeps the click
            // from its ancestors (a bare Button inside a Popover trigger opens it).
            if let Some(f) = on_click.filter(|_| interactive()) {
                e.stop_propagation();
                f.call(e)
            }
        }}
        {...attrs}
    >
        {move || if loading() { <Spinner /> } else { icon.map(|i| <Icon name={i} />).into_element() }}
        {label.map(|l| l.get())}
        {children}
        {icon_end.map(|i| <Icon name={i} />)}
        {ring}
    </button>;
    match tooltip {
        // A real `Tooltip`, centered above the button like shadcn's.
        Some(text) => <Tooltip content={text}>{button()}</Tooltip>,
        None => button(),
    }
}

//! # Theme
//!
//! This is the **single file that styles every gpuicn component** — the gpui
//! equivalent of shadcn/ui's `globals.css`.
//!
//! Every color below is written with [`oklch`], exactly like shadcn's CSS
//! variables, so you can paste values straight from <https://ui.shadcn.com/themes>
//! or <https://tweakcn.com>:
//!
//! ```css
//! --primary: oklch(0.205 0 0);            /* CSS  */
//! ```
//! ```ignore
//! primary: oklch(0.205, 0.0, 0.0),        // Rust
//! ```
//!
//! Alpha values (`oklch(1 0 0 / 10%)`) map to [`oklcha`].
//!
//! Install it once at startup with [`init`] (or `ui::init`, which calls it),
//! then read it from any component with `cx.theme()`.

use gpui::{App, Global, Hsla, Pixels, Rgba, SharedString, Window, WindowAppearance, px};
use std::ops::{Deref, DerefMut};

/// Light or dark color scheme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(self) -> bool {
        self == ThemeMode::Dark
    }
}

/// shadcn's base colors: the gray scale behind backgrounds, borders and text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BaseColor {
    #[default]
    Neutral,
    Zinc,
    Slate,
    Stone,
    Gray,
}

impl BaseColor {
    pub const ALL: [BaseColor; 5] = [BaseColor::Neutral, BaseColor::Zinc, BaseColor::Slate, BaseColor::Stone, BaseColor::Gray];

    pub fn name(self) -> &'static str {
        match self {
            BaseColor::Neutral => "Neutral",
            BaseColor::Zinc => "Zinc",
            BaseColor::Slate => "Slate",
            BaseColor::Stone => "Stone",
            BaseColor::Gray => "Gray",
        }
    }
}

/// shadcn's color themes: a colored `primary` (and ring) on top of a base color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AccentColor {
    /// The base color's own near-black / near-white primary.
    #[default]
    Default,
    Blue,
    Green,
    Rose,
    Orange,
    Violet,
    Red,
    Yellow,
}

impl AccentColor {
    pub const ALL: [AccentColor; 8] = [
        AccentColor::Default,
        AccentColor::Blue,
        AccentColor::Green,
        AccentColor::Rose,
        AccentColor::Orange,
        AccentColor::Violet,
        AccentColor::Red,
        AccentColor::Yellow,
    ];

    pub fn name(self) -> &'static str {
        match self {
            AccentColor::Default => "Default",
            AccentColor::Blue => "Blue",
            AccentColor::Green => "Green",
            AccentColor::Rose => "Rose",
            AccentColor::Orange => "Orange",
            AccentColor::Violet => "Violet",
            AccentColor::Red => "Red",
            AccentColor::Yellow => "Yellow",
        }
    }

    /// `(primary, primary_foreground)` for `mode`, or `None` for `Default`.
    pub fn colors(self, mode: ThemeMode) -> Option<(Hsla, Hsla)> {
        let dark = mode.is_dark();
        Some(match self {
            AccentColor::Default => return None,
            AccentColor::Blue if dark => (oklch(0.546, 0.245, 262.881), oklch(0.97, 0.014, 254.604)),
            AccentColor::Blue => (oklch(0.546, 0.245, 262.881), oklch(0.97, 0.014, 254.604)),
            AccentColor::Green if dark => (oklch(0.696, 0.17, 162.48), oklch(0.393, 0.095, 152.535)),
            AccentColor::Green => (oklch(0.723, 0.219, 149.579), oklch(0.982, 0.018, 155.826)),
            AccentColor::Rose => (oklch(0.645, 0.246, 16.439), oklch(0.969, 0.015, 12.422)),
            AccentColor::Orange if dark => (oklch(0.646, 0.222, 41.116), oklch(0.98, 0.016, 73.684)),
            AccentColor::Orange => (oklch(0.705, 0.213, 47.604), oklch(0.98, 0.016, 73.684)),
            AccentColor::Violet if dark => (oklch(0.541, 0.281, 293.009), oklch(0.969, 0.016, 293.756)),
            AccentColor::Violet => (oklch(0.606, 0.25, 292.717), oklch(0.969, 0.016, 293.756)),
            AccentColor::Red => (oklch(0.637, 0.237, 25.331), oklch(0.971, 0.013, 17.38)),
            AccentColor::Yellow => (oklch(0.795, 0.184, 86.047), oklch(0.421, 0.095, 57.708)),
        })
    }
}

/// The design tokens. Field names mirror shadcn's CSS variables
/// (`--card-foreground` → `card_foreground`).
#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,
    pub chart_1: Hsla,
    pub chart_2: Hsla,
    pub chart_3: Hsla,
    pub chart_4: Hsla,
    pub chart_5: Hsla,
    pub sidebar: Hsla,
    pub sidebar_foreground: Hsla,
    pub sidebar_primary: Hsla,
    pub sidebar_primary_foreground: Hsla,
    pub sidebar_accent: Hsla,
    pub sidebar_accent_foreground: Hsla,
    pub sidebar_border: Hsla,
    pub sidebar_ring: Hsla,
    /// Backdrop behind dialogs, sheets and drawers (`bg-black/50`).
    pub overlay: Hsla,
}

impl ThemeColors {
    /// `:root` — shadcn "neutral" base color.
    pub fn light() -> Self {
        Self {
            background: oklch(1.0, 0.0, 0.0),
            foreground: oklch(0.145, 0.0, 0.0),
            card: oklch(1.0, 0.0, 0.0),
            card_foreground: oklch(0.145, 0.0, 0.0),
            popover: oklch(1.0, 0.0, 0.0),
            popover_foreground: oklch(0.145, 0.0, 0.0),
            primary: oklch(0.205, 0.0, 0.0),
            primary_foreground: oklch(0.985, 0.0, 0.0),
            secondary: oklch(0.97, 0.0, 0.0),
            secondary_foreground: oklch(0.205, 0.0, 0.0),
            muted: oklch(0.97, 0.0, 0.0),
            muted_foreground: oklch(0.556, 0.0, 0.0),
            accent: oklch(0.97, 0.0, 0.0),
            accent_foreground: oklch(0.205, 0.0, 0.0),
            destructive: oklch(0.577, 0.245, 27.325),
            destructive_foreground: oklch(0.985, 0.0, 0.0),
            border: oklch(0.922, 0.0, 0.0),
            input: oklch(0.922, 0.0, 0.0),
            ring: oklch(0.708, 0.0, 0.0),
            chart_1: oklch(0.646, 0.222, 41.116),
            chart_2: oklch(0.6, 0.118, 184.704),
            chart_3: oklch(0.398, 0.07, 227.392),
            chart_4: oklch(0.828, 0.189, 84.429),
            chart_5: oklch(0.769, 0.188, 70.08),
            sidebar: oklch(0.985, 0.0, 0.0),
            sidebar_foreground: oklch(0.145, 0.0, 0.0),
            sidebar_primary: oklch(0.205, 0.0, 0.0),
            sidebar_primary_foreground: oklch(0.985, 0.0, 0.0),
            sidebar_accent: oklch(0.97, 0.0, 0.0),
            sidebar_accent_foreground: oklch(0.205, 0.0, 0.0),
            sidebar_border: oklch(0.922, 0.0, 0.0),
            sidebar_ring: oklch(0.708, 0.0, 0.0),
            overlay: oklcha(0.0, 0.0, 0.0, 0.5),
        }
    }

    /// `.dark` — shadcn "neutral" base color.
    pub fn dark() -> Self {
        Self {
            background: oklch(0.145, 0.0, 0.0),
            foreground: oklch(0.985, 0.0, 0.0),
            card: oklch(0.205, 0.0, 0.0),
            card_foreground: oklch(0.985, 0.0, 0.0),
            popover: oklch(0.205, 0.0, 0.0),
            popover_foreground: oklch(0.985, 0.0, 0.0),
            primary: oklch(0.922, 0.0, 0.0),
            primary_foreground: oklch(0.205, 0.0, 0.0),
            secondary: oklch(0.269, 0.0, 0.0),
            secondary_foreground: oklch(0.985, 0.0, 0.0),
            muted: oklch(0.269, 0.0, 0.0),
            muted_foreground: oklch(0.708, 0.0, 0.0),
            accent: oklch(0.269, 0.0, 0.0),
            accent_foreground: oklch(0.985, 0.0, 0.0),
            destructive: oklch(0.704, 0.191, 22.216),
            destructive_foreground: oklch(0.985, 0.0, 0.0),
            border: oklcha(1.0, 0.0, 0.0, 0.10),
            input: oklcha(1.0, 0.0, 0.0, 0.15),
            ring: oklch(0.556, 0.0, 0.0),
            chart_1: oklch(0.488, 0.243, 264.376),
            chart_2: oklch(0.696, 0.17, 162.48),
            chart_3: oklch(0.769, 0.188, 70.08),
            chart_4: oklch(0.627, 0.265, 303.9),
            chart_5: oklch(0.645, 0.246, 16.439),
            sidebar: oklch(0.205, 0.0, 0.0),
            sidebar_foreground: oklch(0.985, 0.0, 0.0),
            sidebar_primary: oklch(0.488, 0.243, 264.376),
            sidebar_primary_foreground: oklch(0.985, 0.0, 0.0),
            sidebar_accent: oklch(0.269, 0.0, 0.0),
            sidebar_accent_foreground: oklch(0.985, 0.0, 0.0),
            sidebar_border: oklcha(1.0, 0.0, 0.0, 0.10),
            sidebar_ring: oklch(0.556, 0.0, 0.0),
            overlay: oklcha(0.0, 0.0, 0.0, 0.5),
        }
    }

    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }

    /// The palette for a base color and accent (shadcn's `--*` variables for
    /// `:root` / `.dark`, e.g. the "zinc" base color with the "blue" theme).
    pub fn build(mode: ThemeMode, base: BaseColor, accent: AccentColor) -> Self {
        let mut colors = Self::for_mode(mode);
        if base != BaseColor::Neutral {
            colors.apply_base(mode, base);
        }
        if let Some((primary, primary_foreground)) = accent.colors(mode) {
            colors.primary = primary;
            colors.primary_foreground = primary_foreground;
            colors.ring = primary;
            colors.sidebar_primary = primary;
            colors.sidebar_primary_foreground = primary_foreground;
            colors.sidebar_ring = primary;
        }
        colors
    }

    /// Swap the neutral grays for another base color's tinted grays.
    fn apply_base(&mut self, mode: ThemeMode, base: BaseColor) {
        // (foreground/darkest, primary-ish, secondary/muted, muted fg, border, ring, secondary dark)
        let (h, darkest, dark2, light2, muted_fg, border, ring, dark3) = match base {
            BaseColor::Neutral => return,
            BaseColor::Zinc => (
                285.9,
                oklch(0.141, 0.005, 285.823),
                oklch(0.21, 0.006, 285.885),
                oklch(0.967, 0.001, 286.375),
                oklch(0.552, 0.016, 285.938),
                oklch(0.92, 0.004, 286.32),
                oklch(0.705, 0.015, 286.067),
                oklch(0.274, 0.006, 286.033),
            ),
            BaseColor::Slate => (
                264.7,
                oklch(0.129, 0.042, 264.695),
                oklch(0.208, 0.042, 265.755),
                oklch(0.968, 0.007, 247.896),
                oklch(0.554, 0.046, 257.417),
                oklch(0.929, 0.013, 255.508),
                oklch(0.704, 0.04, 256.788),
                oklch(0.279, 0.041, 260.031),
            ),
            BaseColor::Stone => (
                49.3,
                oklch(0.147, 0.004, 49.25),
                oklch(0.216, 0.006, 56.043),
                oklch(0.97, 0.001, 106.424),
                oklch(0.553, 0.013, 58.071),
                oklch(0.923, 0.003, 48.717),
                oklch(0.709, 0.01, 56.259),
                oklch(0.268, 0.007, 34.298),
            ),
            BaseColor::Gray => (
                261.7,
                oklch(0.13, 0.028, 261.692),
                oklch(0.21, 0.034, 264.665),
                oklch(0.967, 0.003, 264.542),
                oklch(0.551, 0.027, 264.364),
                oklch(0.928, 0.006, 264.531),
                oklch(0.707, 0.022, 261.325),
                oklch(0.278, 0.033, 256.848),
            ),
        };
        let near_white = oklch(0.985, 0.002, h);
        if mode.is_dark() {
            self.background = darkest;
            self.foreground = near_white;
            self.card = dark2;
            self.card_foreground = near_white;
            self.popover = dark2;
            self.popover_foreground = near_white;
            self.primary = border;
            self.primary_foreground = dark2;
            self.secondary = dark3;
            self.secondary_foreground = near_white;
            self.muted = dark3;
            self.muted_foreground = ring;
            self.accent = dark3;
            self.accent_foreground = near_white;
            self.ring = muted_fg;
            self.sidebar = dark2;
            self.sidebar_foreground = near_white;
            self.sidebar_accent = dark3;
            self.sidebar_accent_foreground = near_white;
            self.sidebar_ring = muted_fg;
        } else {
            self.foreground = darkest;
            self.card_foreground = darkest;
            self.popover_foreground = darkest;
            self.primary = dark2;
            self.primary_foreground = near_white;
            self.secondary = light2;
            self.secondary_foreground = dark2;
            self.muted = light2;
            self.muted_foreground = muted_fg;
            self.accent = light2;
            self.accent_foreground = dark2;
            self.border = border;
            self.input = border;
            self.ring = ring;
            self.sidebar = near_white;
            self.sidebar_foreground = darkest;
            self.sidebar_primary = dark2;
            self.sidebar_primary_foreground = near_white;
            self.sidebar_accent = light2;
            self.sidebar_accent_foreground = dark2;
            self.sidebar_border = border;
            self.sidebar_ring = ring;
        }
    }

    /// The five chart colors as a slice-friendly array.
    pub fn charts(&self) -> [Hsla; 5] {
        [self.chart_1, self.chart_2, self.chart_3, self.chart_4, self.chart_5]
    }
}

/// The active theme, stored as a gpui [`Global`].
#[derive(Clone, Debug)]
pub struct Theme {
    pub mode: ThemeMode,
    /// The gray scale (`neutral`, `zinc`, ...). Kept when switching modes.
    pub base_color: BaseColor,
    /// The colored primary, if any. Kept when switching modes.
    pub accent_color: AccentColor,
    pub colors: ThemeColors,
    /// `--radius`. The other radii are derived from it, like in shadcn:
    /// `sm = radius - 4`, `md = radius - 2`, `lg = radius`, `xl = radius + 4`.
    pub radius: Pixels,
    /// UI font. `.SystemUIFont` resolves to the platform's UI font.
    pub font_family: SharedString,
    /// Font used by `Kbd`, `InlineCode` and other monospaced text.
    pub mono_font_family: SharedString,
    /// Whether popovers, menus and cards cast shadows.
    pub shadows: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }

    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }

    pub fn new(mode: ThemeMode) -> Self {
        Self {
            mode,
            base_color: BaseColor::default(),
            accent_color: AccentColor::default(),
            colors: ThemeColors::for_mode(mode),
            radius: px(10.0), // 0.625rem
            font_family: ".SystemUIFont".into(),
            mono_font_family: default_mono_font().into(),
            shadows: true,
        }
    }

    pub fn radius_sm(&self) -> Pixels {
        (self.radius - px(4.0)).max(px(0.0))
    }

    pub fn radius_md(&self) -> Pixels {
        (self.radius - px(2.0)).max(px(0.0))
    }

    pub fn radius_lg(&self) -> Pixels {
        self.radius
    }

    pub fn radius_xl(&self) -> Pixels {
        self.radius + px(4.0)
    }

    pub fn is_dark(&self) -> bool {
        self.mode.is_dark()
    }

    /// `color` as it looks at `opacity` on top of the page background.
    ///
    /// gpui's element `opacity()` fades each primitive separately (unlike CSS,
    /// which fades the element as a group), so text on a faded fill comes out
    /// washed out twice. Use this to fade a whole control's colors instead.
    pub fn faded(&self, color: Hsla, opacity: f32) -> Hsla {
        if color.a == 0.0 {
            return color;
        }
        mix(self.background, composite(color, self.background), opacity)
    }

    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Theme {
        cx.global_mut::<Theme>()
    }

    /// Recompute the colors from `mode`, `base` and `accent`. Call after
    /// changing `mode`, `base_color` or `accent_color` directly; overrides made to `colors` are reset.
    pub fn rebuild(&mut self) {
        self.colors = ThemeColors::build(self.mode, self.base_color, self.accent_color);
    }

    /// Switch between light and dark, keeping base color, accent, radius and
    /// fonts, and re-render.
    pub fn change(mode: ThemeMode, cx: &mut App) {
        let theme = cx.global_mut::<Theme>();
        theme.mode = mode;
        theme.rebuild();
        cx.refresh_windows();
    }

    /// Use another base color (`Theme::set_base(BaseColor::Zinc, cx)`).
    pub fn set_base(base: BaseColor, cx: &mut App) {
        let theme = cx.global_mut::<Theme>();
        theme.base_color = base;
        theme.rebuild();
        cx.refresh_windows();
    }

    /// Use a colored primary (`Theme::set_accent(AccentColor::Blue, cx)`).
    pub fn set_accent(accent: AccentColor, cx: &mut App) {
        let theme = cx.global_mut::<Theme>();
        theme.accent_color = accent;
        theme.rebuild();
        cx.refresh_windows();
    }

    /// Change `--radius` (the other radii derive from it).
    pub fn set_radius(radius: Pixels, cx: &mut App) {
        cx.global_mut::<Theme>().radius = radius;
        cx.refresh_windows();
    }

    /// Follow the OS light/dark appearance of `window`.
    pub fn sync_system_appearance(window: &Window, cx: &mut App) {
        let mode = match window.appearance() {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => ThemeMode::Dark,
            WindowAppearance::Light | WindowAppearance::VibrantLight => ThemeMode::Light,
        };
        Self::change(mode, cx);
    }
}

impl Deref for Theme {
    type Target = ThemeColors;

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.colors
    }
}

impl Global for Theme {}

/// `cx.theme()` from anywhere you have an `App` (including `Context<T>`).
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

/// Install the default (light) theme unless one is already set.
pub fn init(cx: &mut App) {
    if !cx.has_global::<Theme>() {
        cx.set_global(Theme::default());
    }
}

/// A monospaced family that ships with the OS (gpui doesn't resolve the CSS
/// generic `monospace`).
fn default_mono_font() -> &'static str {
    if cfg!(target_os = "macos") {
        "Menlo"
    } else if cfg!(target_os = "windows") {
        "Consolas"
    } else {
        "DejaVu Sans Mono"
    }
}

/// Linear blend between two colors in sRGB space (`t = 0` → `a`, `t = 1` → `b`).
pub fn mix(a: Hsla, b: Hsla, t: f32) -> Hsla {
    let (a, b) = (a.to_rgb(), b.to_rgb());
    let lerp = |x: f32, y: f32| x + (y - x) * t;
    Rgba { r: lerp(a.r, b.r), g: lerp(a.g, b.g), b: lerp(a.b, b.b), a: lerp(a.a, b.a) }.into()
}

/// `top` painted over an opaque `bottom`, as an opaque color.
pub fn composite(top: Hsla, bottom: Hsla) -> Hsla {
    mix(bottom, top.alpha(1.0), top.a).alpha(1.0)
}

/// Convert a CSS `oklch(l c h)` color to gpui's [`Hsla`].
pub fn oklch(l: f32, c: f32, h: f32) -> Hsla {
    oklcha(l, c, h, 1.0)
}

/// Convert a CSS `oklch(l c h / alpha)` color to gpui's [`Hsla`].
pub fn oklcha(l: f32, c: f32, h: f32, alpha: f32) -> Hsla {
    let h = h.to_radians();
    let (a, b) = (c * h.cos(), c * h.sin());

    let l_ = l + 0.396_337_78 * a + 0.215_803_76 * b;
    let m_ = l - 0.105_561_346 * a - 0.063_854_17 * b;
    let s_ = l - 0.089_484_18 * a - 1.291_485_5 * b;
    let (l3, m3, s3) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);

    let r = 4.076_741_7 * l3 - 3.307_711_6 * m3 + 0.230_969_94 * s3;
    let g = -1.268_438 * l3 + 2.609_757_4 * m3 - 0.341_319_38 * s3;
    let b = -0.004_196_086_3 * l3 - 0.703_418_6 * m3 + 1.707_614_7 * s3;

    fn to_srgb(x: f32) -> f32 {
        let x = if x <= 0.003_130_8 {
            12.92 * x
        } else {
            1.055 * x.powf(1.0 / 2.4) - 0.055
        };
        x.clamp(0.0, 1.0)
    }

    Rgba {
        r: to_srgb(r),
        g: to_srgb(g),
        b: to_srgb(b),
        a: alpha,
    }
    .into()
}

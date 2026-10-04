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

    /// The five chart colors as a slice-friendly array.
    pub fn charts(&self) -> [Hsla; 5] {
        [self.chart_1, self.chart_2, self.chart_3, self.chart_4, self.chart_5]
    }
}

/// The active theme, stored as a gpui [`Global`].
#[derive(Clone, Debug)]
pub struct Theme {
    pub mode: ThemeMode,
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
            colors: ThemeColors::for_mode(mode),
            radius: px(10.0), // 0.625rem
            font_family: ".SystemUIFont".into(),
            mono_font_family: "monospace".into(),
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

    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Theme {
        cx.global_mut::<Theme>()
    }

    /// Switch between light and dark, keeping radius/fonts, and re-render.
    pub fn change(mode: ThemeMode, cx: &mut App) {
        let theme = cx.global_mut::<Theme>();
        theme.mode = mode;
        theme.colors = ThemeColors::for_mode(mode);
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

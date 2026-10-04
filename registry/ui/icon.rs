//! Lucide icons (https://lucide.dev, ISC license), embedded as source so no
//! asset folder is needed.
//!
//! gpui loads SVGs through the app's [`AssetSource`], so register
//! [`IconAssets`] once:
//!
//! ```ignore
//! Application::new().with_assets(ui::icon::IconAssets).run(|cx| { ... });
//! // already have assets? chain them:
//! Application::new().with_assets(ui::icon::IconAssets.with_fallback(MyAssets))
//! ```
//!
//! Add an icon by adding a line to the `icons!` list below with the inner
//! markup of the lucide SVG (everything between `<svg ...>` and `</svg>`).

use gpui::{
    AnyElement, App, AssetSource, IntoElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Svg, Transformation, Window, svg,
};
use std::borrow::Cow;

use super::utils::StyledExt;

macro_rules! icons {
    ($($variant:ident => $name:literal : $body:literal,)*) => {
        /// Every built-in icon. Use with `Icon::new(IconName::Check)` or pass
        /// directly to components that take an icon.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum IconName {
            $($variant,)*
        }

        impl IconName {
            pub const ALL: &'static [IconName] = &[$(IconName::$variant,)*];

            /// The lucide name, e.g. `"chevron-down"`.
            pub fn name(self) -> &'static str {
                match self {
                    $(IconName::$variant => $name,)*
                }
            }

            fn body(self) -> &'static str {
                match self {
                    $(IconName::$variant => $body,)*
                }
            }
        }
    };
}

icons! {
    AlignCenter => "align-center": r#"<path d="M17 12H7"/><path d="M19 18H5"/><path d="M21 6H3"/>"#,
    AlignLeft => "align-left": r#"<path d="M15 12H3"/><path d="M17 18H3"/><path d="M21 6H3"/>"#,
    AlignRight => "align-right": r#"<path d="M21 12H9"/><path d="M21 18H7"/><path d="M21 6H3"/>"#,
    ArrowDown => "arrow-down": r#"<path d="M12 5v14"/><path d="m19 12-7 7-7-7"/>"#,
    ArrowLeft => "arrow-left": r#"<path d="m12 19-7-7 7-7"/><path d="M19 12H5"/>"#,
    ArrowRight => "arrow-right": r#"<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>"#,
    ArrowUp => "arrow-up": r#"<path d="m5 12 7-7 7 7"/><path d="M12 19V5"/>"#,
    ArrowUpDown => "arrow-up-down": r#"<path d="m21 16-4 4-4-4"/><path d="M17 20V4"/><path d="m3 8 4-4 4 4"/><path d="M7 4v16"/>"#,
    Bell => "bell": r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"/>"#,
    Bold => "bold": r#"<path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"/>"#,
    Calendar => "calendar": r#"<path d="M8 2v4"/><path d="M16 2v4"/><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M3 10h18"/>"#,
    Check => "check": r#"<path d="M20 6 9 17l-5-5"/>"#,
    ChevronDown => "chevron-down": r#"<path d="m6 9 6 6 6-6"/>"#,
    ChevronLeft => "chevron-left": r#"<path d="m15 18-6-6 6-6"/>"#,
    ChevronRight => "chevron-right": r#"<path d="m9 18 6-6-6-6"/>"#,
    ChevronUp => "chevron-up": r#"<path d="m18 15-6-6-6 6"/>"#,
    ChevronsUpDown => "chevrons-up-down": r#"<path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>"#,
    Circle => "circle": r#"<circle cx="12" cy="12" r="10"/>"#,
    CircleAlert => "circle-alert": r#"<circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>"#,
    CircleCheck => "circle-check": r#"<circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/>"#,
    CircleX => "circle-x": r#"<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/>"#,
    Copy => "copy": r#"<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#,
    CreditCard => "credit-card": r#"<rect width="20" height="14" x="2" y="5" rx="2"/><line x1="2" x2="22" y1="10" y2="10"/>"#,
    Dot => "dot": r#"<circle cx="12.1" cy="12.1" r="1"/>"#,
    Ellipsis => "ellipsis": r#"<circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/>"#,
    Eye => "eye": r#"<path d="M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0"/><circle cx="12" cy="12" r="3"/>"#,
    EyeOff => "eye-off": r#"<path d="M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49"/><path d="M14.084 14.158a3 3 0 0 1-4.242-4.242"/><path d="M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143"/><path d="m2 2 20 20"/>"#,
    File => "file": r#"<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/>"#,
    Folder => "folder": r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#,
    GripVertical => "grip-vertical": r#"<circle cx="9" cy="12" r="1"/><circle cx="9" cy="5" r="1"/><circle cx="9" cy="19" r="1"/><circle cx="15" cy="12" r="1"/><circle cx="15" cy="5" r="1"/><circle cx="15" cy="19" r="1"/>"#,
    Heart => "heart": r#"<path d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"/>"#,
    House => "house": r#"<path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>"#,
    Inbox => "inbox": r#"<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>"#,
    Info => "info": r#"<circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/>"#,
    Italic => "italic": r#"<line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/>"#,
    LoaderCircle => "loader-circle": r#"<path d="M21 12a9 9 0 1 1-6.219-8.56"/>"#,
    LogOut => "log-out": r#"<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" x2="9" y1="12" y2="12"/>"#,
    Mail => "mail": r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>"#,
    Menu => "menu": r#"<line x1="4" x2="20" y1="12" y2="12"/><line x1="4" x2="20" y1="6" y2="6"/><line x1="4" x2="20" y1="18" y2="18"/>"#,
    Minus => "minus": r#"<path d="M5 12h14"/>"#,
    Moon => "moon": r#"<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>"#,
    PanelLeft => "panel-left": r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/>"#,
    Plus => "plus": r#"<path d="M5 12h14"/><path d="M12 5v14"/>"#,
    Search => "search": r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#,
    Settings => "settings": r#"<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>"#,
    Star => "star": r#"<polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>"#,
    Sun => "sun": r#"<circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>"#,
    Terminal => "terminal": r#"<polyline points="4 17 10 11 4 5"/><line x1="12" x2="20" y1="19" y2="19"/>"#,
    Trash => "trash": r#"<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>"#,
    TriangleAlert => "triangle-alert": r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/>"#,
    Underline => "underline": r#"<path d="M6 4v6a6 6 0 0 0 12 0V4"/><line x1="4" x2="20" y1="20" y2="20"/>"#,
    User => "user": r#"<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>"#,
    X => "x": r#"<path d="M18 6 6 18"/><path d="m6 6 12 12"/>"#,
}

const PREFIX: &str = "icons/";

impl IconName {
    /// The asset path this icon is served from by [`IconAssets`].
    pub fn path(self) -> SharedString {
        format!("{PREFIX}{}.svg", self.name()).into()
    }

    /// Full SVG document for this icon.
    pub fn svg(self) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{}</svg>"#,
            self.body()
        )
    }

    pub fn from_path(path: &str) -> Option<Self> {
        let name = path.strip_prefix(PREFIX)?.strip_suffix(".svg")?;
        Self::ALL.iter().copied().find(|icon| icon.name() == name)
    }
}

/// Serves every [`IconName`] at `icons/<name>.svg`.
#[derive(Clone, Copy, Debug, Default)]
pub struct IconAssets;

impl IconAssets {
    /// Serve icons first, then fall back to your own asset source.
    pub fn with_fallback<A: AssetSource>(self, fallback: A) -> ChainedAssets<A> {
        ChainedAssets(fallback)
    }
}

impl AssetSource for IconAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(IconName::from_path(path).map(|icon| Cow::Owned(icon.svg().into_bytes())))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(IconName::ALL
            .iter()
            .map(|icon| icon.path())
            .filter(|p| p.starts_with(path))
            .collect())
    }
}

/// [`IconAssets`] followed by another [`AssetSource`].
pub struct ChainedAssets<A>(A);

impl<A: AssetSource> AssetSource for ChainedAssets<A> {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        match IconAssets.load(path)? {
            Some(bytes) => Ok(Some(bytes)),
            None => self.0.load(path),
        }
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        let mut list = IconAssets.list(path)?;
        list.extend(self.0.list(path)?);
        Ok(list)
    }
}

/// An SVG icon. Defaults to `size-4` and the surrounding text color.
#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    rotation: Option<f32>,
    style: StyleRefinement,
}

impl Icon {
    pub fn new(icon: impl Into<Icon>) -> Self {
        icon.into()
    }

    /// Any SVG served by your app's asset source.
    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            rotation: None,
            style: StyleRefinement::default(),
        }
    }

    /// Rotate the icon (in radians), e.g. for a spinning loader.
    pub fn rotate(mut self, radians: f32) -> Self {
        self.rotation = Some(radians);
        self
    }
}

impl From<IconName> for Icon {
    fn from(icon: IconName) -> Self {
        Icon::from_path(icon.path())
    }
}

impl From<IconName> for AnyElement {
    fn from(icon: IconName) -> Self {
        Icon::new(icon).into_any_element()
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = window.text_style().color;
        let svg: Svg = svg()
            .flex_none()
            .size_4()
            .text_color(color)
            .refine_style(&self.style)
            .path(self.path);
        match self.rotation {
            Some(radians) => svg.with_transformation(Transformation::rotate(gpui::radians(radians))),
            None => svg,
        }
    }
}

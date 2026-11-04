//! Component gallery: every registry component, rendered straight from
//! `registry/ui` so what you see is exactly what `gpuicn add` installs.

#[path = "../../../registry/ui/mod.rs"]
mod ui;
mod demos;

use gpui::{
    App, Application, Bounds, Context, FontWeight, SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px, relative,
    rems, size,
};
use ui::{
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonSize},
    icon::{Icon, IconName},
    popover::Popover,
    separator::Separator,
    portal::PortalHost,
    scroll_area::ScrollArea,
    sidebar::{Sidebar, SidebarCollapsible, SidebarGroup, SidebarInset, SidebarMenuItem},
    sonner::Toaster,
    theme::{AccentColor, ActiveTheme, BaseColor, Theme, ThemeColors, ThemeMode},
    utils::Align,
};

pub struct Gallery {
    page: usize,
}

impl Gallery {
    pub fn new() -> Self {
        Self { page: 0 }
    }

    pub fn select(&mut self, page: usize, cx: &mut Context<Self>) {
        self.page = page.min(demos::DEMOS.len() - 1);
        cx.notify();
    }
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (name, demo) = demos::DEMOS[self.page];
        let dark = cx.theme().mode.is_dark();
        let count = demos::DEMOS.len();

        let view = cx.entity().downgrade();
        let items = demos::DEMOS.iter().enumerate().map(|(ix, (label, _))| {
            let view = view.clone();
            SidebarMenuItem::new(*label).active(ix == self.page).on_click(move |_, _, cx| {
                view.update(cx, |this, cx| this.select(ix, cx)).ok();
            })
        });

        let content = demo(window, cx);
        let theme = cx.theme();

        let brand = div()
            .flex()
            .items_center()
            .gap_2()
            .h(px(40.))
            .px_2()
            .child(
                div()
                    .flex()
                    .flex_none()
                    .size_7()
                    .items_center()
                    .justify_center()
                    .rounded(theme.radius_md())
                    .bg(theme.primary)
                    .text_color(theme.primary_foreground)
                    .child(Icon::new(IconName::Terminal).size_4()),
            )
            .child(div().font_weight(FontWeight::SEMIBOLD).child("gpuicn"))
            .child(div().text_xs().text_color(theme.muted_foreground).child(concat!("v", env!("CARGO_PKG_VERSION"))));
        let sidebar = Sidebar::new("gallery-nav")
            .collapsible(SidebarCollapsible::None)
            .header(brand)
            .group(SidebarGroup::new().label(format!("Components · {count}")).items(items));

        let nav = |id: &'static str, icon: IconName, tooltip: &'static str, target: usize| {
            let view = view.clone();
            Button::new(id).ghost().size(ButtonSize::Icon).icon(icon).tooltip(tooltip).on_click(move |_, _, cx| {
                view.update(cx, |this, cx| this.select(target, cx)).ok();
            })
        };
        let header = div()
            .flex()
            .flex_none()
            .h(px(56.))
            .items_center()
            .justify_between()
            .gap_4()
            .px_6()
            .border_b_1()
            .border_color(theme.border)
            .child(Breadcrumb::new().item(BreadcrumbItem::new("Components")).item(BreadcrumbItem::new(name).current()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(nav("prev", IconName::ChevronLeft, "Previous component", (self.page + count - 1) % count))
                    .child(nav("next", IconName::ChevronRight, "Next component", (self.page + 1) % count))
                    .child(Separator::vertical().h_4().mx_1())
                    .child(customizer())
                    .child(
                        Button::new("theme-toggle")
                            .ghost()
                            .size(ButtonSize::Icon)
                            .icon(if dark { IconName::Sun } else { IconName::Moon })
                            .tooltip("Toggle theme")
                            .on_click(move |_, _, cx| {
                                Theme::change(if dark { ThemeMode::Light } else { ThemeMode::Dark }, cx);
                            }),
                    ),
            );

        let title = div()
            .flex()
            .flex_none()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_3xl()
                    .line_height(rems(2.25))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(name),
            )
            .child(
                div()
                    .text_base()
                    .line_height(rems(1.5))
                    .text_color(theme.muted_foreground)
                    .child(demos::description(name)),
            );
        let preview = div()
            .flex()
            .flex_none()
            .min_h(px(440.))
            .items_center()
            .justify_center()
            .p_10()
            .rounded(theme.radius_xl())
            .border_1()
            .border_color(theme.border)
            .child(content);

        div()
            .flex()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .font_family(theme.font_family.clone())
            .text_sm()
            // Tailwind-like leading for text that doesn't set its own.
            .line_height(relative(1.43))
            .child(sidebar)
            .child(
                SidebarInset::new().child(header).child(
                    ScrollArea::new("gallery-content").flex_1().min_h_0().child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_8()
                            .w_full()
                            .max_w(px(1100.))
                            .px_8()
                            .pt_8()
                            .pb_12()
                            .child(title)
                            .child(preview),
                    ),
                ),
            )
            .child(Toaster::new())
            .child(PortalHost::new())
    }
}

/// The theme customizer popover: accent color, base color, radius and mode,
/// all driven through `Theme`'s setters.
fn customizer() -> impl IntoElement {
    Popover::new("customizer")
        .align(Align::End)
        .w(px(340.))
        .trigger(Button::new("customize").outline().size(ButtonSize::Sm).icon(IconName::Palette).label("Customize"))
        .content(|_, cx| {
            let theme = cx.theme();
            let (accent, base, radius, mode) = (theme.accent_color, theme.base_color, theme.radius, theme.mode);
            let muted = theme.muted_foreground;
            let section = |title: &'static str| {
                div().text_xs().line_height(rems(1.)).font_weight(FontWeight::MEDIUM).text_color(muted).child(title)
            };
            let grid = || div().grid().grid_cols(3).gap_2();
            // One selected treatment for every group: primary border + tint.
            let primary = theme.primary;
            let mark = move |button: Button, selected: bool| {
                button.when(selected, |this| this.border_color(primary).bg(primary.opacity(0.08)))
            };

            let accents = AccentColor::ALL.map(|a| {
                let swatch = ThemeColors::build(mode, base, a).primary;
                mark(Button::new(SharedString::from(format!("accent-{}", a.name()))).outline(), a == accent)
                    .size(ButtonSize::Sm)
                    .justify_start()
                    .child(div().size_3p5().flex_none().rounded_full().bg(swatch))
                    .child(a.name())
                    .on_click(move |_, _, cx| Theme::set_accent(a, cx))
            });
            let bases = BaseColor::ALL.map(|b| {
                let swatch = ThemeColors::build(ThemeMode::Light, b, AccentColor::Default).muted_foreground;
                mark(Button::new(SharedString::from(format!("base-{}", b.name()))).outline(), b == base)
                    .size(ButtonSize::Sm)
                    .justify_start()
                    .child(div().size_3p5().flex_none().rounded_full().bg(swatch))
                    .child(b.name())
                    .on_click(move |_, _, cx| Theme::set_base(b, cx))
            });
            let radii = [("0", 0.), ("0.3", 4.8), ("0.5", 8.), ("0.625", 10.), ("0.75", 12.), ("1.0", 16.)].map(|(label, value)| {
                mark(Button::new(SharedString::from(format!("radius-{label}"))).outline(), radius == px(value))
                    .size(ButtonSize::Sm)
                    .label(label)
                    .on_click(move |_, _, cx| Theme::set_radius(px(value), cx))
            });
            let modes = [(ThemeMode::Light, "Light", IconName::Sun), (ThemeMode::Dark, "Dark", IconName::Moon)].map(|(m, label, icon)| {
                mark(Button::new(SharedString::from(format!("mode-{label}"))).outline(), m == mode)
                    .size(ButtonSize::Sm)
                    .icon(icon)
                    .label(label)
                    .on_click(move |_, _, cx| Theme::change(m, cx))
            });

            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(div().font_weight(FontWeight::SEMIBOLD).line_height(rems(1.25)).child("Customize"))
                        .child(div().text_color(muted).line_height(rems(1.25)).child("Pick a color, base and radius for every component.")),
                )
                .child(div().flex().flex_col().gap_2().child(section("Color")).child(grid().children(accents)))
                .child(div().flex().flex_col().gap_2().child(section("Base color")).child(grid().children(bases)))
                .child(div().flex().flex_col().gap_2().child(section("Radius")).child(grid().children(radii)))
                .child(div().flex().flex_col().gap_2().child(section("Mode")).child(grid().children(modes)))
        })
}

/// `GALLERY_PAGE` (a demo name or index), `GALLERY_THEME` (`light`/`dark`)
/// and `GALLERY_SIZE` (`1280x800`) pick the initial state, which makes the
/// gallery scriptable for screenshots.
fn initial_page() -> usize {
    let Ok(page) = std::env::var("GALLERY_PAGE") else { return 0 };
    page.parse().ok().unwrap_or_else(|| {
        let wanted = page.to_lowercase().replace(['-', '_'], " ");
        demos::DEMOS.iter().position(|(name, _)| name.to_lowercase() == wanted).unwrap_or(0)
    })
}

/// The gallery ships Geist (shadcn's default typeface) so it looks the same on
/// every platform. Your app can do the same, or keep `.SystemUIFont`.
fn load_fonts(cx: &mut App) {
    use std::borrow::Cow;
    let fonts: Vec<Cow<'static, [u8]>> = vec![
        Cow::Borrowed(include_bytes!("../fonts/Geist-Regular.ttf")),
        Cow::Borrowed(include_bytes!("../fonts/Geist-Medium.ttf")),
        Cow::Borrowed(include_bytes!("../fonts/Geist-SemiBold.ttf")),
        Cow::Borrowed(include_bytes!("../fonts/Geist-Bold.ttf")),
        Cow::Borrowed(include_bytes!("../fonts/Geist-Italic.ttf")),
        Cow::Borrowed(include_bytes!("../fonts/Geist-MediumItalic.ttf")),
        Cow::Borrowed(include_bytes!("../fonts/GeistMono-Regular.ttf")),
        Cow::Borrowed(include_bytes!("../fonts/GeistMono-Medium.ttf")),
    ];
    if cx.text_system().add_fonts(fonts).is_ok() {
        let theme = Theme::global_mut(cx);
        theme.font_family = "Geist".into();
        theme.mono_font_family = "Geist Mono".into();
    }
}

fn main() {
    Application::new().with_assets(ui::icon::IconAssets).run(|cx: &mut App| {
        ui::init(cx);
        load_fonts(cx);
        if std::env::var("GALLERY_THEME").is_ok_and(|t| t == "dark") {
            Theme::change(ThemeMode::Dark, cx);
        }
        if let Ok(name) = std::env::var("GALLERY_ACCENT")
            && let Some(accent) = AccentColor::ALL.into_iter().find(|a| a.name().eq_ignore_ascii_case(&name))
        {
            Theme::set_accent(accent, cx);
        }
        if let Ok(name) = std::env::var("GALLERY_BASE")
            && let Some(base) = BaseColor::ALL.into_iter().find(|b| b.name().eq_ignore_ascii_case(&name))
        {
            Theme::set_base(base, cx);
        }
        let (w, h) = std::env::var("GALLERY_SIZE")
            .ok()
            .and_then(|s| s.split_once('x').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))))
            .unwrap_or((1200., 800.));
        let bounds = Bounds::centered(None, size(px(w), px(h)), cx);
        let page = initial_page();
        cx.open_window(
            WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), ..Default::default() },
            |_, cx| cx.new(|_| Gallery { page }),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests;

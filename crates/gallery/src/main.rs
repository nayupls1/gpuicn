//! Component gallery: every registry component, rendered straight from
//! `registry/ui` so what you see is exactly what `gpuicn add` installs.

#[path = "../../../registry/ui/mod.rs"]
mod ui;
mod demos;

use gpui::{
    App, Application, Bounds, Context, FontWeight, Window, WindowBounds, WindowOptions, div, prelude::*, px, relative,
    rems, size,
};
use ui::{
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonSize},
    icon::{Icon, IconName},
    separator::Separator,
    portal::PortalHost,
    scroll_area::ScrollArea,
    sidebar::{Sidebar, SidebarCollapsible, SidebarGroup, SidebarInset, SidebarMenuItem},
    sonner::Toaster,
    theme::{ActiveTheme, Theme, ThemeMode},
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

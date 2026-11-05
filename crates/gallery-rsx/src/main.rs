//! Component gallery: every registry-rsx component, rendered straight from
//! `registry-rsx/ui` so what you see is exactly what `gpuicn-rsx add` installs.
//!
//! `GALLERY_PAGE=select GALLERY_THEME=dark GALLERY_ACCENT=blue GALLERY_BASE=zinc`
//! opens it in a given state; `--html` prints the page as HTML instead of
//! opening a window (handy for screenshots in a browser).

#[path = "../../../registry-rsx/ui/mod.rs"]
mod ui;
mod demos;
rsx::module!(app);
rsx::module!(tests);
rsx::module!(tests_data);

fn main() {
    if std::env::args().any(|a| a == "--html") {
        println!("{}", rsx_html::render_document(app::App::new));
        return;
    }
    run();
}

/// A GPUI app hosting the rsx gallery. It isn't `rsx_gpui::run` only because
/// the gallery bundles Geist, shadcn's typeface, and rsx can neither register
/// fonts nor set a font family yet (rsx#8): fonts are added here and the
/// family is set on a GPUI wrapper, whose text style the rsx tree inherits.
fn run() {
    use gpui::{prelude::*, px, size, App, Application, Bounds, WindowBounds, WindowOptions};

    struct Root {
        host: gpui::Entity<rsx_gpui::Host>,
        font: gpui::SharedString,
    }
    impl gpui::Render for Root {
        fn render(&mut self, _: &mut gpui::Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
            gpui::div().size_full().font_family(self.font.clone()).line_height(gpui::relative(1.43)).child(self.host.clone())
        }
    }

    Application::new().run(|cx: &mut App| {
        let font = load_fonts(cx);
        let (w, h) = std::env::var("GALLERY_SIZE")
            .ok()
            .and_then(|s| s.split_once('x').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))))
            .unwrap_or((1200., 800.));
        let bounds = Bounds::centered(None, size(px(w), px(h)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions { title: Some("gpuicn-rsx gallery".into()), ..Default::default() }),
                ..Default::default()
            },
            |window, cx| {
                let host = cx.new(|cx| rsx_gpui::Host::new(app::App::new, window, cx));
                cx.new(|_| Root { host, font })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

/// The gallery ships Geist (shadcn's default typeface) so it looks the same
/// on every platform; returns the family to use.
fn load_fonts(cx: &mut gpui::App) -> gpui::SharedString {
    use std::borrow::Cow;
    let fonts: Vec<Cow<'static, [u8]>> = vec![
        Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Regular.ttf")),
        Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Medium.ttf")),
        Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-SemiBold.ttf")),
        Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Bold.ttf")),
        Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Italic.ttf")),
        Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-MediumItalic.ttf")),
        Cow::Borrowed(include_bytes!("../../gallery/fonts/GeistMono-Regular.ttf")),
        Cow::Borrowed(include_bytes!("../../gallery/fonts/GeistMono-Medium.ttf")),
    ];
    if cx.text_system().add_fonts(fonts).is_ok() {
        ui::theme::Theme::update(|t| {
            t.font_family = "Geist".into();
            t.mono_font_family = "Geist Mono".into();
        });
        "Geist".into()
    } else {
        ".SystemUIFont".into()
    }
}

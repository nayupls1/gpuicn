//! Component gallery: every registry-rsx component, rendered straight from
//! `registry-rsx/ui` so what you see is exactly what `gpuicn-rsx add` installs.
//!
//! `GALLERY_PAGE=select GALLERY_THEME=dark GALLERY_ACCENT=blue GALLERY_BASE=zinc`
//! opens it in a given state (`GALLERY_SIZE=1280x800` sizes the window);
//! `--html` prints the page as HTML instead of opening a window (handy for
//! screenshots in a browser).

#[path = "../../../registry-rsx/ui/mod.rs"]
mod ui;
mod demos;
rsx::module!(app);
rsx::module!(tests);
rsx::module!(tests_data);
rsx::module!(tests_forms);
rsx::module!(tests_integrate);
rsx::module!(tests_layout);
rsx::module!(tests_menus);
rsx::module!(tests_overlays);
rsx::module!(tests_search);
rsx::module!(tests_toggles);

use std::borrow::Cow;

fn main() {
    if std::env::args().any(|a| a == "--html") {
        demos::inline_images(true);
        println!("{}", rsx_html::render_document(app::App::new));
        return;
    }
    let (w, h) = std::env::var("GALLERY_SIZE")
        .ok()
        .and_then(|s| s.split_once('x').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))))
        .unwrap_or((1200., 800.));
    // The gallery ships Geist (shadcn's default typeface) so it looks the same
    // on every platform; the theme's font families put it on every component.
    ui::theme::Theme::update(|t| {
        t.font_family = "Geist, sans-serif".into();
        t.mono_font_family = "Geist Mono, monospace".into();
    });
    rsx_gpui::App::new()
        .fonts(vec![
            Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Regular.ttf")),
            Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Medium.ttf")),
            Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-SemiBold.ttf")),
            Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Bold.ttf")),
            Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-Italic.ttf")),
            Cow::Borrowed(include_bytes!("../../gallery/fonts/Geist-MediumItalic.ttf")),
            Cow::Borrowed(include_bytes!("../../gallery/fonts/GeistMono-Regular.ttf")),
            Cow::Borrowed(include_bytes!("../../gallery/fonts/GeistMono-Medium.ttf")),
        ])
        .assets(Assets)
        .run(rsx::WindowOptions::new("gpuicn-rsx gallery").size(w, h), app::App::new);
}

/// Serves the bundled avatar photos (`avatar-1.png`…, see
/// `demos::avatar_image`) to `<img src>`, so packaged builds have them.
struct Assets;

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::avatar(path).map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        Ok((1..=demos::AVATARS.len()).map(|n| format!("avatar-{n}.png")).filter(|p| p.starts_with(path)).map(Into::into).collect())
    }
}

impl Assets {
    fn avatar(path: &str) -> Option<&'static [u8]> {
        let n: usize = path.strip_prefix("avatar-")?.strip_suffix(".png")?.parse().ok()?;
        demos::AVATARS.get(n.checked_sub(1)?).copied()
    }
}

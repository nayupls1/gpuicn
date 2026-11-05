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

fn main() {
    if std::env::args().any(|a| a == "--html") {
        println!("{}", rsx_html::render_document(app::App::new));
        return;
    }
    rsx_gpui::run(rsx::WindowOptions::new("gpuicn-rsx gallery").size(1200.0, 800.0), app::App::new);
}

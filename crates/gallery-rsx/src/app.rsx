//! The gallery shell: a sidebar listing every page, and the current page.

use rsx::prelude::*;

use crate::demos::{self, DEMOS};
use crate::ui::theme::{theme, AccentColor, BaseColor, Theme, ThemeMode};

fn env_page() -> usize {
    let wanted = std::env::var("GALLERY_PAGE").unwrap_or_default().to_lowercase().replace(['-', '_', ' '], "");
    DEMOS.iter().position(|(name, _)| name.to_lowercase().replace(' ', "") == wanted).unwrap_or(0)
}

fn env_theme() {
    let var = |k: &str| std::env::var(k).unwrap_or_default().to_lowercase();
    if var("GALLERY_THEME") == "dark" {
        Theme::change(ThemeMode::Dark);
    }
    if let Some(base) = BaseColor::ALL.iter().find(|b| b.name().to_lowercase() == var("GALLERY_BASE")) {
        Theme::set_base(*base);
    }
    if let Some(accent) = AccentColor::ALL.iter().find(|a| a.name().to_lowercase() == var("GALLERY_ACCENT")) {
        Theme::set_accent(*accent);
    }
}

#[component]
pub fn App() -> impl IntoElement {
    crate::ui::init();
    env_theme();
    let t = theme();
    let page = signal(env_page());

    <div class="size-full flex text-sm" bg={t.get().background} text_color={t.get().foreground}>
        <div class="flex flex-col w-[220px] p-2 gap-1 border-r overflow-y-scroll" border_color={t.get().border} bg={t.get().sidebar}>
            {DEMOS.iter().enumerate().map(|(ix, (name, _))| <div
                class="px-2 py-1.5 rounded-md cursor-pointer"
                bg={if page.get() == ix { t.get().sidebar_accent } else { Color::TRANSPARENT }}
                onClick={move |_| page.set(ix)}
            >{*name}</div>).collect::<Vec<_>>()}
        </div>
        <div class="flex flex-col flex-1 p-10 gap-6 overflow-y-scroll">
            {move || {
                let (name, demo) = DEMOS[page.get()];
                <div class="flex flex-col gap-6">
                    <div class="flex flex-col gap-1">
                        <div class="text-3xl font-semibold">{name}</div>
                        <div text_color={t.get().muted_foreground}>{demos::description(name)}</div>
                    </div>
                    {demo()}
                </div>
            }}
        </div>
    </div>
}

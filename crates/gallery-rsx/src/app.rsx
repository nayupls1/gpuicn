//! The gallery shell: a sidebar listing every page, a header (breadcrumb,
//! previous/next, the theme customizer, light/dark), and the current page in
//! a preview box, like `crates/gallery/src/main.rs`.

use rsx::prelude::*;

use crate::demos::{self, DEMOS};
use crate::ui::breadcrumb::{Breadcrumb, BreadcrumbItem};
use crate::ui::button::{Button, ButtonSize};
use crate::ui::icon::{Icon, IconName};
use crate::ui::popover::Popover;
use crate::ui::portal::PortalHost;
use crate::ui::scroll_area::ScrollArea;
use crate::ui::separator::Separator;
use crate::ui::sidebar::{Sidebar, SidebarCollapsible, SidebarGroup, SidebarInset, SidebarMenuItem};
use crate::ui::sonner::Toaster;
use crate::ui::theme::{theme, AccentColor, BaseColor, Theme, ThemeColors, ThemeMode};
use crate::ui::utils::Align;

/// `GALLERY_PAGE` (a page name or index) picks the first page.
fn env_page() -> usize {
    let Ok(page) = std::env::var("GALLERY_PAGE") else { return 0 };
    page.parse().ok().filter(|&ix| ix < DEMOS.len()).unwrap_or_else(|| {
        let wanted = page.to_lowercase().replace(['-', '_', ' '], "");
        DEMOS.iter().position(|(name, _)| name.to_lowercase().replace(' ', "") == wanted).unwrap_or(0)
    })
}

/// `GALLERY_THEME` (`light`/`dark`), `GALLERY_ACCENT` and `GALLERY_BASE`.
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
pub fn App(
    /// The current page (tests drive it); defaults to `GALLERY_PAGE`.
    page: Option<Signal<usize>>,
) -> impl IntoElement {
    crate::ui::init();
    env_theme();
    let t = theme();
    let page = page.unwrap_or_else(|| signal(env_page()));
    let count = DEMOS.len();
    let select = move |ix: usize| page.set(ix.min(count - 1));

    let brand = move || {
        let theme = t.get();
        <div class="flex items-center gap-2 h-[40px] px-2">
            <div
                class="flex flex-none size-7 items-center justify-center"
                corner_radius={theme.radius_md()}
                bg={theme.primary}
                text_color={theme.primary_foreground}
            >
                <Icon name={IconName::Terminal} class="size-4" />
            </div>
            <div class="font-semibold">gpuicn-rsx</div>
            <div class="text-xs" text_color={theme.muted_foreground}>{concat!("v", env!("CARGO_PKG_VERSION"))}</div>
        </div>
    };

    let nav = move |icon: IconName, tooltip: &'static str, step: usize| <Button
        ghost
        size={ButtonSize::Icon}
        icon={icon}
        tooltip={tooltip.to_string()}
        onClick={move |_| select((page.get_untracked() + step) % count)}
    />;

    let header = move || <div
        class="flex flex-none h-[56px] items-center justify-between gap-4 px-6 border-b"
        border_color={t.get().border}
    >
        <Breadcrumb>
            <BreadcrumbItem label="Components" />
            <BreadcrumbItem current label={DEMOS[page.get()].0} />
        </Breadcrumb>
        <div class="flex items-center gap-1">
            {nav(IconName::ChevronLeft, "Previous component", count - 1)}
            {nav(IconName::ChevronRight, "Next component", 1)}
            <Separator vertical class="h-4 mx-1" />
            {customizer()}
            {move || {
                let dark = t.get().is_dark();
                <Button
                    ghost
                    size={ButtonSize::Icon}
                    icon={if dark { IconName::Sun } else { IconName::Moon }}
                    tooltip={"Toggle theme".to_string()}
                    onClick={move |_| Theme::change(if dark { ThemeMode::Light } else { ThemeMode::Dark })}
                />
            }}
        </div>
    </div>;

    let content = move || {
        let (name, demo) = DEMOS[page.get()];
        let theme = t.get_untracked();
        <div class="flex flex-col gap-8 w-full max-w-[1100px] px-8 pt-8 pb-12">
            <div class="flex flex-none flex-col gap-2">
                <div class="text-3xl leading-[2.25rem] font-semibold">{name}</div>
                <div class="text-base leading-[1.5rem]" text_color={t.get().muted_foreground}>{demos::description(name)}</div>
            </div>
            <div
                class="flex flex-none min-h-[440px] items-center justify-center p-10 border"
                corner_radius={theme.radius_xl()}
                border_color={t.get().border}
            >
                {demo()}
            </div>
        </div>
    };

    <div class="flex size-full text-sm" bg={t.get().background} text_color={t.get().foreground}>
        <Sidebar id="gallery-nav" collapsible={SidebarCollapsible::None} header={brand}>
            <SidebarGroup label={format!("Components · {count}")}>
                {DEMOS
                    .iter()
                    .enumerate()
                    .map(|(ix, (label, _))| <SidebarMenuItem label={*label} active={page.get() == ix} onClick={move |_| select(ix)} />)
                    .collect::<Vec<_>>()}
            </SidebarGroup>
        </Sidebar>
        <SidebarInset>
            {header}
            <ScrollArea class="flex-1 min-h-0">{content}</ScrollArea>
        </SidebarInset>
        <Toaster />
        <PortalHost />
    </div>
}

/// The theme customizer popover: accent color, base color, radius and mode,
/// all driven through `Theme`'s setters.
fn customizer() -> Element {
    let t = theme();
    <Popover
        align={Align::End}
        class="w-[340px]"
        trigger={<Button outline size={ButtonSize::Sm} icon={IconName::Palette} label="Customize" />}
        content={move |_| {
            let section = move |title: &'static str| <div class="text-xs leading-[1rem] font-medium" text_color={t.get().muted_foreground}>{title}</div>;
            // One selected treatment for every group: a primary border.
            let mark = move |selected: bool| {
                rsx::Attributes::new().with(move |el: rsx::Div| if selected { el.class("border-2").border_color(t.get().primary) } else { el })
            };
            let swatch = |color: Color| <div class="size-3.5 flex-none rounded-full" bg={color} />;
            // rsx#19: no CSS grid, so rows of equal-width cells.
            let grid = |cells: Vec<Element>, columns: usize| {
                let mut rows = Vec::new();
                let mut cells = cells.into_iter().peekable();
                while cells.peek().is_some() {
                    let mut row: Vec<Element> = cells.by_ref().take(columns).map(|c| <div class="flex flex-1 min-w-0">{c}</div>).collect();
                    while row.len() < columns {
                        row.push(<div class="flex-1" />);
                    }
                    rows.push(<div class="flex gap-2">{row}</div>);
                }
                <div class="flex flex-col gap-2">{rows}</div>
            };
            move || {
                let theme = t.get();
                let (accent, base, radius, mode) = (theme.accent_color, theme.base_color, theme.radius, theme.mode);
                let accents = AccentColor::ALL
                    .iter()
                    .map(|&a| <Button outline size={ButtonSize::Sm} class="w-full justify-start" onClick={move |_| Theme::set_accent(a)} {...mark(a == accent)}>
                        {swatch(ThemeColors::build(mode, base, a).primary)}
                        {a.name()}
                    </Button>)
                    .collect();
                let bases = BaseColor::ALL
                    .iter()
                    .map(|&b| <Button outline size={ButtonSize::Sm} class="w-full justify-start" onClick={move |_| Theme::set_base(b)} {...mark(b == base)}>
                        {swatch(ThemeColors::build(ThemeMode::Light, b, AccentColor::Default).muted_foreground)}
                        {b.name()}
                    </Button>)
                    .collect();
                let radii = [("0", 0.0f32), ("0.3", 4.8), ("0.5", 8.0), ("0.625", 10.0), ("0.75", 12.0), ("1.0", 16.0)]
                    .iter()
                    .map(|&(label, value)| <Button outline size={ButtonSize::Sm} class="w-full" label={label} onClick={move |_| Theme::set_radius(value)} {...mark(radius == value)} />)
                    .collect();
                let modes = [(ThemeMode::Light, "Light", IconName::Sun), (ThemeMode::Dark, "Dark", IconName::Moon)]
                    .iter()
                    .map(|&(m, label, icon)| <Button outline size={ButtonSize::Sm} class="w-full" icon={icon} label={label} onClick={move |_| Theme::change(m)} {...mark(m == mode)} />)
                    .collect();
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-1">
                        <div class="font-semibold leading-[1.25rem]">Customize</div>
                        <div class="leading-[1.25rem]" text_color={theme.muted_foreground}>Pick a color, base and radius for every component.</div>
                    </div>
                    <div class="flex flex-col gap-2">{section("Color")}{grid(accents, 3)}</div>
                    <div class="flex flex-col gap-2">{section("Base color")}{grid(bases, 3)}</div>
                    <div class="flex flex-col gap-2">{section("Radius")}{grid(radii, 3)}</div>
                    <div class="flex flex-col gap-2">{section("Mode")}{grid(modes, 2)}</div>
                </div>
            }
        }}
    />
}

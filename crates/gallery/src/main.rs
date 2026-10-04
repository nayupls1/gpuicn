//! Component gallery: every registry component, rendered straight from
//! `registry/ui` so what you see is exactly what `gpuicn add` installs.

#[path = "../../../registry/ui/mod.rs"]
mod ui;
mod demos;

use gpui::{App, Application, Bounds, Context, FontWeight, Window, WindowBounds, WindowOptions, div, prelude::*, px, size};
use ui::{
    button::Button,
    icon::IconName,
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

        let items = demos::DEMOS.iter().enumerate().map(|(ix, (label, _))| {
            let view = cx.entity().downgrade();
            SidebarMenuItem::new(*label).active(ix == self.page).on_click(move |_, _, cx| {
                view.update(cx, |this, cx| this.select(ix, cx)).ok();
            })
        });
        let sidebar = Sidebar::new("gallery-nav")
            .collapsible(SidebarCollapsible::None)
            .header(div().px_2().py_1().text_sm().font_weight(FontWeight::SEMIBOLD).child("gpuicn"))
            .group(SidebarGroup::new().label("Components").items(items));

        let content = demo(window, cx);
        let theme = cx.theme();
        let header = div()
            .flex()
            .flex_none()
            .h(px(56.))
            .items_center()
            .justify_between()
            .px_6()
            .border_b_1()
            .border_color(theme.border)
            .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).child(name))
            .child(
                Button::new("theme-toggle")
                    .ghost()
                    .icon(if dark { IconName::Sun } else { IconName::Moon })
                    .on_click(move |_, _, cx| {
                        Theme::change(if dark { ThemeMode::Light } else { ThemeMode::Dark }, cx);
                    }),
            );
        let preview = div()
            .flex()
            .min_h(px(420.))
            .items_center()
            .justify_center()
            .p_10()
            .rounded(theme.radius_lg())
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
            .child(sidebar)
            .child(
                SidebarInset::new().child(header).child(
                    ScrollArea::new("gallery-content")
                        .flex_1()
                        .min_h_0()
                        .child(div().p_6().child(preview)),
                ),
            )
            .child(Toaster::new())
            .child(PortalHost::new())
    }
}

fn main() {
    Application::new().with_assets(ui::icon::IconAssets).run(|cx: &mut App| {
        ui::init(cx);
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);
        cx.open_window(
            WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), ..Default::default() },
            |_, cx| cx.new(|_| Gallery::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests;

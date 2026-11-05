use rsx::prelude::*;

use crate::ui::icon::{Icon, IconName};
use crate::ui::navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuLink};
use crate::ui::sonner::toast;
use crate::ui::theme::{mix, theme, ColorExt};

pub fn demo() -> Element {
    let t = theme();
    let items = vec![
        NavigationMenuItem::new("Getting started").content(move || {
            let theme = t.get();
            // The featured tile (`row-span-3` with a gradient) next to three links.
            <div class="flex gap-2 w-[480px]">
                <div
                    class="flex flex-col justify-end flex-none w-[190px] p-4 gap-2 bg-gradient-to-b"
                    corner_radius={theme.radius_md()}
                    gradient_from={theme.muted.opacity(0.4)}
                    gradient_to={mix(theme.muted, theme.foreground, 0.06)}
                >
                    <Icon name={IconName::Terminal} class="size-8" />
                    <div class="mt-4 text-lg leading-[24px] font-medium">gpuicn</div>
                    <div class="text-sm leading-[18px]" text_color={theme.muted_foreground}>
                        Beautifully designed components, copied into your gpui app.
                    </div>
                </div>
                <div class="flex flex-col flex-1 min-w-0 gap-1">
                    <NavigationMenuLink title="Introduction" description="Re-usable components you copy into your app." />
                    <NavigationMenuLink title="Installation" description="How to install dependencies and structure your app." />
                    <NavigationMenuLink title="Typography" description="Styles for headings, lists and more." />
                </div>
            </div>
        }),
        NavigationMenuItem::new("Components").content(|| {
            <div class="grid grid-cols-2 gap-1 w-[420px]">
                <NavigationMenuLink title="Alert Dialog" description="A modal dialog that interrupts the user." />
                <NavigationMenuLink title="Hover Card" description="Preview content available behind a link." />
                <NavigationMenuLink title="Progress" description="Displays an indicator showing completion." />
                <NavigationMenuLink title="Tabs" description="Layered sections of content." />
            </div>
        }),
        NavigationMenuItem::new("Docs").on_click(|_| {
            toast("Docs clicked");
        }),
    ];
    <NavigationMenu items={items} />
}

use rsx::prelude::*;

use crate::demos::avatar_image;
use crate::ui::avatar::Avatar;
use crate::ui::breadcrumb::{Breadcrumb, BreadcrumbItem};
use crate::ui::icon::{Icon, IconName};
use crate::ui::separator::Separator;
use crate::ui::sidebar::{Sidebar, SidebarGroup, SidebarInset, SidebarMenuItem, SidebarTrigger};
use crate::ui::theme::{theme, ColorExt, Theme};

/// Two-line block with an icon tile, used for the sidebar's team switcher
/// and user menu (`SidebarMenuButton size="lg"`).
fn sidebar_block(tile: Element, title: &'static str, subtitle: &'static str) -> Element {
    let t = theme();
    <div class="flex flex-nowrap items-center gap-2 p-2" corner_radius={t.get().radius_md()}>
        {tile}
        <div class="flex flex-col flex-1 min-w-0 text-sm leading-[18px]">
            <div class="font-medium">{title}</div>
            <div class="text-xs" text_color={t.get().muted_foreground}>{subtitle}</div>
        </div>
        <Icon name={IconName::ChevronsUpDown} class="size-4 flex-none" text_color={t.get().muted_foreground} />
    </div>
}

/// `bg-muted/50` reads on dark; in light it needs full `bg-muted` to show.
fn placeholder(theme: &Theme) -> Color {
    if theme.is_dark() { theme.muted.opacity(0.5) } else { theme.muted }
}

pub fn demo() -> Element {
    let t = theme();
    let logo = <div
        class="flex flex-none size-8 items-center justify-center"
        corner_radius={t.get().radius_lg()}
        bg={t.get().sidebar_primary}
        text_color={t.get().sidebar_primary_foreground}
    >
        <Icon name={IconName::Terminal} />
    </div>;
    let user = <Avatar src={avatar_image(1)} fallback="CN" class="size-8" corner_radius={t.get_untracked().radius_lg()} />;
    let header = sidebar_block(logo, "Acme Inc.", "Enterprise");
    let footer = sidebar_block(user, "shadcn", "m@example.com");

    <div class="flex w-[720px] h-[420px] rounded-lg border overflow-hidden" border_color={t.get().border}>
        <Sidebar id="demo-sidebar" header={header} footer={footer}>
            <SidebarGroup label="Application">
                <SidebarMenuItem label="Home" icon={IconName::House} active />
                <SidebarMenuItem label="Inbox" icon={IconName::Inbox} badge="24" />
                <SidebarMenuItem label="Calendar" icon={IconName::Calendar} />
                <SidebarMenuItem label="Search" icon={IconName::Search} />
                <SidebarMenuItem label="Settings" icon={IconName::Settings}>
                    <SidebarMenuItem label="General" />
                    <SidebarMenuItem label="Team" />
                    <SidebarMenuItem label="Billing" />
                </SidebarMenuItem>
            </SidebarGroup>
        </Sidebar>
        <SidebarInset>
            <div class="flex items-center gap-2 h-12 px-4 border-b" border_color={t.get().border}>
                <SidebarTrigger sidebar="demo-sidebar" />
                <Separator vertical class="h-4" />
                <Breadcrumb>
                    <BreadcrumbItem label="Building Your Application" />
                    <BreadcrumbItem label="Data Fetching" current />
                </Breadcrumb>
            </div>
            <div class="flex flex-col gap-4 p-4">
                {[24, 40].map(|h| <div class={format!("h-{h} w-full rounded-xl")} bg={placeholder(&t.get())} />)}
            </div>
        </SidebarInset>
    </div>
}

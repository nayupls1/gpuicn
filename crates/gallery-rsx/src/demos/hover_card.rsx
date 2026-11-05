use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::hover_card::HoverCard;
use crate::ui::icon::{Icon, IconName};
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <HoverCard
        class="w-[320px]"
        trigger={<Button link label="@nextjs" />}
        content={move || <div class="flex items-start gap-4 flex-nowrap">
            {/* TODO(avatar): `<Avatar src={avatar_image(2)} fallback="VC" />` once `avatar` is ported. */}
            <div class="size-8 flex-none rounded-full flex items-center justify-center text-sm" bg={t.get().muted}>VC</div>
            <div class="flex flex-col flex-1 min-w-0 gap-1">
                <div class="text-sm font-semibold">@nextjs</div>
                <div class="text-sm">The React Framework – created and maintained by @vercel.</div>
                <div class="flex items-center gap-1.5 pt-1 text-xs leading-[16px]" text_color={t.get().muted_foreground}>
                    <Icon name={IconName::Calendar} class="size-3.5" />
                    Joined December 2021
                </div>
            </div>
        </div>}
    />
}

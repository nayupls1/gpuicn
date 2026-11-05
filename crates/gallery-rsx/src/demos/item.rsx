use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::icon::{Icon, IconName};
use crate::ui::item::{Item, ItemGroup};
use crate::ui::theme::theme;

/// Stand-in for an avatar: initials in a muted circle.
fn avatar(initials: &'static str) -> Element {
    let t = theme();
    <div class="flex flex-none size-8 items-center justify-center rounded-full text-sm" bg={t.get().muted}>{initials}</div>
}

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 w-[480px]">
        <Item
            outline
            title="Basic Item"
            description="A simple item with title and description."
            action={<Button outline size={ButtonSize::Sm} label="Action" />}
        />
        <Item
            outline
            icon={IconName::CircleCheck}
            title="Your profile has been verified."
            action={<Icon name={IconName::ChevronRight} />}
        />
        <ItemGroup separated>
            {/* TODO(avatar): `media={<Avatar src={avatar_image(0)} fallback="S" />}` once `avatar` is ported. */}
            <Item media={avatar("S")} title="shadcn" description="shadcn@vercel.com" />
            {/* TODO(avatar): `media={<Avatar fallback="M" />}` once `avatar` is ported. */}
            <Item media={avatar("M")} title="maxleiter" description="maxleiter@vercel.com" />
        </ItemGroup>
    </div>
}

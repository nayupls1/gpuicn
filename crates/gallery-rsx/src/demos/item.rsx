use rsx::prelude::*;

use crate::demos::avatar_image;
use crate::ui::avatar::Avatar;
use crate::ui::button::{Button, ButtonSize};
use crate::ui::icon::{Icon, IconName};
use crate::ui::item::{Item, ItemGroup};

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
            <Item media={<Avatar src={avatar_image(0)} fallback="S" />} title="shadcn" description="shadcn@vercel.com" />
            <Item media={<Avatar fallback="M" />} title="maxleiter" description="maxleiter@vercel.com" />
        </ItemGroup>
    </div>
}

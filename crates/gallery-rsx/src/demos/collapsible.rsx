use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::collapsible::Collapsible;
use crate::ui::icon::IconName;
use crate::ui::theme::theme;

fn repo(name: &'static str) -> Element {
    let t = theme();
    // rsx#8: no `font-mono`.
    <div class="rounded-md border px-4 py-2 text-sm" border_color={t.get().border}>{name}</div>
}

pub fn demo() -> Element {
    <Collapsible
        class="w-[350px] gap-2"
        trigger={<div class="flex items-center justify-between gap-4 px-4">
            <div class="text-sm font-semibold">@peduarte starred 3 repositories</div>
            <Button ghost size={ButtonSize::IconSm} icon={IconName::ChevronsUpDown} />
        </div>}
        content={<div class="flex flex-col gap-2">
            {repo("@radix-ui/colors")}
            {repo("@stitches/react")}
        </div>}
    >
        {repo("@radix-ui/primitives")}
    </Collapsible>
}

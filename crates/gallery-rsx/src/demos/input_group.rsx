use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::icon::IconName;
use crate::ui::input::Input;
use crate::ui::input_group::{InputGroup, InputGroupAddon};

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 w-[380px]">
        <InputGroup start={InputGroupAddon::icon(IconName::Search)} end={InputGroupAddon::text("12 results")}>
            <Input placeholder="Search..." />
        </InputGroup>
        <InputGroup
            start={InputGroupAddon::text("https://")}
            end={InputGroupAddon::button(<Button ghost size={ButtonSize::IconSm} icon={IconName::Copy} />)}
        >
            <Input placeholder="example.com" />
        </InputGroup>
        <InputGroup block_end={InputGroupAddon::element(<div class="flex w-full items-center justify-between gap-2">
            <Button outline size={ButtonSize::IconSm} class="rounded-full" icon={IconName::Plus} />
            <Button size={ButtonSize::IconSm} class="rounded-full" icon={IconName::ArrowUp} />
        </div>)}>
            <Input multiline placeholder="Ask, Search or Chat..." />
        </InputGroup>
    </div>
}

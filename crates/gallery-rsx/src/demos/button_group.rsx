use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::button_group::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText};
use crate::ui::icon::IconName;
use crate::ui::input::Input;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4">
        <ButtonGroup>
            <Button outline label="Archive" />
            <Button outline label="Report" />
            <Button outline label="Snooze" />
            <Button outline size={ButtonSize::Icon} icon={IconName::Ellipsis} />
        </ButtonGroup>
        <ButtonGroup>
            <ButtonGroupText text="https://" />
            <Input placeholder="example.com" class="rounded-none w-[200px]" />
            <Button outline label="Go" />
        </ButtonGroup>
        <ButtonGroup>
            <Button secondary size={ButtonSize::Icon} icon={IconName::Minus} />
            <ButtonGroupSeparator />
            <Button secondary size={ButtonSize::Icon} icon={IconName::Plus} />
        </ButtonGroup>
    </div>
}

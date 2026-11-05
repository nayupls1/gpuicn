use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::icon::IconName;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4">
        <div class="flex items-center gap-2">
            <Button label="Button" />
            <Button secondary label="Secondary" />
            <Button destructive label="Destructive" />
            <Button outline label="Outline" />
            <Button ghost label="Ghost" />
            <Button link label="Link" />
        </div>
        <div class="flex items-center gap-2">
            <Button size={ButtonSize::Sm} outline label="Small" />
            <Button size={ButtonSize::Lg} outline label="Large" />
            <Button outline size={ButtonSize::Icon} icon={IconName::ArrowUp} />
            <Button outline icon={IconName::Mail} label="Login with Email" />
            <Button size={ButtonSize::Sm} loading={true} label="Please wait" />
            <Button disabled label="Disabled" />
            {/* TODO(sonner): show a toast on click once `sonner` is ported. */}
            <Button outline label="Click me" tooltip={"Shows a toast".to_string()} />
        </div>
    </div>
}

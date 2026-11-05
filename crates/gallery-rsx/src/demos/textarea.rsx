use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::textarea::Textarea;

pub fn demo() -> Element {
    <div class="flex flex-col gap-4 w-[400px]">
        <Textarea placeholder="Type your message here." />
        <Button label="Send message" />
    </div>
}

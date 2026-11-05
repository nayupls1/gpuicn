use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::toast::{show, Toast};

pub fn demo() -> Element {
    <Button outline label="Add to calendar" onClick={move |_| {
        show(Toast::new("Scheduled: Catch up").description("Friday, February 10, 2023 at 5:57 PM").action("Undo", || {}));
    }} />
}

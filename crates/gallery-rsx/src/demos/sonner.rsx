use std::time::Duration;

use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::sonner::{toast, update, Toast};

pub fn demo() -> Element {
    <div class="flex items-center gap-2">
        <Button outline label="Show Toast" onClick={move |_| {
            toast(
                Toast::new("Event has been created")
                    .description("Sunday, December 03, 2023 at 9:00 AM")
                    .action("Undo", || println!("Undo")),
            );
        }} />
        <Button outline label="Success" onClick={move |_| {
            toast(Toast::new("Event has been created").success());
        }} />
        <Button outline label="Error" onClick={move |_| {
            toast(Toast::new("Something went wrong").error());
        }} />
        <Button outline label="Loading" onClick={move |_| {
            let id = toast(Toast::new("Uploading…").loading());
            spawn_local(async move {
                sleep(Duration::from_secs(2)).await;
                update(id, Toast::new("Upload complete").success());
            });
        }} />
    </div>
}

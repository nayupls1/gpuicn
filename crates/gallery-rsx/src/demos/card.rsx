use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::field::Field;
use crate::ui::input::Input;
use crate::ui::label::Label;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <Card class="w-[380px]">
        <CardHeader action={<Button link class="h-auto p-0" label="Sign Up" />}>
            <CardTitle text="Login to your account" />
            <CardDescription text="Enter your email below to login to your account" />
        </CardHeader>
        <CardContent>
            <div class="flex flex-col gap-6">
                <Field label="Email"><Input placeholder="m@example.com" /></Field>
                <div class="flex flex-col gap-3">
                    <div class="flex flex-nowrap items-center justify-between gap-2">
                        <Label text="Password" />
                        <Button link class="h-[14px] p-0 font-normal" text_color={t.get().foreground} label="Forgot your password?" />
                    </div>
                    <Input masked={true} />
                </div>
            </div>
        </CardContent>
        <CardFooter class="flex-col">
            <Button label="Login" class="w-full" />
            <Button outline label="Login with Google" class="w-full" />
        </CardFooter>
    </Card>
}

use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::label::Label;
use crate::ui::tabs::{Tab, Tabs};

pub fn demo() -> Element {
    // Fixed height so the tab list stays put when panels differ in height.
    <div class="h-[360px]">
        <Tabs class="w-[400px]" full_width default_value="account">
            <Tab value="account" label="Account">
                <Card>
                    <CardHeader>
                        <CardTitle text="Account" />
                        <CardDescription text="Make changes to your account here. Click save when you're done." />
                    </CardHeader>
                    <CardContent>
                        {/* TODO(field): `<Field label="Name"><Input default_value="Pedro Duarte" /></Field>` once `field` and `input` are ported. */}
                        <div class="flex flex-col gap-3">
                            <Label text="Name" />
                            <input value="Pedro Duarte" />
                        </div>
                    </CardContent>
                    <CardFooter><Button label="Save changes" /></CardFooter>
                </Card>
            </Tab>
            <Tab value="password" label="Password">
                <Card>
                    <CardHeader>
                        <CardTitle text="Password" />
                        <CardDescription text="Change your password here." />
                    </CardHeader>
                    <CardContent>
                        {/* TODO(field): `<Field label="Current password"><Input masked /></Field>` (and "New password") once `field` and `input` are ported. */}
                        <div class="flex flex-col gap-4">
                            <div class="flex flex-col gap-3">
                                <Label text="Current password" />
                                <input type="password" />
                            </div>
                            <div class="flex flex-col gap-3">
                                <Label text="New password" />
                                <input type="password" />
                            </div>
                        </div>
                    </CardContent>
                    <CardFooter><Button label="Save password" /></CardFooter>
                </Card>
            </Tab>
        </Tabs>
    </div>
}

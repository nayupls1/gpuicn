//! A dashboard-style composition of many components (the examples on the
//! shadcn homepage), like the gpui gallery's first page.

use rsx::prelude::*;

use crate::demos::avatar_image;
use crate::ui::avatar::Avatar;
use crate::ui::badge::Badge;
use crate::ui::button::{Button, ButtonSize};
use crate::ui::calendar::{Calendar, CalendarSelection, Date};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::chart::{ChartSeries, LineChart};
use crate::ui::field::Field;
use crate::ui::icon::IconName;
use crate::ui::input::Input;
use crate::ui::label::Label;
use crate::ui::select::{Select, SelectItem};
use crate::ui::separator::Separator;
use crate::ui::switch::Switch;
use crate::ui::textarea::Textarea;
use crate::ui::theme::theme;

fn create_account() -> Element {
    let t = theme();
    <Card>
        <CardHeader>
            <CardTitle text="Create an account" class="text-2xl leading-[28px]" />
            <CardDescription text="Enter your email below to create your account" />
        </CardHeader>
        <CardContent>
            <div class="flex flex-col gap-4">
                <div class="grid grid-cols-2 gap-4">
                    <Button outline icon={IconName::User} label="SSO" class="w-full" />
                    <Button outline icon={IconName::Mail} label="Magic link" class="w-full" />
                </div>
                <div class="flex items-center gap-3">
                    <Separator class="flex-1" />
                    <div class="text-xs leading-[16px]" text_color={t.get().muted_foreground}>OR CONTINUE WITH</div>
                    <Separator class="flex-1" />
                </div>
                <Field label="Email"><Input placeholder="m@example.com" /></Field>
                <Field label="Password"><Input masked={true} /></Field>
            </div>
        </CardContent>
        <CardFooter><Button label="Create account" class="w-full" /></CardFooter>
    </Card>
}

fn setting(title: &'static str, description: &'static str, on: bool) -> Element {
    let t = theme();
    <div class="flex items-center justify-between gap-4">
        <div class="flex flex-col gap-1 flex-1 min-w-0">
            <Label text={title} />
            <div class="text-sm leading-[20px]" text_color={t.get().muted_foreground}>{description}</div>
        </div>
        <Switch default_checked={on} />
    </div>
}

fn cookies() -> Element {
    <Card>
        <CardHeader>
            <CardTitle text="Cookie Settings" />
            <CardDescription text="Manage your cookie settings here." />
        </CardHeader>
        <CardContent>
            <div class="flex flex-col gap-5">
                {setting("Strictly Necessary", "These cookies are essential to use the app.", true)}
                {setting("Functional Cookies", "These cookies remember your preferences.", false)}
                {setting("Performance Cookies", "These help us improve the experience.", true)}
            </div>
        </CardContent>
        <CardFooter><Button outline label="Save preferences" class="w-full" /></CardFooter>
    </Card>
}

fn revenue() -> Element {
    let t = theme();
    <Card>
        <CardHeader action={<Button ghost size={ButtonSize::IconSm} icon={IconName::Ellipsis} />}>
            <CardDescription text="Total Revenue" />
            <CardTitle text="$15,231.89" class="text-2xl leading-[32px]" />
            <div class="flex items-center gap-1.5">
                <Badge outline icon={IconName::ArrowUpRight} label="+20.1%" />
                <div class="text-xs leading-[16px]" text_color={t.get().muted_foreground}>from last month</div>
            </div>
        </CardHeader>
        <CardContent>
            <LineChart
                class="w-[230px]"
                labels={["Jan", "Feb", "Mar", "Apr", "May", "Jun"]}
                series={vec![ChartSeries::new("Revenue").data([10_400., 14_405., 9_400., 8_200., 12_800., 15_231.])]}
                height={110}
                legend={false}
                dots
            />
        </CardContent>
    </Card>
}

fn member(n: usize, name: &'static str, email: &'static str, role: &'static str) -> Element {
    let t = theme();
    <div class="flex items-center gap-3">
        <Avatar src={avatar_image(n)} fallback={name[..1].to_string()} />
        <div class="flex flex-col flex-1 min-w-0">
            <div class="truncate text-sm leading-[20px] font-medium">{name}</div>
            <div class="truncate text-sm leading-[20px]" text_color={t.get().muted_foreground}>{email}</div>
        </div>
        <Button outline size={ButtonSize::Sm} class="gap-1 px-2.5" label={role} icon_end={IconName::ChevronDown} />
    </div>
}

fn team() -> Element {
    <Card>
        <CardHeader>
            <CardTitle text="Team Members" />
            <CardDescription text="Invite your team members to collaborate." />
        </CardHeader>
        <CardContent>
            <div class="flex flex-col gap-5">
                {member(0, "Sofia Davis", "m@acme.co", "Owner")}
                {member(1, "Ethan Lee", "e@acme.co", "Member")}
                {member(2, "Olivia Kim", "o@acme.co", "Member")}
            </div>
        </CardContent>
    </Card>
}

fn calendar() -> Element {
    let t = theme();
    let today = Date::today();
    <Card class="py-0 items-center">
        <Calendar bg={t.get().card} default_selection={CalendarSelection::Single(Some(today.add_days(3)))} />
    </Card>
}

fn report() -> Element {
    <Card>
        <CardHeader>
            <CardTitle text="Report an issue" />
            <CardDescription text="What area are you having problems with?" />
        </CardHeader>
        <CardContent>
            <div class="flex flex-col gap-4 w-[230px]">
                <div class="grid grid-cols-2 gap-4">
                    <Field label="Area">
                        <Select
                            class="w-full"
                            default_value="billing"
                            items={vec![SelectItem::new("team", "Team"), SelectItem::new("billing", "Billing"), SelectItem::new("account", "Account")]}
                        />
                    </Field>
                    <Field label="Severity">
                        <Select
                            class="w-full"
                            default_value="2"
                            items={vec![SelectItem::new("1", "Severity 1"), SelectItem::new("2", "Severity 2"), SelectItem::new("3", "Severity 3")]}
                        />
                    </Field>
                </div>
                <Field label="Subject" class="w-full"><Input class="w-full" placeholder="I need help with..." /></Field>
                <Field label="Description" class="w-full">
                    <Textarea class="w-full" placeholder="Please include all information relevant to your issue." />
                </Field>
            </div>
        </CardContent>
        <CardFooter class="justify-between">
            <Button ghost label="Cancel" />
            <Button label="Submit" />
        </CardFooter>
    </Card>
}

pub fn demo() -> Element {
    <div class="w-[872px] flex items-start gap-4">
        <div class="flex flex-col gap-4 w-[280px] flex-none">{create_account()}{cookies()}</div>
        <div class="flex flex-col gap-4 w-[280px] flex-none">{revenue()}{team()}</div>
        <div class="flex flex-col gap-4 w-[280px] flex-none">{calendar()}{report()}</div>
    </div>
}

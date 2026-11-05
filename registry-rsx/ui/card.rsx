//! Displays a card with header, content, and footer.
//!
//! ```ignore
//! <Card class="w-[380px]">
//!     <CardHeader action={<Button link label="Sign Up" />}>
//!         <CardTitle text="Login to your account" />
//!         <CardDescription text="Enter your email below" />
//!     </CardHeader>
//!     <CardContent>...</CardContent>
//!     <CardFooter><Button label="Login" class="w-full" /></CardFooter>
//! </Card>
//! ```

use rsx::prelude::*;

use super::theme::theme;

#[component]
pub fn Card(children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();
        <div
            class="flex flex-col gap-6 py-6 border"
            class:shadow-sm={theme.shadows}
            corner_radius={theme.radius_xl()}
            border_color={theme.border}
            bg={theme.card}
            text_color={theme.card_foreground}
            {...&attrs}
        >
            {children}
        </div>
    }
}

#[component]
pub fn CardHeader(
    /// Content pinned to the top-right corner of the header (`CardAction`).
    action: Option<Children>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    <div class="flex items-start gap-2 px-6" {...attrs}>
        <div class="flex flex-col flex-1 min-w-0 gap-1.5">{children}</div>
        {action.map(|action| <div class="flex-none">{action}</div>)}
    </div>
}

#[component]
pub fn CardTitle(text: Option<Accessor<String>>, children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    <div class="text-base leading-none font-semibold" {...attrs}>
        {text}
        {children}
    </div>
}

#[component]
pub fn CardDescription(text: Option<Accessor<String>>, children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    let t = theme();
    move || {
        <div class="text-sm leading-[1.25rem]" text_color={t.get().muted_foreground} {...&attrs}>
            {text}
            {children}
        </div>
    }
}

#[component]
pub fn CardContent(children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    <div class="px-6" {...attrs}>{children}</div>
}

#[component]
pub fn CardFooter(children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    <div class="flex items-center gap-2 px-6" {...attrs}>{children}</div>
}

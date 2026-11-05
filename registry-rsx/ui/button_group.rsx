//! A container that groups related buttons together with consistent styling.
//!
//! ```ignore
//! <ButtonGroup>
//!     <Button outline label="Archive" />
//!     <Button outline label="Report" />
//!     <ButtonGroupSeparator />
//!     <Button outline size={ButtonSize::Icon} icon={IconName::Ellipsis} />
//! </ButtonGroup>
//! <ButtonGroup>
//!     <ButtonGroupText text="https://" />
//!     <Input placeholder="example.com" />
//!     <Button outline label="Go" />
//! </ButtonGroup>
//! ```
//!
//! `Button`s, `ButtonGroupText`s and `ButtonGroupSeparator`s inside take
//! their corners and borders from their position; any other element is
//! inserted as-is.

use std::rc::Rc;

use rsx::prelude::*;
use rsx::{Div, Length};

use super::button::ButtonJoin;
use super::theme::theme;
use super::utils::Siblings;

#[derive(Clone, Copy)]
struct GroupContext {
    vertical: bool,
    items: Siblings,
}

impl GroupContext {
    /// `(first, last)` for a registered part (tracked).
    fn position(&self, id: u64) -> (bool, bool) {
        (self.items.is_first(id), self.items.is_last(id))
    }
}

/// Which corners a part keeps: `rounded-l-*` / `rounded-r-*` (or `-t-` /
/// `-b-` when vertical), all of them when it's alone, else none.
fn corners(el: Div, radius: Length, vertical: bool, first: bool, last: bool) -> Div {
    let el = el.class("rounded-none");
    match (vertical, first, last) {
        (_, true, true) => el.corner_radius(radius),
        (false, true, false) => el.rounded_l_radius(radius),
        (false, false, true) => el.rounded_r_radius(radius),
        (true, true, false) => el.rounded_t_radius(radius),
        (true, false, true) => el.rounded_b_radius(radius),
        _ => el,
    }
}

/// `border-l-0` (or `border-t-0` when vertical) for parts after the first,
/// so neighbours don't draw a double border.
fn collapse_border(el: Div, vertical: bool, first: bool) -> Div {
    match (first, vertical) {
        (true, _) => el,
        (false, false) => el.class("border-l-0"),
        (false, true) => el.class("border-t-0"),
    }
}

#[component]
pub fn ButtonGroup(
    vertical: bool,
    /// `Button`s, `ButtonGroupText`s, `ButtonGroupSeparator`s, or anything else.
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let context = GroupContext { vertical, items: Siblings::new() };
    let t = theme();
    provide_context(context);
    // Each `Button` inside registers here and gets its corners and borders.
    provide_context(ButtonJoin(Rc::new(move || {
        let id = context.items.register();
        Attributes::new().bind(move |el| {
            let (first, last) = context.position(id);
            collapse_border(corners(el, t.get().radius_md(), vertical, first, last), vertical, first)
        })
    })));
    <div class="flex" class:flex-col={vertical} class:items-stretch={vertical} {...attrs}>
        {children}
    </div>
}

/// `ButtonGroupText` — a static label segment.
#[component]
pub fn ButtonGroupText(text: Option<Accessor<String>>, children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    let context = expect_context::<GroupContext>();
    let id = context.items.register();
    let vertical = context.vertical;
    let t = theme();
    let style = move |el: Div| {
        let theme = t.get();
        let (first, last) = context.position(id);
        // Only the outer edge is rounded, even when it's the only part.
        let el = match (vertical, first, last) {
            (false, true, _) => el.class("rounded-none").rounded_l_radius(theme.radius_md()),
            (false, _, true) => el.class("rounded-none").rounded_r_radius(theme.radius_md()),
            (true, true, _) => el.class("rounded-none").rounded_t_radius(theme.radius_md()),
            (true, _, true) => el.class("rounded-none").rounded_b_radius(theme.radius_md()),
            _ => el,
        };
        let el = collapse_border(el.bg(theme.muted).border_color(theme.border), vertical, first);
        if theme.shadows { el.shadow_xs() } else { el }
    };
    <div class="flex items-center gap-2 px-4 text-sm leading-[1.25rem] font-medium border" bind={style} {...attrs}>
        {text}
        {children}
    </div>
}

/// `ButtonGroupSeparator`.
#[component]
pub fn ButtonGroupSeparator(attrs: Attributes) -> impl IntoElement {
    let context = expect_context::<GroupContext>();
    let _id = context.items.register();
    let vertical = context.vertical;
    let t = theme();
    <div
        class="flex-none"
        class:h-px={vertical}
        class:w-full={vertical}
        class:w-px={!vertical}
        bg={t.get().input}
        {...attrs}
    />
}

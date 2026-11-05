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

/// Rounds only the given corners, `rounded-l-*` style.
fn corners(mut el: Div, radius: Length, [tl, tr, br, bl]: [bool; 4]) -> Div {
    let r = |on: bool| Some(if on { radius } else { px(0.0) });
    let c = &mut el.style().corner_radii;
    c.top_left = r(tl);
    c.top_right = r(tr);
    c.bottom_right = r(br);
    c.bottom_left = r(bl);
    el
}

/// `border-l-0` (or `border-t-0` when vertical) for parts after the first.
fn collapse_border(mut el: Div, vertical: bool, first: bool) -> Div {
    if !first {
        let b = &mut el.style().border_widths;
        if vertical {
            b.top = Some(px(0.0));
        } else {
            b.left = Some(px(0.0));
        }
    }
    el
}

#[component]
pub fn ButtonGroup(
    vertical: bool,
    /// `Button`s, `ButtonGroupText`s, `ButtonGroupSeparator`s, or anything else.
    children: Option<ChildrenFn>,
    attrs: Attributes,
) -> impl IntoElement {
    let context = GroupContext { vertical, items: Siblings::new() };
    let t = theme();
    // Each `Button` inside registers here and gets its corners and borders.
    let join = ButtonJoin(Rc::new(move || {
        let id = context.items.register();
        Attributes::new().with(move |el| {
            let (first, last) = context.position(id);
            let radius = t.get().radius_md();
            let el = corners(el, radius, match (vertical, first, last) {
                (_, true, true) => [true; 4],
                (false, true, false) => [true, false, false, true],
                (false, false, true) => [false, true, true, false],
                (true, true, false) => [true, true, false, false],
                (true, false, true) => [false, false, true, true],
                _ => [false; 4],
            });
            collapse_border(el, vertical, first)
        })
    }));
    // rsx gap: a context doesn't reach `Children` (built in the caller's scope), so the
    // parts come in as `ChildrenFn`, rendered inside the `Provider`.
    <div class="flex" class:flex-col={vertical} class:items-stretch={vertical} {...attrs}>
        <Provider value={context}>
            <Provider value={join}>{children}</Provider>
        </Provider>
    </div>
}

/// `ButtonGroupText` — a static label segment.
#[component]
pub fn ButtonGroupText(text: Option<Accessor<String>>, children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    let context = expect_context::<GroupContext>();
    let id = context.items.register();
    let vertical = context.vertical;
    let t = theme();
    move || {
        let theme = t.get();
        let (first, last) = context.position(id);
        let el = div()
            .class("flex items-center gap-2 px-4 text-sm leading-[1.25rem] font-medium border")
            .bg(theme.muted)
            .border_color(theme.border);
        let el = collapse_border(el, vertical, first);
        let el = corners(el, theme.radius_md(), match (vertical, first, last) {
            (false, true, _) => [true, false, false, true],
            (false, _, true) => [false, true, true, false],
            (true, true, _) => [true, true, false, false],
            (true, _, true) => [false, false, true, true],
            _ => [false; 4],
        });
        // rsx#7: no `shadow-xs`.
        el.spread(&attrs).child(text).child(children)
    }
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

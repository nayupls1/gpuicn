#![cfg(test)]
//! Interaction tests for the layout batch: accordion, collapsible, tabs,
//! pagination, breadcrumb, item, scroll area, resizable, button group.

use gpui::{Modifiers, MouseButton, TestAppContext};
use rsx::prelude::*;

use crate::tests::{center, mount, settle};

/// `ref` for a component's root: components don't forward `ref`, but take it through a spread.
fn node(r: NodeRef) -> Attributes {
    Attributes::new().with(move |el| el.node_ref(r))
}

fn point(x: f32, y: f32) -> gpui::Point<gpui::Pixels> {
    gpui::point(gpui::px(x), gpui::px(y))
}

#[gpui::test]
fn accordion_opens_one_item_at_a_time(cx: &mut TestAppContext) {
    use crate::ui::accordion::{Accordion, AccordionItem};
    let changes = cx.update(|_| signal(Vec::<SharedString>::new()));
    let (first, second, second_body) = cx.update(|_| (NodeRef::new(), NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8 w-[480px]">
            <Accordion default_value={vec!["one".into()]} onChange={move |v: &Vec<SharedString>| changes.set(v.clone())}>
                <AccordionItem value="one" title="One"><div ref={first}>first</div></AccordionItem>
                <AccordionItem value="two" title="Two"><div ref={second_body}>second</div></AccordionItem>
                <AccordionItem value="three" title="Three"><div ref={second}>third</div></AccordionItem>
            </Accordion>
        </div>
    });
    assert!(first.bounds().is_some(), "the default item is open");
    assert!(second_body.bounds().is_none());
    // Click the second heading: just above its body's position is its row.
    let b = first.bounds().unwrap();
    // Heading rows are 52px (py-4 + 20px line); "Two" follows the first body (pb-4).
    cx.simulate_click(point(b.origin.x + 20.0, b.origin.y + b.size.height + 16.0 + 26.0), Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| changes.get_untracked()), vec![SharedString::from("two")]);
    assert!(second_body.bounds().is_some(), "the second item opened");
    // Enter on the focused heading closes it again (collapsible).
    cx.simulate_keystrokes("enter");
    settle(cx);
    assert!(cx.update(|_, _| changes.get_untracked()).is_empty());
}

#[gpui::test]
fn collapsible_toggles_on_trigger_click(cx: &mut TestAppContext) {
    use crate::ui::collapsible::Collapsible;
    let (trigger, content) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    let open = cx.update(|_| signal(false));
    let cx = mount(cx, move || {
        <div class="p-8">
            <Collapsible
                trigger={<div ref={trigger} class="p-2">Toggle</div>}
                content={<div ref={content}>hidden</div>}
                onOpenChange={move |o: &bool| open.set(*o)}
            >
                <div>always</div>
            </Collapsible>
        </div>
    });
    assert!(content.bounds().is_none());
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    assert!(cx.update(|_, _| open.get_untracked()));
    assert!(content.bounds().is_some(), "the content is shown");
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    assert!(!cx.update(|_, _| open.get_untracked()));
}

#[gpui::test]
fn tabs_switch_on_click_and_arrow_keys(cx: &mut TestAppContext) {
    use crate::ui::tabs::{Tab, Tabs};
    let selected = cx.update(|_| signal(SharedString::default()));
    let (account, password) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8">
            <Tabs class="w-[400px]" full_width onChange={move |v: &SharedString| selected.set(v.clone())}>
                <Tab value="account" label="Account"><div ref={account}>account</div></Tab>
                <Tab value="password" label="Password"><div ref={password}>password</div></Tab>
            </Tabs>
        </div>
    });
    assert!(account.bounds().is_some(), "the first tab is selected by default");
    assert!(password.bounds().is_none());
    // The list sits above the content: click its right half.
    let b = account.bounds().unwrap();
    cx.simulate_click(point(b.origin.x + 300.0, b.origin.y - 26.0), Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| selected.get_untracked()), "password");
    assert!(password.bounds().is_some());
    cx.simulate_keystrokes("left");
    settle(cx);
    assert_eq!(cx.update(|_, _| selected.get_untracked()), "account");
}

#[gpui::test]
fn pagination_moves_between_pages(cx: &mut TestAppContext) {
    use crate::ui::pagination::Pagination;
    let page = cx.update(|_| signal(2usize));
    let root = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 flex">
            <Pagination {...node(root)} total={10usize} page={page} onChange={move |p: &usize| page.set(*p)} />
        </div>
    });
    let b = root.bounds().unwrap();
    // "Next" is the right-most button, centered vertically.
    cx.simulate_click(point(b.origin.x + b.size.width - 30.0, b.origin.y + b.size.height / 2.0), Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| page.get_untracked()), 3);
}

#[gpui::test]
fn breadcrumb_links_and_items_are_clickable(cx: &mut TestAppContext) {
    use crate::ui::breadcrumb::{Breadcrumb, BreadcrumbItem};
    use crate::ui::item::Item;
    let (clicks, item_clicks) = cx.update(|_| (signal(0), signal(0)));
    let (home, item) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col gap-4 w-[400px]">
            <Breadcrumb>
                <BreadcrumbItem label="Home" {...node(home)} onClick={move |_| clicks.update(|n| *n += 1)} />
                <BreadcrumbItem label="Here" current />
            </Breadcrumb>
            <Item {...node(item)} title="Clickable" onClick={move |_| item_clicks.update(|n| *n += 1)} />
        </div>
    });
    cx.simulate_click(center(home), Modifiers::default());
    cx.simulate_click(center(item), Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| clicks.get_untracked()), 1);
    assert_eq!(cx.update(|_, _| item_clicks.get_untracked()), 1);
}

#[gpui::test]
fn scroll_area_scrolls_with_the_wheel(cx: &mut TestAppContext) {
    use crate::ui::scroll_area::{ScrollArea, ScrollHandle};
    let handle = cx.update(|_| ScrollHandle::new());
    let area = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8">
            <ScrollArea {...node(area)} class="h-[200px] w-[200px]" track_scroll={handle}>
                {(0..50).map(|n| <div class="h-[20px]">{format!("row {n}")}</div>)}
            </ScrollArea>
        </div>
    });
    assert!((cx.update(|_, _| handle.max_offset().height) - 800.0).abs() < 1.0, "1000px of rows in a 200px viewport");
    // Drag the thumb (8px wide at the right edge, 40px long at the top) down by 80px:
    // the 160px of travel maps to 800px of content.
    let a = area.bounds().unwrap();
    let (x, y) = (a.origin.x + a.size.width - 6.0, a.origin.y + 25.0);
    cx.simulate_mouse_down(point(x, y), MouseButton::Left, Modifiers::default());
    settle(cx);
    cx.simulate_mouse_move(point(x, y + 80.0), MouseButton::Left, Modifiers::default());
    settle(cx);
    cx.simulate_mouse_up(point(x, y + 80.0), MouseButton::Left, Modifiers::default());
    settle(cx);
    assert!((cx.update(|_, _| handle.offset().y) + 400.0).abs() < 1.0, "{}", cx.update(|_, _| handle.offset().y));
    cx.update(|_, _| handle.set_offset(Default::default()));
    settle(cx);
    cx.simulate_event(gpui::ScrollWheelEvent {
        position: center(area),
        delta: gpui::ScrollDelta::Pixels(point(0.0, -120.0)),
        modifiers: Modifiers::default(),
        touch_phase: gpui::TouchPhase::Moved,
    });
    settle(cx);
    assert_eq!(cx.update(|_, _| handle.offset().y), -120.0);
    // Scrolling past the end clamps.
    cx.simulate_event(gpui::ScrollWheelEvent {
        position: center(area),
        delta: gpui::ScrollDelta::Pixels(point(0.0, -5000.0)),
        modifiers: Modifiers::default(),
        touch_phase: gpui::TouchPhase::Moved,
    });
    settle(cx);
    assert!((cx.update(|_, _| handle.offset().y) + 800.0).abs() < 1.0);
}

#[gpui::test]
fn resizable_handle_drags(cx: &mut TestAppContext) {
    use crate::ui::resizable::{ResizablePanel, ResizablePanelGroup};
    let sizes = cx.update(|_| signal(Vec::<f32>::new()));
    let (group, left) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8">
            <div ref={group} class="w-[400px] h-[200px]">
                <ResizablePanelGroup with_handle onResize={move |s: &Vec<f32>| sizes.set(s.clone())}>
                    <ResizablePanel default_size={50.0f32} min_size={20.0f32}><div ref={left} class="size-full">One</div></ResizablePanel>
                    <ResizablePanel><div class="size-full">Two</div></ResizablePanel>
                </ResizablePanelGroup>
            </div>
        </div>
    });
    let g = group.bounds().unwrap();
    let l = left.bounds().unwrap();
    assert!((l.size.width - 200.0).abs() < 2.0, "half of the group: {}", l.size.width);
    // Grab the handle at the panels' boundary and drag it to 75%.
    let y = g.origin.y + 100.0;
    let handle_x = l.origin.x + l.size.width + 0.5;
    cx.simulate_mouse_down(point(handle_x, y), MouseButton::Left, Modifiers::default());
    settle(cx);
    cx.simulate_mouse_move(point(g.origin.x + 300.0, y), MouseButton::Left, Modifiers::default());
    settle(cx);
    cx.simulate_mouse_up(point(g.origin.x + 300.0, y), MouseButton::Left, Modifiers::default());
    settle(cx);
    let s = cx.update(|_, _| sizes.get_untracked());
    assert!((s[0] - 75.0).abs() < 0.5 && (s[1] - 25.0).abs() < 0.5, "{s:?}");
    assert!((left.bounds().unwrap().size.width - 300.0).abs() < 2.0);
    // The min size holds.
    cx.simulate_mouse_down(point(g.origin.x + 300.0, y), MouseButton::Left, Modifiers::default());
    settle(cx);
    cx.simulate_mouse_move(point(g.origin.x + 10.0, y), MouseButton::Left, Modifiers::default());
    settle(cx);
    cx.simulate_mouse_up(point(g.origin.x + 10.0, y), MouseButton::Left, Modifiers::default());
    settle(cx);
    assert!((cx.update(|_, _| sizes.get_untracked())[0] - 20.0).abs() < 0.5);
}

/// GPUI focuses a button on mouse down; the click must survive that.
#[gpui::test]
fn button_clicks_on_first_press(cx: &mut TestAppContext) {
    use crate::ui::button::Button;
    let clicks = cx.update(|_| signal(0));
    let b = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 flex">
            <div ref={b}><Button label="Hi" onClick={move |_| clicks.update(|n| *n += 1)} /></div>
        </div>
    });
    cx.simulate_click(center(b), Modifiers::default());
    settle(cx);
    assert_eq!(cx.update(|_, _| clicks.get_untracked()), 1);
}

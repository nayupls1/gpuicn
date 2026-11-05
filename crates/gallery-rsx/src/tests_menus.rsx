#![cfg(test)]
//! GPUI tests for the menus batch: dropdown menu, context menu, menubar,
//! navigation menu, select, native select.

use std::cell::RefCell;
use std::rc::Rc;

use gpui::{Modifiers, MouseButton, TestAppContext};
use rsx::prelude::*;

use crate::tests::{center, mount, settle};
use crate::ui::dropdown_menu::{DropdownMenu, Menu, MenuBuilder, MenuEntry, MenuItem};
use crate::ui::popover::PopoverHandle;

fn log() -> Rc<RefCell<Vec<String>>> {
    Rc::new(RefCell::new(Vec::new()))
}

#[gpui::test]
fn dropdown_menu_keyboard_and_select(cx: &mut TestAppContext) {
    let handle = cx.update(|_| PopoverHandle::new());
    let trigger = cx.update(|_| NodeRef::new());
    let picked = log();
    let p = picked.clone();
    let cx = mount(cx, move || {
        let p1 = p.clone();
        let p2 = p.clone();
        <div class="p-8">
            <DropdownMenu
                handle={handle}
                trigger={<div ref={trigger} class="p-2">Open</div>}
                menu={Menu::new()
                    .label("My Account")
                    .item(MenuItem::new("Profile").on_select(move || p1.borrow_mut().push("profile".into())))
                    .item(MenuItem::new("API").disabled(true))
                    .separator()
                    .item(MenuItem::new("Log out").on_select(move || p2.borrow_mut().push("logout".into())))}
            />
        </div>
    });
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    assert!(handle.is_open_untracked());
    // ↓ highlights "Profile", ↓ skips the disabled item to "Log out", Enter picks it.
    cx.simulate_keystrokes("down down enter");
    settle(cx);
    assert_eq!(*picked.borrow(), vec!["logout".to_string()]);
    assert!(!handle.is_open_untracked(), "selecting an item closes the menu");

    // Reopen, ↑ wraps to the last item, Escape closes without picking.
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    assert!(handle.is_open_untracked());
    cx.simulate_keystrokes("up up enter");
    settle(cx);
    assert_eq!(*picked.borrow(), vec!["logout".to_string(), "profile".to_string()]);
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("escape");
    settle(cx);
    assert!(!handle.is_open_untracked());
}

#[gpui::test]
fn dropdown_menu_click_hover_and_submenu(cx: &mut TestAppContext) {
    let handle = cx.update(|_| PopoverHandle::new());
    let trigger = cx.update(|_| NodeRef::new());
    let checked = cx.update(|_| signal(false));
    let picked = log();
    let p = picked.clone();
    let cx = mount(cx, move || {
        let p1 = p.clone();
        <div class="p-8">
            <DropdownMenu
                handle={handle}
                trigger={<div ref={trigger} class="p-2">Open</div>}
                class="w-[224px]"
                menu={move || Menu::new()
                    .item(MenuItem::checkbox("Status Bar", checked.get()).keep_open().on_select(move || checked.update(|c| *c = !*c)))
                    .separator()
                    .submenu("Invite users", vec![MenuItem::new("Email").on_select({
                        let p1 = p1.clone();
                        move || p1.borrow_mut().push("email".into())
                    }).into(), MenuEntry::Separator, MenuItem::new("More...").into()])}
            />
        </div>
    });
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    let content = handle.content_ref().bounds().expect("menu laid out");
    assert!((content.size.width - 224.0).abs() < 1.0, "class sets the panel width: {content:?}");
    // The first row is the checkbox item; clicking keeps the menu open and toggles it.
    let first = gpui::point(gpui::px(content.origin.x + 40.0), gpui::px(content.origin.y + 5.0 + 10.0));
    cx.simulate_click(first, Modifiers::default());
    settle(cx);
    assert!(checked.get_untracked(), "checkbox item toggled");
    assert!(handle.is_open_untracked(), "keep_open items leave the menu open");
    // ↑ reaches the submenu row (last), → opens it beside the list, ↓ Enter picks "Email".
    cx.simulate_keystrokes("up right");
    settle(cx);
    // The submenu sits 8px right of its row, its first item level with the row:
    // "Email" is just right of the panel, at the submenu row's height.
    let row_y = content.origin.y + 5.0 + 32.0 + 9.0 + 16.0;
    let email = gpui::point(gpui::px(content.origin.x + 224.0 + 3.0 + 1.0 + 4.0 + 6.0), gpui::px(row_y));
    cx.simulate_mouse_move(email, None, Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("enter");
    settle(cx);
    assert_eq!(*picked.borrow(), vec!["email".to_string()], "hovering the submenu's first item, then Enter, picks it");
    assert!(!handle.is_open_untracked(), "picking from a submenu closes the whole menu");
}


fn pt(x: f32, y: f32) -> gpui::Point<gpui::Pixels> {
    gpui::point(gpui::px(x), gpui::px(y))
}

#[gpui::test]
fn context_menu_opens_at_the_right_click(cx: &mut TestAppContext) {
    use crate::ui::context_menu::ContextMenu;
    let area = cx.update(|_| NodeRef::new());
    let picked = log();
    let p = picked.clone();
    let cx = mount(cx, move || {
        let p = p.clone();
        <div class="p-8">
            <ContextMenu
                trigger={<div ref={area} class="w-[300px] h-[150px]">Right click here</div>}
                class="min-w-[208px]"
                menu={Menu::new().item(MenuItem::new("Back").on_select(move || p.borrow_mut().push("back".into())))}
            />
        </div>
    });
    let a = area.bounds().unwrap();
    let at = pt(a.origin.x + 50.0, a.origin.y + 40.0);
    // A left click does nothing.
    cx.simulate_click(at, Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("down enter");
    settle(cx);
    assert!(picked.borrow().is_empty());
    // A right click opens the menu with its corner at the pointer.
    cx.simulate_mouse_down(at, MouseButton::Right, Modifiers::default());
    cx.simulate_mouse_up(at, MouseButton::Right, Modifiers::default());
    settle(cx);
    // "Back" is the first row: 1px border + 4px padding below the pointer.
    cx.simulate_click(pt(a.origin.x + 50.0 + 20.0, a.origin.y + 40.0 + 5.0 + 16.0), Modifiers::default());
    settle(cx);
    assert_eq!(*picked.borrow(), vec!["back".to_string()]);
    // Escape closes it: ↓ Enter afterwards picks nothing.
    cx.simulate_mouse_down(at, MouseButton::Right, Modifiers::default());
    cx.simulate_mouse_up(at, MouseButton::Right, Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("escape");
    settle(cx);
    cx.simulate_keystrokes("down enter");
    settle(cx);
    assert_eq!(picked.borrow().len(), 1);
}

#[gpui::test]
fn menubar_switches_menus_as_the_pointer_moves(cx: &mut TestAppContext) {
    use crate::ui::menubar::{Menubar, MenubarMenu};
    let bar = cx.update(|_| NodeRef::new());
    let picked = log();
    let p = picked.clone();
    let cx = mount(cx, move || {
        let (p1, p2) = (p.clone(), p.clone());
        <div class="p-8 flex">
            <div ref={bar} class="flex">
                <Menubar menus={vec![
                    MenubarMenu::new("File").item(MenuItem::new("New Tab").on_select(move || p1.borrow_mut().push("new tab".into()))),
                    MenubarMenu::new("Edit").item(MenuItem::new("Undo").on_select(move || p2.borrow_mut().push("undo".into()))),
                ]} />
            </div>
        </div>
    });
    let b = bar.bounds().unwrap();
    // Triggers: 1px border + 4px padding, "File" ~ 40px wide, gap 4px.
    let file = pt(b.origin.x + 15.0, b.origin.y + 18.0);
    let edit = pt(b.origin.x + b.size.width - 20.0, b.origin.y + 18.0);
    cx.simulate_click(file, Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("down enter");
    settle(cx);
    assert_eq!(*picked.borrow(), vec!["new tab".to_string()], "File opened and its item was picked");
    // Open File, then move over Edit: the open menu follows the pointer.
    cx.simulate_click(file, Modifiers::default());
    settle(cx);
    cx.simulate_mouse_move(edit, None, Modifiers::default());
    settle(cx);
    settle(cx);
    cx.simulate_keystrokes("down enter");
    settle(cx);
    assert_eq!(*picked.borrow(), vec!["new tab".to_string(), "undo".to_string()], "Edit took over from File");
    // Clicking away closes.
    cx.simulate_click(file, Modifiers::default());
    settle(cx);
    cx.simulate_click(pt(b.origin.x + 400.0, b.origin.y + 300.0), Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("down enter");
    settle(cx);
    assert_eq!(picked.borrow().len(), 2);
}

#[gpui::test]
fn navigation_menu_opens_on_hover(cx: &mut TestAppContext) {
    use crate::ui::navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuLink};
    let nav = cx.update(|_| NodeRef::new());
    let panel = cx.update(|_| NodeRef::new());
    let shown = cx.update(|_| signal(false));
    let clicks = log();
    let c = clicks.clone();
    let cx = mount(cx, move || {
        let c = c.clone();
        let items = vec![
            NavigationMenuItem::new("Getting started").content(move || {
                shown.set(true);
                on_cleanup(move || shown.set(false));
                <div ref={panel} class="w-[300px]">
                <NavigationMenuLink title="Introduction" description="Re-usable components." />
            </div>}),
            NavigationMenuItem::new("Docs").on_click(move |_| c.borrow_mut().push("docs".into())),
        ];
        <div class="p-8 flex"><div ref={nav} class="flex"><NavigationMenu items={items} /></div></div>
    });
    let n = nav.bounds().unwrap();
    cx.simulate_mouse_move(pt(n.origin.x + 30.0, n.origin.y + 18.0), None, Modifiers::default());
    settle(cx);
    // The hover delays run on rsx's `sleep`, i.e. GPUI's (test) clock.
    cx.executor().advance_clock(std::time::Duration::from_millis(300));
    settle(cx);
    assert!(shown.get_untracked(), "hovering the trigger opens its panel");
    let p = panel.bounds().unwrap();
    assert!(p.origin.y >= n.origin.y + 36.0, "panel below the trigger: {p:?}");
    // Moving away closes it after the delay.
    cx.simulate_mouse_move(pt(n.origin.x + 600.0, n.origin.y + 400.0), None, Modifiers::default());
    settle(cx);
    cx.executor().advance_clock(std::time::Duration::from_millis(400));
    settle(cx);
    assert!(!shown.get_untracked(), "leaving closes the panel");
    // "Docs" is a plain link.
    cx.simulate_click(pt(n.origin.x + n.size.width - 30.0, n.origin.y + 18.0), Modifiers::default());
    settle(cx);
    assert_eq!(*clicks.borrow(), vec!["docs".to_string()]);
}

#[gpui::test]
fn select_picks_with_mouse_and_keyboard(cx: &mut TestAppContext) {
    use crate::ui::select::{Select, SelectGroup, SelectItem};
    let trigger = cx.update(|_| NodeRef::new());
    let changes = log();
    let ch = changes.clone();
    let cx = mount(cx, move || {
        let ch = ch.clone();
        <div class="p-8 flex">
            <div ref={trigger} class="flex">
                <Select
                    class="w-[180px]"
                    placeholder="Select a fruit"
                    groups={vec![SelectGroup::new("Fruits")
                        .item(SelectItem::new("apple", "Apple"))
                        .item(SelectItem::new("banana", "Banana").disabled(true))
                        .item(SelectItem::new("grapes", "Grapes"))]}
                    onChange={move |v: &SharedString| ch.borrow_mut().push(v.to_string())}
                />
            </div>
        </div>
    });
    let tb = trigger.bounds().unwrap();
    assert!((tb.size.width - 180.0).abs() < 1.0 && (tb.size.height - 36.0).abs() < 1.0, "trigger size {tb:?}");
    // Open, ↓ to Apple, ↓ skips disabled Banana to Grapes, Enter picks.
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("down down enter");
    settle(cx);
    assert_eq!(*changes.borrow(), vec!["grapes".to_string()]);
    // Reopen: the list starts on the selected item; ↑ goes to Apple.
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("up enter");
    settle(cx);
    assert_eq!(*changes.borrow(), vec!["grapes".to_string(), "apple".to_string()]);
    // Click an item: list is below the trigger (4px), 1px border + 4px padding,
    // group label 28px, then 32px rows: "Grapes" is the third.
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    let y = tb.origin.y + tb.size.height + 4.0 + 5.0 + 28.0 + 32.0 * 2.0 + 16.0;
    cx.simulate_click(pt(tb.origin.x + 40.0, y), Modifiers::default());
    settle(cx);
    assert_eq!(changes.borrow().last().map(String::as_str), Some("grapes"));
    // Escape closes without changing.
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("escape");
    settle(cx);
    cx.simulate_keystrokes("enter");
    settle(cx);
    assert_eq!(changes.borrow().len(), 3);
}

#[gpui::test]
fn select_controlled_and_native_select(cx: &mut TestAppContext) {
    use crate::ui::native_select::{disabled_option, option, NativeSelect};
    use crate::ui::select::{Select, SelectItem};
    let value = cx.update(|_| signal(Some(SharedString::from("b"))));
    let native_value = cx.update(|_| signal(None::<SharedString>));
    let (controlled, native) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col gap-4 w-[300px]">
            <div ref={controlled} class="flex">
                <Select
                    value={value.get()}
                    onChange={move |v: &SharedString| value.set(Some(v.clone()))}
                    items={vec![SelectItem::new("a", "Alpha"), SelectItem::new("b", "Beta")]}
                />
            </div>
            <div ref={native} class="flex">
                <NativeSelect
                    options={vec![option("todo", "Todo"), disabled_option("x", "X"), option("done", "Done")]}
                    onChange={move |v: &SharedString| native_value.set(Some(v.clone()))}
                />
            </div>
        </div>
    });
    let cb = controlled.bounds().unwrap();
    cx.simulate_click(pt(cb.origin.x + 20.0, cb.origin.y + 18.0), Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("up enter");
    settle(cx);
    assert_eq!(value.get_untracked().as_deref(), Some("a"));
    // NativeSelect fills its row and starts on the first option.
    let nb = native.bounds().unwrap();
    assert!((nb.size.width - 236.0).abs() < 1.0, "native select fills its 236px row: {nb:?}");
    cx.simulate_click(center(native), Modifiers::default());
    settle(cx);
    cx.simulate_keystrokes("down enter");
    settle(cx);
    assert_eq!(native_value.get_untracked().as_deref(), Some("done"), "↓ from Todo skips the disabled option");
}

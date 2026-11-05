#![cfg(test)]
//! Search: the command palette, its dialog, and the combobox on GPUI's test platform.

use gpui::{Modifiers, TestAppContext};
use rsx::prelude::*;

use crate::tests::{center, mount, settle};
use crate::ui::command::{CommandGroup, CommandItem};
use crate::ui::icon::IconName;

fn groups() -> Vec<CommandGroup> {
    vec![
        CommandGroup::new("Suggestions")
            .item(CommandItem::new("Calendar").icon(IconName::Calendar))
            .item(CommandItem::new("Search Emoji").icon(IconName::Search))
            .item(CommandItem::new("Calculator").icon(IconName::Terminal).disabled(true)),
        CommandGroup::new("Settings")
            .item(CommandItem::new("Profile").icon(IconName::User).shortcut("⌘P"))
            .item(CommandItem::new("Billing").value("billing").keywords(["invoice"]).shortcut("⌘B")),
    ]
}

fn type_text(cx: &mut gpui::VisualTestContext, text: &str) {
    cx.simulate_input(text);
    settle(cx);
}

fn key(cx: &mut gpui::VisualTestContext, keys: &str) {
    cx.simulate_keystrokes(keys);
    settle(cx);
}

fn clear(cx: &mut gpui::VisualTestContext) {
    key(cx, "cmd-a backspace");
}

#[gpui::test]
fn command_filters_navigates_and_selects(cx: &mut TestAppContext) {
    use crate::ui::command::Command;
    let picked = cx.update(|_| signal(Vec::<String>::new()));
    let item_ran = cx.update(|_| signal(0));
    let root = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        let mut groups = groups();
        groups[1] = CommandGroup::new("Settings")
            .item(CommandItem::new("Profile").on_select(move || item_ran.update(|n| *n += 1)))
            .item(CommandItem::new("Billing").value("billing").keywords(["invoice"]));
        <div class="p-8">
            <div ref={root} class="w-[450px]">
                <Command groups={groups} onSelect={move |v: &SharedString| picked.update(|p| p.push(v.to_string()))} />
            </div>
        </div>
    });
    let last = || picked.get_untracked().last().cloned();

    // The search field has the focus from the start; Enter runs the first item.
    key(cx, "enter");
    assert_eq!(last().as_deref(), Some("Calendar"));
    // ↓ moves the highlight; a disabled item does nothing.
    key(cx, "down enter");
    assert_eq!(last().as_deref(), Some("Search Emoji"));
    key(cx, "down enter");
    assert_eq!(picked.get_untracked().len(), 2, "Calculator is disabled");
    // ↓ crosses groups, and the item's own handler runs too.
    key(cx, "down enter");
    assert_eq!(last().as_deref(), Some("Profile"));
    assert_eq!(item_ran.get_untracked(), 1);
    // ↑ wraps around from the top.
    key(cx, "up up up up enter");
    assert_eq!(last().as_deref(), Some("billing"));

    // Typing filters (fuzzy, keywords too) and starts again from the first match.
    type_text(cx, "invc");
    key(cx, "enter");
    assert_eq!(last().as_deref(), Some("billing"), "matched by keyword, reported by value");
    clear(cx);
    type_text(cx, "srch");
    key(cx, "enter");
    assert_eq!(last().as_deref(), Some("Search Emoji"));
    // Nothing matches: Enter does nothing.
    clear(cx);
    type_text(cx, "zzz");
    let count = picked.get_untracked().len();
    key(cx, "enter");
    assert_eq!(picked.get_untracked().len(), count);

    // Clicking an item selects it (hover highlights first), and the search
    // field keeps the focus.
    clear(cx);
    let top = root.bounds().unwrap().origin;
    // Header (36px + border), group padding (4px), heading (28px), then 32px rows.
    let second = gpui::point(gpui::px(top.x + 100.0), gpui::px(top.y + 37.0 + 4.0 + 28.0 + 32.0 + 16.0));
    cx.simulate_mouse_move(second, None, Modifiers::default());
    settle(cx);
    cx.simulate_click(second, Modifiers::default());
    settle(cx);
    assert_eq!(last().as_deref(), Some("Search Emoji"));
    type_text(cx, "cal");
    key(cx, "enter");
    assert_eq!(last().as_deref(), Some("Calendar"), "typing still reaches the search field");
}

#[gpui::test]
fn command_dialog_selects_closes_and_escapes(cx: &mut TestAppContext) {
    use crate::ui::command::CommandDialog;
    use crate::ui::dialog::DialogHandle;
    let handle = cx.update(|_| DialogHandle::new());
    let picked = cx.update(|_| signal(Vec::<String>::new()));
    let changes = cx.update(|_| signal(Vec::<bool>::new()));
    let cx = mount(cx, move || {
        <div class="p-8">
            <CommandDialog
                handle={handle}
                placeholder="Type a command or search..."
                groups={groups()}
                onSelect={move |v: &SharedString| picked.update(|p| p.push(v.to_string()))}
                onOpenChange={move |o: &bool| changes.update(|c| c.push(*o))}
            />
        </div>
    });
    cx.update(|_, _| handle.open());
    settle(cx);
    assert!(handle.is_open_untracked());

    // The search field inside the dialog has the focus.
    type_text(cx, "prof");
    key(cx, "enter");
    assert_eq!(picked.get_untracked(), vec!["Profile".to_string()]);
    assert!(!handle.is_open_untracked(), "selecting closes the palette");

    // Reopened: fresh query; Escape closes without selecting.
    cx.update(|_, _| handle.open());
    settle(cx);
    key(cx, "down");
    key(cx, "escape");
    assert!(!handle.is_open_untracked());
    assert_eq!(picked.get_untracked().len(), 1);
    assert_eq!(changes.get_untracked(), vec![true, false, true, false]);
}

#[gpui::test]
fn controlled_command_dialog_follows_open(cx: &mut TestAppContext) {
    use crate::ui::command::CommandDialog;
    let open = cx.update(|_| signal(false));
    let picked = cx.update(|_| signal(None::<String>));
    let cx = mount(cx, move || {
        <CommandDialog
            open={open}
            onOpenChange={move |o: &bool| open.set(*o)}
            groups={groups()}
            onSelect={move |v: &SharedString| picked.set(Some(v.to_string()))}
        />
    });
    cx.update(|_, _| open.set(true));
    settle(cx);
    key(cx, "enter");
    assert_eq!(picked.get_untracked().as_deref(), Some("Calendar"));
    assert!(!open.get_untracked(), "selecting reported the close");
}

#[gpui::test]
fn combobox_opens_filters_selects_and_toggles(cx: &mut TestAppContext) {
    use crate::ui::combobox::Combobox;
    let trigger = cx.update(|_| NodeRef::new());
    let changes = cx.update(|_| signal(Vec::<Option<String>>::new()));
    let cx = mount(cx, move || {
        <div class="flex items-start p-8">
            <div ref={trigger} class="flex">
                <Combobox
                    default_value={SharedString::from("next.js")}
                    items={crate::demos::combobox::frameworks().into_iter().map(|(v, l)| (v.into(), l.into())).collect::<Vec<_>>()}
                    onChange={move |v: &Option<SharedString>| changes.update(|c| c.push(v.as_ref().map(|s| s.to_string())))}
                />
            </div>
        </div>
    });
    let open = |cx: &mut gpui::VisualTestContext| {
        cx.simulate_mouse_move(center(trigger), None, Modifiers::default());
        settle(cx);
        cx.simulate_click(center(trigger), Modifiers::default());
        settle(cx);
    };

    // Opens with the focus in its search field; Enter picks the first match.
    open(cx);
    type_text(cx, "svelte");
    key(cx, "enter");
    assert_eq!(changes.get_untracked(), vec![Some("sveltekit".to_string())]);

    // Picking the current value again clears it.
    open(cx);
    type_text(cx, "svelte");
    key(cx, "enter");
    assert_eq!(changes.get_untracked().last(), Some(&None));

    // Escape closes without a change (focus is back on the trigger, where
    // Enter reopens it with an empty search: Enter then picks the first item).
    open(cx);
    type_text(cx, "astro");
    key(cx, "escape");
    assert_eq!(changes.get_untracked().len(), 2);
    key(cx, "enter");
    key(cx, "enter");
    assert_eq!(changes.get_untracked().last(), Some(&Some("next.js".to_string())), "keyboard reopen, fresh query");
}

#[gpui::test]
fn controlled_combobox_reports_and_follows_value(cx: &mut TestAppContext) {
    use crate::ui::combobox::Combobox;
    let trigger = cx.update(|_| NodeRef::new());
    let value = cx.update(|_| signal(Some(SharedString::from("remix"))));
    let reported = cx.update(|_| signal(Vec::<Option<SharedString>>::new()));
    let cx = mount(cx, move || {
        <div class="flex items-start p-8">
            <div ref={trigger} class="flex">
                <Combobox
                    value={value}
                    items={crate::demos::combobox::frameworks().into_iter().map(|(v, l)| (v.into(), l.into())).collect::<Vec<_>>()}
                    onChange={move |v: &Option<SharedString>| reported.update(|r| r.push(v.clone()))}
                />
            </div>
        </div>
    });
    // The caller moves the value; picking it again then clears it.
    cx.update(|_, _| value.set(Some("astro".into())));
    settle(cx);
    cx.simulate_mouse_move(center(trigger), None, Modifiers::default());
    settle(cx);
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    type_text(cx, "astro");
    key(cx, "enter");
    assert_eq!(reported.get_untracked(), vec![None]);
    // Controlled: the value only changes when the caller says so.
    assert_eq!(value.get_untracked().as_deref(), Some("astro"));
}

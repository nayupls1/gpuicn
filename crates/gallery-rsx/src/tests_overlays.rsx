#![cfg(test)]
//! Overlays: dialogs, sheets, drawers, toasts, hover cards and the carousel
//! on GPUI's test platform.

use std::time::Duration;

use gpui::{Modifiers, TestAppContext};
use rsx::prelude::*;

use crate::tests::{center, mount, settle};

fn corner() -> gpui::Point<gpui::Pixels> {
    gpui::point(gpui::px(4.0), gpui::px(4.0))
}

/// Hover, then click: components that restyle on hover rebuild their
/// element, so a press must not be the first event at a new position.
fn click(cx: &mut gpui::VisualTestContext, at: gpui::Point<gpui::Pixels>) {
    cx.simulate_mouse_move(at, None, Modifiers::default());
    settle(cx);
    cx.simulate_click(at, Modifiers::default());
    settle(cx);
}

/// Advances the test clock (rsx's `sleep` runs on GPUI's timer), then draws.
fn wait(cx: &mut gpui::VisualTestContext, ms: u64) {
    cx.executor().advance_clock(Duration::from_millis(ms));
    settle(cx);
}

#[gpui::test]
fn dialog_opens_from_trigger_and_closes_on_escape_and_backdrop(cx: &mut TestAppContext) {
    use crate::ui::dialog::{Dialog, DialogHandle};
    let handle = cx.update(|_| DialogHandle::new());
    let trigger = cx.update(|_| NodeRef::new());
    let marker = cx.update(|_| NodeRef::new());
    let changes = cx.update(|_| signal(Vec::<bool>::new()));
    let cx = mount(cx, move || {
        <div class="p-8">
            <Dialog
                handle={handle}
                trigger={<div ref={trigger} class="p-2">Open</div>}
                title="Edit profile"
                content={move |_| <div ref={marker} class="h-8">body</div>}
                onOpenChange={move |o: &bool| changes.update(|c| c.push(*o))}
            />
        </div>
    });
    assert!(marker.bounds().is_none());

    click(cx, center(trigger));
    assert!(handle.is_open_untracked());
    assert!(marker.bounds().is_some(), "the panel was laid out in the portal");
    assert!(handle.layer_ref().is_focused_untracked(), "focus moved into the dialog");

    // A press inside the panel doesn't close it.
    click(cx, center(marker));
    assert!(handle.is_open_untracked());

    cx.simulate_keystrokes("escape");
    settle(cx);
    assert!(!handle.is_open_untracked());
    assert!(handle.trigger_ref().is_focused_untracked(), "focus returned to the trigger");

    // Backdrop press closes.
    click(cx, center(trigger));
    assert!(handle.is_open_untracked());
    click(cx, corner());
    assert!(!handle.is_open_untracked());
    assert_eq!(changes.get_untracked(), vec![true, false, true, false]);
}

#[gpui::test]
fn controlled_dialog_follows_open_and_reports_close(cx: &mut TestAppContext) {
    use crate::ui::dialog::{Dialog, DialogHandle};
    let open = cx.update(|_| signal(false));
    let handle = cx.update(|_| DialogHandle::new());
    let marker = cx.update(|_| NodeRef::new());
    let close = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <Dialog
            handle={handle}
            open={open}
            onOpenChange={move |o: &bool| open.set(*o)}
            show_close_button={false}
            content={move |_| <div ref={marker}>body</div>}
            footer={move |dialog: DialogHandle| <div ref={close} class="p-2" onClick={move |_| dialog.close()}>Close</div>}
        />
    });
    assert!(marker.bounds().is_none());
    cx.update(|_, _| open.set(true));
    settle(cx);
    assert!(marker.bounds().is_some());
    click(cx, center(close));
    assert!(!open.get_untracked(), "on_open_change(false) updated the caller's signal");
    assert!(!handle.is_open_untracked());
}

#[gpui::test]
fn alert_dialog_ignores_backdrop_and_closes_on_escape(cx: &mut TestAppContext) {
    use crate::ui::alert_dialog::AlertDialog;
    use crate::ui::dialog::DialogHandle;
    let handle = cx.update(|_| DialogHandle::new());
    let trigger = cx.update(|_| NodeRef::new());
    let acted = cx.update(|_| signal(0));
    let cx = mount(cx, move || {
        <div class="p-8">
            <AlertDialog
                handle={handle}
                trigger={<div ref={trigger} class="p-2">Delete</div>}
                title="Are you absolutely sure?"
                action="Continue"
                onAction={move |_| acted.update(|n| *n += 1)}
            />
        </div>
    });
    click(cx, center(trigger));
    assert!(handle.is_open_untracked());
    click(cx, corner());
    assert!(handle.is_open_untracked(), "the backdrop doesn't dismiss an alert dialog");

    cx.simulate_keystrokes("escape");
    settle(cx);
    assert!(!handle.is_open_untracked(), "Escape still closes it");
    assert_eq!(acted.get_untracked(), 0);
}

#[gpui::test]
fn alert_dialog_buttons_close_and_call_handlers(cx: &mut TestAppContext) {
    use crate::ui::alert_dialog::AlertDialog;
    use crate::ui::dialog::DialogHandle;
    let handle = cx.update(|_| DialogHandle::new());
    let acted = cx.update(|_| signal(0));
    let cancelled = cx.update(|_| signal(0));
    let cx = mount(cx, move || {
        // A fixed-size panel with the footer at its bottom, so the buttons
        // can be found by position: "Yes" bottom right, "No" left of it.
        <AlertDialog
            handle={handle}
            class="h-[200px] justify-between"
            title="Sure?"
            cancel="No"
            action="Yes"
            onAction={move |_| acted.update(|n| *n += 1)}
            onCancel={move |_| cancelled.update(|n| *n += 1)}
        />
    });
    let buttons = |handle: DialogHandle| {
        let layer = handle.layer_ref().bounds().unwrap();
        let right = layer.origin.x + layer.size.width / 2.0 + 256.0 - 24.0;
        let y = layer.origin.y + layer.size.height / 2.0 + 100.0 - 24.0 - 18.0;
        // "Yes" is ~50px wide, then an 8px gap, then "No".
        (gpui::point(gpui::px(right - 12.0), gpui::px(y)), gpui::point(gpui::px(right - 50.0 - 8.0 - 12.0), gpui::px(y)))
    };
    cx.update(|_, _| handle.open());
    wait(cx, 300);
    let (yes, _) = buttons(handle);
    click(cx, yes);
    assert_eq!(acted.get_untracked(), 1);
    assert!(!handle.is_open_untracked());

    cx.update(|_, _| handle.open());
    wait(cx, 300);
    let (_, no) = buttons(handle);
    click(cx, no);
    assert_eq!(cancelled.get_untracked(), 1);
    assert_eq!(acted.get_untracked(), 1);
    assert!(!handle.is_open_untracked());
}

#[gpui::test]
fn sheet_and_drawer_open_and_close_on_backdrop(cx: &mut TestAppContext) {
    use crate::ui::dialog::DialogHandle;
    use crate::ui::drawer::Drawer;
    use crate::ui::sheet::Sheet;
    use crate::ui::utils::Side;
    let sheet = cx.update(|_| DialogHandle::new());
    let drawer = cx.update(|_| DialogHandle::new());
    let sheet_body = cx.update(|_| NodeRef::new());
    let drawer_body = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 flex gap-2">
            <Sheet handle={sheet} side={Side::Right} title="Edit" content={move |_| <div ref={sheet_body}>sheet</div>} />
            <Drawer handle={drawer} title="Goal" content={move |_| <div ref={drawer_body}>drawer</div>} />
        </div>
    });
    cx.update(|_, _| sheet.open());
    settle(cx);
    let body = sheet_body.bounds().expect("sheet content laid out");
    let viewport = sheet.layer_ref().bounds().unwrap();
    assert!(body.origin.x > viewport.size.width / 2.0, "a right sheet sits on the right edge");
    click(cx, corner());
    assert!(!sheet.is_open_untracked());

    cx.update(|_, _| drawer.open());
    settle(cx);
    let body = drawer_body.bounds().expect("drawer content laid out");
    let viewport = drawer.layer_ref().bounds().unwrap();
    assert!(body.origin.y > viewport.size.height / 2.0, "the drawer sits at the bottom");
    cx.simulate_keystrokes("escape");
    settle(cx);
    assert!(!drawer.is_open_untracked());
}

#[gpui::test]
fn toasts_show_update_dismiss_and_expire(cx: &mut TestAppContext) {
    use crate::ui::sonner::{dismiss, toast, toasts, update, Toast, Toaster};
    let cx = mount(cx, move || <div class="size-full"><Toaster /></div>);
    let id = cx.update(|_, _| toast("Event has been created"));
    settle(cx);
    assert_eq!(cx.update(|_, _| toasts().len()), 1);

    // update() keeps the toast, replaces its content.
    cx.update(|_, _| update(id, Toast::new("Updated").duration(Duration::from_millis(150))));
    settle(cx);
    assert_eq!(cx.update(|_, _| toasts()[0].title().to_string()), "Updated");
    wait(cx, 400);
    assert!(cx.update(|_, _| toasts().is_empty()), "the toast expired");

    // dismiss()
    let id = cx.update(|_, _| toast("Saved"));
    settle(cx);
    cx.update(|_, _| dismiss(id));
    settle(cx);
    assert!(cx.update(|_, _| toasts().is_empty()));

    // A loading toast stays.
    cx.update(|_, _| toast(Toast::new("Uploading…").loading()));
    wait(cx, 100);
    assert_eq!(cx.update(|_, _| toasts().len()), 1);
}

#[gpui::test]
fn toast_action_runs_and_dismisses(cx: &mut TestAppContext) {
    use crate::ui::sonner::{toast, toasts, Toast, ToasterPosition, Toaster};
    let undone = cx.update(|_| signal(0));
    let cx = mount(cx, move || <div class="size-full"><Toaster position={ToasterPosition::TopLeft} /></div>);
    cx.update(|_, _| toast(Toast::new("Event has been created").action("Undo", move || undone.update(|n| *n += 1))));
    wait(cx, 300);
    // Top-left toaster: the card starts at (24, 24) and is 356px wide; the
    // action button is at its right edge, 16px in, vertically centered.
    let card_height = 52.0;
    let at = gpui::point(gpui::px(24.0 + 356.0 - 16.0 - 12.0), gpui::px(24.0 + card_height / 2.0));
    click(cx, at);
    assert_eq!(undone.get_untracked(), 1, "the action ran");
    assert!(cx.update(|_, _| toasts().is_empty()), "and dismissed the toast");
}

#[gpui::test]
fn hover_card_opens_on_hover(cx: &mut TestAppContext) {
    use crate::ui::hover_card::HoverCard;
    let trigger = cx.update(|_| NodeRef::new());
    let marker = cx.update(|_| NodeRef::new());
    // How many cards exist (NodeRef bounds outlive their element).
    let shown = cx.update(|_| signal(0));
    let cx = mount(cx, move || {
        <div class="p-8">
            <HoverCard
                open_delay={0u64}
                close_delay={0u64}
                trigger={<div ref={trigger} class="p-2">@nextjs</div>}
                content={move || {
                    shown.update(|n| *n += 1);
                    on_cleanup(move || shown.update(|n| *n -= 1));
                    <div ref={marker}>card</div>
                }}
            />
        </div>
    });
    assert_eq!(shown.get_untracked(), 0);
    cx.simulate_mouse_move(center(trigger), None, Modifiers::default());
    settle(cx);
    assert_eq!(shown.get_untracked(), 1, "the card opened");
    assert!(marker.bounds().is_some(), "and was laid out");
    cx.simulate_mouse_move(gpui::point(gpui::px(900.0), gpui::px(700.0)), None, Modifiers::default());
    settle(cx);
    assert_eq!(shown.get_untracked(), 0, "and closed when the pointer left");
}

#[gpui::test]
fn carousel_moves_and_clamps_or_loops(cx: &mut TestAppContext) {
    use crate::ui::carousel::Carousel;
    let changes = cx.update(|_| signal(Vec::<usize>::new()));
    let looped = cx.update(|_| signal(Vec::<usize>::new()));
    let root = cx.update(|_| NodeRef::new());
    let root2 = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="flex flex-col gap-8 px-16 py-8">
            <div ref={root} class="flex">
                <Carousel class="w-[300px]" items={(1..=3).map(|n| <div class="h-10">{n.to_string()}</div>).collect::<Vec<_>>()} onChange={move |i: &usize| changes.update(|c| c.push(*i))} />
            </div>
            <div ref={root2} class="flex">
                <Carousel looping class="w-[300px]" items={(1..=3).map(|n| <div class="h-10">{n.to_string()}</div>).collect::<Vec<_>>()} onChange={move |i: &usize| looped.update(|c| c.push(*i))} />
            </div>
        </div>
    });
    // The buttons are 32px, 48px outside the carousel, vertically centered.
    let buttons = |r: NodeRef| {
        let b = r.bounds().unwrap();
        let y = b.origin.y + b.size.height / 2.0;
        (gpui::point(gpui::px(b.origin.x - 48.0 + 16.0), gpui::px(y)), gpui::point(gpui::px(b.origin.x + 300.0 + 48.0 - 16.0), gpui::px(y)))
    };
    let (prev, next) = buttons(root);
    click(cx, prev);
    assert!(changes.get_untracked().is_empty(), "prev is disabled on the first slide");
    for _ in 0..3 {
        click(cx, next);
    }
    assert_eq!(changes.get_untracked(), vec![1, 2], "next stops at the last slide");

    let (prev, _) = buttons(root2);
    click(cx, prev);
    assert_eq!(looped.get_untracked(), vec![2], "looping wraps around");
}

// rsx gap: a `<Portal>` inside another `<Portal>` is drawn *below* it
// (rsx-gpui collects the inner portal first and both layers share priority
// 1), so a popover, select or tooltip opened from a dialog is hidden under
// the dialog and can't be clicked.
#[gpui::test]

fn popover_inside_dialog_is_drawn_above_it(cx: &mut TestAppContext) {
    use crate::ui::dialog::{Dialog, DialogHandle};
    use crate::ui::popover::{Popover, PopoverHandle};
    let dialog = cx.update(|_| DialogHandle::new());
    let popover = cx.update(|_| PopoverHandle::new());
    let trigger = cx.update(|_| NodeRef::new());
    let item = cx.update(|_| NodeRef::new());
    let clicked = cx.update(|_| signal(0));
    let cx = mount(cx, move || {
        <Dialog
            handle={dialog}
            content={move |_| <Popover
                handle={popover}
                trigger={<div ref={trigger} class="p-2">Pick</div>}
                content={move |_| <div ref={item} class="p-2" onClick={move |_| clicked.update(|n| *n += 1)}>Item</div>}
            />}
        />
    });
    cx.update(|_, _| dialog.open());
    settle(cx);
    click(cx, center(trigger));
    assert!(popover.is_open_untracked());
    click(cx, center(item));
    assert_eq!(clicked.get_untracked(), 1, "the popover's content receives the click");
    assert!(dialog.is_open_untracked());
}


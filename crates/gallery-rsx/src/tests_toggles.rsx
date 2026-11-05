#![cfg(test)]
//! GPUI tests for the toggles batch: checkbox, switch, radio group, toggle,
//! toggle group, slider, aspect ratio and avatar.

use gpui::{Modifiers, MouseButton, TestAppContext};
use rsx::prelude::*;

use crate::tests::{center, mount, settle};
use crate::ui::aspect_ratio::AspectRatio;
use crate::ui::checkbox::Checkbox;
use crate::ui::icon::IconName;
use crate::ui::radio_group::{RadioGroup, RadioGroupItem};
use crate::ui::slider::Slider;
use crate::ui::switch::Switch;
use crate::ui::toggle::Toggle;
use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

/// A full key press. `simulate_keystrokes` only sends key-down events, and
/// GPUI turns Space / Enter into a click on the key-up.
fn press(cx: &mut gpui::VisualTestContext, key: &str) {
    cx.simulate_keystrokes(key);
    cx.simulate_event(gpui::KeyUpEvent { keystroke: gpui::Keystroke::parse(key).unwrap() });
    settle(cx);
}

/// A point `fx` of the way across `r` (0 = left edge, 1 = right edge), vertically centered.
fn across(r: NodeRef, fx: f32) -> gpui::Point<gpui::Pixels> {
    let b = r.bounds().expect("the element was laid out");
    gpui::point(gpui::px(b.origin.x + b.size.width * fx), gpui::px(b.origin.y + b.size.height / 2.0))
}

#[gpui::test]
fn checkbox_toggles_on_click_and_space(cx: &mut TestAppContext) {
    let checked = cx.update(|_| signal(false));
    let row = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col">
            <div ref={row}><Checkbox label="Accept" onChange={move |c: &bool| checked.set(*c)} /></div>
        </div>
    });
    cx.simulate_click(center(row), Modifiers::default());
    settle(cx);
    assert!(checked.get_untracked(), "a click checks it");
    // It kept focus while its state changed, so Space toggles it again (twice).
    press(cx, "space");
    assert!(!checked.get_untracked(), "Space unchecks it");
    press(cx, "enter");
    assert!(checked.get_untracked(), "Enter checks it");
}

#[gpui::test]
fn checkbox_controlled_and_disabled(cx: &mut TestAppContext) {
    let reported = cx.update(|_| signal(None::<bool>));
    let disabled_changes = cx.update(|_| signal(0));
    let (a, b) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col gap-4">
            {/* Controlled and never updated by its parent: reports, but stays unchecked. */}
            <div ref={a}><Checkbox checked={false} label="Controlled" onChange={move |c: &bool| reported.set(Some(*c))} /></div>
            <div ref={b}><Checkbox disabled={true} label="Disabled" onChange={move |_| disabled_changes.update(|n| *n += 1)} /></div>
        </div>
    });
    cx.simulate_click(center(a), Modifiers::default());
    settle(cx);
    assert_eq!(reported.get_untracked(), Some(true));
    cx.simulate_click(center(a), Modifiers::default());
    settle(cx);
    assert_eq!(reported.get_untracked(), Some(true), "still unchecked, so it asks for `true` again");
    cx.simulate_click(center(b), Modifiers::default());
    settle(cx);
    assert_eq!(disabled_changes.get_untracked(), 0, "a disabled checkbox ignores clicks");
}

#[gpui::test]
fn switch_toggles_on_click_and_enter(cx: &mut TestAppContext) {
    let on = cx.update(|_| signal(true));
    let row = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col">
            <div ref={row}><Switch default_checked label="Wi-Fi" onChange={move |c: &bool| on.set(*c)} /></div>
        </div>
    });
    cx.simulate_click(center(row), Modifiers::default());
    settle(cx);
    assert!(!on.get_untracked());
    press(cx, "enter");
    assert!(on.get_untracked());
}

#[gpui::test]
fn radio_group_selects_on_click_and_arrows(cx: &mut TestAppContext) {
    let value = cx.update(|_| signal(None::<SharedString>));
    let group = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col">
            <div ref={group}>
                <RadioGroup
                    default_value="comfortable"
                    onChange={move |v: &SharedString| value.set(Some(v.clone()))}
                    items={vec![
                        RadioGroupItem::new("default", "Default"),
                        RadioGroupItem::new("comfortable", "Comfortable"),
                        RadioGroupItem::new("compact", "Compact").disabled(true),
                    ]}
                />
            </div>
        </div>
    });
    // Three 16px rows, 12px apart: the first row's middle is 8px from the top.
    let b = group.bounds().unwrap();
    let row = |ix: f32| gpui::point(gpui::px(b.origin.x + 8.0), gpui::px(b.origin.y + 8.0 + ix * 28.0));
    cx.simulate_click(row(0.0), Modifiers::default());
    settle(cx);
    assert_eq!(value.get_untracked().as_deref(), Some("default"));
    cx.simulate_click(row(2.0), Modifiers::default());
    settle(cx);
    assert_eq!(value.get_untracked().as_deref(), Some("default"), "the disabled item can't be picked");
    cx.simulate_click(row(0.0), Modifiers::default());
    settle(cx);
    // Focus is on "Default": arrows move the selection, skipping the disabled item.
    cx.simulate_keystrokes("down");
    settle(cx);
    assert_eq!(value.get_untracked().as_deref(), Some("comfortable"));
    cx.simulate_keystrokes("down");
    settle(cx);
    assert_eq!(value.get_untracked().as_deref(), Some("default"), "wraps around past the disabled item");
    cx.simulate_keystrokes("up");
    settle(cx);
    assert_eq!(value.get_untracked().as_deref(), Some("comfortable"));
}

#[gpui::test]
fn toggle_and_toggle_group_change_value(cx: &mut TestAppContext) {
    let pressed = cx.update(|_| signal(false));
    let multi = cx.update(|_| signal(Vec::<SharedString>::new()));
    let single = cx.update(|_| signal(vec![SharedString::from("left")]));
    let (toggle, multi_ref, single_ref) = cx.update(|_| (NodeRef::new(), NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col gap-4 w-[200px]">
            <div ref={toggle} class="flex"><Toggle icon={IconName::Bold} onChange={move |p: &bool| pressed.set(*p)} /></div>
            <div ref={multi_ref} class="w-[108px]">
                <ToggleGroup multiple outline onChange={move |v: &Vec<SharedString>| multi.set(v.clone())} items={vec![
                    ToggleGroupItem::new("bold").icon(IconName::Bold),
                    ToggleGroupItem::new("italic").icon(IconName::Italic),
                    ToggleGroupItem::new("underline").icon(IconName::Underline),
                ]} />
            </div>
            <div ref={single_ref} class="w-[108px]">
                <ToggleGroup default_value={vec!["left".into()]} onChange={move |v: &Vec<SharedString>| single.set(v.clone())} items={vec![
                    ToggleGroupItem::new("left").icon(IconName::AlignLeft),
                    ToggleGroupItem::new("center").icon(IconName::AlignCenter),
                    ToggleGroupItem::new("right").icon(IconName::AlignRight),
                ]} />
            </div>
        </div>
    });
    // The toggle is 36px wide at the left of its row.
    let b = toggle.bounds().unwrap();
    let toggle_center = gpui::point(gpui::px(b.origin.x + 18.0), gpui::px(b.origin.y + 18.0));
    cx.simulate_click(toggle_center, Modifiers::default());
    settle(cx);
    assert!(pressed.get_untracked());
    press(cx, "space");
    assert!(!pressed.get_untracked(), "Space toggles the focused toggle");

    let item = |r: NodeRef, ix: f32| across(r, (ix + 0.5) / 3.0);
    cx.simulate_click(item(multi_ref, 0.0), Modifiers::default());
    settle(cx);
    cx.simulate_click(item(multi_ref, 2.0), Modifiers::default());
    settle(cx);
    assert_eq!(multi.get_untracked(), vec![SharedString::from("bold"), "underline".into()]);
    cx.simulate_click(item(multi_ref, 0.0), Modifiers::default());
    settle(cx);
    assert_eq!(multi.get_untracked(), vec![SharedString::from("underline")]);

    cx.simulate_click(item(single_ref, 1.0), Modifiers::default());
    settle(cx);
    assert_eq!(single.get_untracked(), vec![SharedString::from("center")], "single mode replaces the value");
    cx.simulate_click(item(single_ref, 1.0), Modifiers::default());
    settle(cx);
    assert!(single.get_untracked().is_empty(), "pressing the pressed item clears it");
}

#[gpui::test]
fn slider_follows_press_and_drag(cx: &mut TestAppContext) {
    let values = cx.update(|_| signal(vec![50.0f32]));
    let range = cx.update(|_| signal(vec![25.0f32, 75.0]));
    let (one, two) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    let cx = mount(cx, move || {
        <div class="p-8 flex flex-col gap-8 w-[264px]">
            <div ref={one}><Slider default_value={vec![50.0]} onChange={move |v: &Vec<f32>| values.set(v.clone())} /></div>
            <div ref={two}><Slider default_value={vec![25.0, 75.0]} step={5} onChange={move |v: &Vec<f32>| range.set(v.clone())} /></div>
        </div>
    });
    let none = Modifiers::default();
    // Press on the track: the thumb jumps there.
    cx.simulate_mouse_down(across(one, 0.75), MouseButton::Left, none);
    settle(cx);
    assert_eq!(values.get_untracked(), vec![75.0]);
    // Drag: moves keep reaching the slider, even outside it (full-window layer).
    cx.simulate_mouse_move(across(one, 0.2), MouseButton::Left, none);
    settle(cx);
    assert_eq!(values.get_untracked(), vec![20.0]);
    let mut far_right = across(one, 1.0);
    far_right.x += gpui::px(300.0);
    far_right.y += gpui::px(200.0);
    cx.simulate_mouse_move(far_right, MouseButton::Left, none);
    settle(cx);
    assert_eq!(values.get_untracked(), vec![100.0], "clamped to max");
    cx.simulate_mouse_up(far_right, MouseButton::Left, none);
    settle(cx);
    // Released: moving with the button held (a new gesture elsewhere) no longer drags.
    cx.simulate_mouse_move(across(one, 0.5), MouseButton::Left, none);
    settle(cx);
    assert_eq!(values.get_untracked(), vec![100.0], "the drag ended on mouse up");

    // Range: a press moves the closest thumb, snapped to the step.
    cx.simulate_mouse_down(across(two, 0.88), MouseButton::Left, none);
    settle(cx);
    assert_eq!(range.get_untracked(), vec![25.0, 90.0]);
    cx.simulate_mouse_up(across(two, 0.88), MouseButton::Left, none);
    settle(cx);
    cx.simulate_mouse_down(across(two, 0.1), MouseButton::Left, none);
    settle(cx);
    assert_eq!(range.get_untracked(), vec![10.0, 90.0]);
    cx.simulate_mouse_up(across(two, 0.1), MouseButton::Left, none);
    settle(cx);
    // The pressed thumb took focus and follows the arrow keys.
    cx.simulate_keystrokes("right");
    settle(cx);
    assert_eq!(range.get_untracked(), vec![15.0, 90.0]);
    cx.simulate_keystrokes("end");
    settle(cx);
    assert_eq!(range.get_untracked(), vec![100.0, 90.0]);
}

#[gpui::test]
fn aspect_ratio_sets_height_from_width(cx: &mut TestAppContext) {
    let r = cx.update(|_| NodeRef::new());
    mount(cx, move || {
        <div class="w-[320px] flex flex-col">
            <div ref={r}><AspectRatio ratio={16.0 / 9.0}><div class="size-full">16 / 9</div></AspectRatio></div>
        </div>
    });
    let b = r.bounds().unwrap();
    assert_eq!(b.size.width, 320.0);
    assert!((b.size.height - 180.0).abs() < 0.5, "height {} for a 16/9 box 320px wide", b.size.height);
}

#[gpui::test]
fn avatar_draws_its_image_at_the_avatar_size(cx: &mut TestAppContext) {
    use crate::ui::avatar::Avatar;
    let (r, rounded) = cx.update(|_| (NodeRef::new(), NodeRef::new()));
    mount(cx, move || {
        <div class="p-8 flex items-start gap-4">
            <div ref={r}><Avatar src={crate::demos::avatar_image(0)} fallback="CN" /></div>
            <div ref={rounded}><Avatar src={crate::demos::avatar_image(1)} fallback="ER" class="rounded-lg size-10" /></div>
        </div>
    });
    let b = r.bounds().unwrap();
    assert_eq!((b.size.width, b.size.height), (32.0, 32.0));
    let b = rounded.bounds().unwrap();
    assert_eq!((b.size.width, b.size.height), (40.0, 40.0), "the caller's size wins");
    assert!(std::path::Path::new(&crate::demos::avatar_image(0)).exists(), "rsx loads `<img src>` file paths");
}

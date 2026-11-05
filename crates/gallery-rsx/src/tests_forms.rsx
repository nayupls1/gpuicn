#![cfg(test)]
//! GPUI tests for the forms batch: input, textarea, input group, input OTP,
//! field and form.

use gpui::{Modifiers, TestAppContext};
use rsx::prelude::*;

use crate::tests::{center, mount, settle};
use crate::ui::form::{Form, FormField, FormValues};
use crate::ui::input::Input;
use crate::ui::input_group::{InputGroup, InputGroupAddon};
use crate::ui::input_otp::InputOtp;
use crate::ui::textarea::Textarea;
use crate::ui::icon::IconName;

#[gpui::test]
fn input_edits_its_bound_signal_and_keeps_focus(cx: &mut TestAppContext) {
    let text = cx.update(|_| signal(String::new()));
    let changes = cx.update(|_| signal(0usize));
    let submitted = cx.update(|_| signal(None::<String>));
    let input = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        crate::ui::init();
        <div class="p-8 w-[320px]">
            <Input
                text={text}
                node_ref={input}
                placeholder="Email"
                onChange={move |_: &String| changes.update(|n| *n += 1)}
                onSubmit={move |t: &String| submitted.set(Some(t.clone()))}
            />
        </div>
    });
    cx.simulate_click(center(input), Modifiers::default());
    settle(cx);
    assert!(input.is_focused_untracked());
    cx.simulate_input("hello");
    settle(cx);
    assert_eq!(text.get_untracked(), "hello");
    assert_eq!(changes.get_untracked(), 5);
    // The field wasn't rebuilt by the focus ring / border bindings.
    assert!(input.is_focused_untracked(), "typing keeps the focus");
    cx.simulate_keystrokes("backspace");
    settle(cx);
    assert_eq!(text.get_untracked(), "hell");
    cx.simulate_keystrokes("enter");
    settle(cx);
    assert_eq!(submitted.get_untracked().as_deref(), Some("hell"));
    // Writing the signal updates the field.
    cx.update(|_, _| text.set("new".into()));
    settle(cx);
    cx.simulate_input("!");
    settle(cx);
    assert_eq!(text.get_untracked(), "new!");
}

#[gpui::test]
fn controlled_input_follows_value(cx: &mut TestAppContext) {
    let name = cx.update(|_| signal(String::from("a")));
    let input = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 w-[320px]">
            <Input value={name.get()} node_ref={input} onChange={move |t: &String| name.set(t.to_uppercase())} />
        </div>
    });
    cx.simulate_click(center(input), Modifiers::default());
    settle(cx);
    cx.simulate_input("b");
    settle(cx);
    assert_eq!(name.get_untracked(), "AB");
}

#[gpui::test]
fn disabled_input_ignores_typing(cx: &mut TestAppContext) {
    let text = cx.update(|_| signal(String::from("x")));
    let disabled = cx.update(|_| signal(true));
    let input = cx.update(|_| NodeRef::new());
    let wrapper = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        crate::ui::init();
        <div class="p-8 w-[320px]" ref={wrapper}>
            <Input text={text} node_ref={input} disabled={disabled.get()} />
        </div>
    });
    cx.simulate_click(center(wrapper), Modifiers::default());
    settle(cx);
    cx.simulate_input("abc");
    settle(cx);
    assert_eq!(text.get_untracked(), "x");
    cx.update(|_, _| disabled.set(false));
    settle(cx);
    cx.simulate_click(center(input), Modifiers::default());
    settle(cx);
    cx.simulate_input("y");
    settle(cx);
    assert_eq!(text.get_untracked(), "xy");
}

#[gpui::test]
fn textarea_takes_newlines(cx: &mut TestAppContext) {
    let text = cx.update(|_| signal(String::new()));
    let area = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 w-[320px]"><Textarea text={text} node_ref={area} placeholder="Message" /></div>
    });
    cx.simulate_click(center(area), Modifiers::default());
    settle(cx);
    cx.simulate_input("a");
    cx.simulate_keystrokes("enter");
    cx.simulate_input("b");
    settle(cx);
    assert_eq!(text.get_untracked(), "a\nb");
    // rsx-gpui doesn't wrap long lines in a `<textarea>` yet, so no wrapping check.
}

#[gpui::test]
fn input_group_draws_the_focus_for_its_input(cx: &mut TestAppContext) {
    let text = cx.update(|_| signal(String::new()));
    let input = cx.update(|_| NodeRef::new());
    let group = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8 w-[380px]" ref={group}>
            <InputGroup start={InputGroupAddon::icon(IconName::Search)} end={InputGroupAddon::text("12 results")}>
                <Input text={text} node_ref={input} placeholder="Search..." />
            </InputGroup>
        </div>
    });
    let (g, i) = (group.bounds().unwrap(), input.bounds().unwrap());
    // The input sits between the addons, inside the group.
    assert!(i.origin.x > g.origin.x + 40.0 && i.size.width < g.size.width - 120.0, "{i:?} in {g:?}");
    cx.simulate_click(center(input), Modifiers::default());
    settle(cx);
    cx.simulate_input("rsx");
    settle(cx);
    assert_eq!(text.get_untracked(), "rsx");
}

#[gpui::test]
fn input_otp_fills_slots_and_completes(cx: &mut TestAppContext) {
    let code = cx.update(|_| signal(String::new()));
    let completed = cx.update(|_| signal(None::<String>));
    let otp = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8"><div class="w-[320px]" ref={otp}>
            <InputOtp
                length={6usize}
                groups={[3, 3]}
                onChange={move |c: &String| code.set(c.clone())}
                onComplete={move |c: &String| completed.set(Some(c.clone()))}
            />
        </div></div>
    });
    // The first slot (the wrapper is wider than the slots).
    let b = otp.bounds().unwrap();
    cx.simulate_click(gpui::point(gpui::px(b.origin.x + 18.0), gpui::px(b.origin.y + b.size.height / 2.0)), Modifiers::default());
    settle(cx);
    cx.simulate_input("12a3");
    settle(cx);
    assert_eq!(code.get_untracked(), "123", "letters are filtered out");
    cx.simulate_keystrokes("backspace");
    settle(cx);
    assert_eq!(code.get_untracked(), "12");
    // Moving the hidden caret doesn't matter: digits go to the next slot.
    cx.simulate_keystrokes("left left");
    cx.simulate_input("3456789");
    settle(cx);
    assert_eq!(code.get_untracked(), "123456");
    assert_eq!(completed.get_untracked().as_deref(), Some("123456"));
}

#[gpui::test]
fn form_validates_and_submits_on_enter(cx: &mut TestAppContext) {
    let submitted = cx.update(|_| signal(None::<FormValues>));
    let wrapper = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        crate::ui::init();
        <div class="p-8"><div class="w-[420px]" ref={wrapper}>
            <Form
                fields={vec![
                    FormField::text("username")
                        .label("Username")
                        .validate(|v| (v.chars().count() < 2).then(|| "Username must be at least 2 characters.".into())),
                    FormField::text("email").label("Email").default_value("m@example.com").required("Required"),
                ]}
                onSubmit={move |v: &FormValues| submitted.set(Some(v.clone()))}
            />
        </div></div>
    });
    let before = wrapper.bounds().unwrap().size.height;
    // The username input: below its 14px label and the 12px gap.
    let b = wrapper.bounds().unwrap();
    cx.simulate_click(gpui::point(gpui::px(b.origin.x + 60.0), gpui::px(b.origin.y + 14.0 + 12.0 + 18.0)), Modifiers::default());
    settle(cx);
    cx.simulate_input("a");
    cx.simulate_keystrokes("enter");
    settle(cx);
    assert!(submitted.get_untracked().is_none(), "invalid: not submitted");
    assert!(wrapper.bounds().unwrap().size.height > before, "the error message appeared");
    cx.simulate_input("b");
    cx.simulate_keystrokes("enter");
    settle(cx);
    let values = submitted.get_untracked().expect("submitted");
    assert_eq!(values.get("username").map(|s| s.as_ref()), Some("ab"));
    assert_eq!(values.get("email").map(|s| s.as_ref()), Some("m@example.com"));
    assert_eq!(wrapper.bounds().unwrap().size.height, before, "the error is gone");
}


#[gpui::test]
fn input_in_a_popover_keeps_the_focus(cx: &mut TestAppContext) {
    use crate::ui::popover::{Popover, PopoverHandle};
    let handle = cx.update(|_| PopoverHandle::new());
    let trigger = cx.update(|_| NodeRef::new());
    let input = cx.update(|_| NodeRef::new());
    let text = cx.update(|_| signal(String::new()));
    let cx = mount(cx, move || {
        <div class="p-8">
            <Popover
                handle={handle}
                trigger={<div ref={trigger} class="p-2">Open</div>}
                content={move |_| <div class="w-[240px]"><Input text={text} node_ref={input} /></div>}
            />
        </div>
    });
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    cx.simulate_click(center(input), Modifiers::default());
    settle(cx);
    // The popover's panel (it has a `ref`) would take the focus on click.
    assert!(input.is_focused_untracked());
    cx.simulate_input("42");
    settle(cx);
    assert_eq!(text.get_untracked(), "42");
    assert!(handle.is_open_untracked());
}

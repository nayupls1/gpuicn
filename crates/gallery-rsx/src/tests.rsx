#![cfg(test)]
//! Headless tests on GPUI's test platform: render every demo page, in light
//! and dark mode, and open the overlays.

use gpui::{Modifiers, TestAppContext, VisualTestContext};
use rsx::prelude::*;
use rsx_gpui::Host;

use crate::demos::DEMOS;
use crate::ui::theme::{Theme, ThemeMode};

pub fn center(r: NodeRef) -> gpui::Point<gpui::Pixels> {
    let b = r.bounds().expect("the element was laid out");
    gpui::point(gpui::px(b.origin.x + b.size.width / 2.0), gpui::px(b.origin.y + b.size.height / 2.0))
}

pub fn mount<E: IntoElement + 'static>(cx: &mut TestAppContext, app: impl FnOnce() -> E + 'static) -> &mut VisualTestContext {
    let (_host, cx) = cx.add_window_view(|window, cx| Host::new(app, window, cx));
    cx.update(|window, _| window.activate_window());
    settle(cx);
    cx
}

pub fn settle(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    cx.update(|window, _| window.refresh());
    cx.run_until_parked();
}

#[gpui::test]
fn renders_every_demo(cx: &mut TestAppContext) {
    let page = cx.update(|_| signal(0usize));
    // Spinners and pulses animate forever, which keeps the test executor from parking.
    crate::ui::utils::set_reduced_motion(true);
    let cx = mount(cx, move || crate::app::App::new().page(page));
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        cx.update(|_, _| Theme::change(mode));
        for (ix, (name, _)) in DEMOS.iter().enumerate() {
            eprintln!("rendering {name} ({mode:?})");
            cx.update(|_, _| page.set(ix));
            settle(cx);
        }
    }
}

#[gpui::test]
fn popover_opens_on_click_and_closes_on_escape(cx: &mut TestAppContext) {
    use crate::ui::popover::{Popover, PopoverHandle};
    let handle = cx.update(|_| PopoverHandle::new());
    let marker = cx.update(|_| NodeRef::new());
    let trigger = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="p-8">
            <Popover handle={handle} trigger={<div ref={trigger} class="p-2">Open</div>} content={move |_| <div ref={marker}>marker</div>} />
        </div>
    });
    assert!(marker.bounds().is_none());
    cx.simulate_click(center(trigger), Modifiers::default());
    settle(cx);
    assert!(handle.is_open_untracked());
    assert!(marker.bounds().is_some(), "the content was laid out in the portal");
    cx.simulate_keystrokes("escape");
    settle(cx);
    assert!(!handle.is_open_untracked());
}

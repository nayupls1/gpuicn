//! Headless tests: render every demo page and open the overlays.

use super::*;
use gpui::{TestAppContext, VisualTestContext};
use ui::{
    alert_dialog::AlertDialog,
    command::{CommandDialog, CommandGroup, CommandItem},
    dialog::Dialog,
    drawer::Drawer,
    dropdown_menu::{DropdownMenu, MenuBuilder, MenuItem},
    input::Input,
    popover::Popover,
    sheet::Sheet,
};

fn settle(cx: &mut VisualTestContext) {
    cx.run_until_parked();
    cx.update(|window, _| window.refresh());
    cx.run_until_parked();
}

#[gpui::test]
fn renders_every_demo(cx: &mut TestAppContext) {
    cx.update(ui::init);
    let (view, cx) = cx.add_window_view(|_, _| Gallery::new());
    for (ix, (name, _)) in demos::DEMOS.iter().enumerate() {
        eprintln!("rendering {name}");
        view.update(cx, |gallery, cx| gallery.select(ix, cx));
        settle(cx);
    }
    // And once more in dark mode.
    cx.update(|_, cx| Theme::change(ThemeMode::Dark, cx));
    for ix in 0..demos::DEMOS.len() {
        view.update(cx, |gallery, cx| gallery.select(ix, cx));
        settle(cx);
    }
}

/// Renders `$overlay` with the root view and asserts that the element tagged
/// `marker` inside it was laid out.
macro_rules! overlay_test {
    ($name:ident, |$marker:ident| $overlay:expr) => {
        #[gpui::test]
        fn $name(cx: &mut TestAppContext) {
            struct View;
            impl Render for View {
                fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                    let $marker = || div().debug_selector(|| "marker".into()).child("marker");
                    div().size_full().child($overlay).child(PortalHost::new())
                }
            }
            cx.update(ui::init);
            let (_view, cx) = cx.add_window_view(|_, _| View);
            settle(cx);
            assert!(cx.debug_bounds("marker").is_some(), "overlay content was not drawn");
        }
    };
}

overlay_test!(dialog_opens, |marker| Dialog::new("d")
    .open(true)
    .trigger(Button::new("t").label("Open"))
    .title("Title")
    .content(move |_, _, _| marker()));

overlay_test!(sheet_opens, |marker| Sheet::new("s")
    .open(true)
    .trigger(Button::new("t").label("Open"))
    .title("Title")
    .content(move |_, _, _| marker()));

overlay_test!(drawer_opens, |marker| Drawer::new("dr")
    .open(true)
    .trigger(Button::new("t").label("Open"))
    .content(move |_, _, _| marker()));

overlay_test!(alert_dialog_opens, |marker| div().child(marker()).child(
    AlertDialog::new("a").open(true).trigger(Button::new("t").label("Open")).title("Are you sure?").description("Really?")
));

overlay_test!(dropdown_opens_with_submenu, |marker| div().child(marker()).child(
    DropdownMenu::new("m")
        .open(true)
        .trigger(Button::new("t").label("Open"))
        .item(MenuItem::new("A"))
        .submenu("More", vec![MenuItem::new("B").into()])
));

overlay_test!(popover_with_nested_menu, |marker| Popover::new("p")
    .open(true)
    .trigger(Button::new("t").label("Open"))
    .content(move |_, _| {
        div().child(marker()).child(
            DropdownMenu::new("nested").open(true).trigger(Button::new("t2").label("menu")).item(MenuItem::new("A")),
        )
    }));

overlay_test!(command_dialog_opens, |marker| div()
    .child(marker())
    .child(CommandDialog::new("c").open(true).group(CommandGroup::new("Suggestions").item(CommandItem::new("Calendar")))));

#[gpui::test]
fn input_accepts_typing(cx: &mut TestAppContext) {
    struct View(gpui::Entity<ui::input::InputState>);
    impl Render for View {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(Input::from_state(&self.0))
        }
    }
    cx.update(ui::init);
    let (view, cx) = cx.add_window_view(|window, cx| View(cx.new(|cx| ui::input::InputState::new(window, cx))));
    settle(cx);
    let state = view.read_with(cx, |view, _| view.0.clone());
    cx.update(|window, cx| state.read(cx).focus(window));
    settle(cx);
    cx.simulate_input("hello");
    settle(cx);
    assert_eq!(state.read_with(cx, |state, _| state.text().to_string()), "hello");
}

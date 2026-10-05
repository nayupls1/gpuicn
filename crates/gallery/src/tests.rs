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

mod hover {
    use super::*;
    use gpui::{Modifiers, Point, point};
    use std::time::Duration;
    use ui::{hover_card::HoverCard, popover::PopoverTrigger, utils::Side};

    fn mv(cx: &mut VisualTestContext, at: Point<gpui::Pixels>) {
        cx.simulate_mouse_move(at, None, Modifiers::none());
        settle(cx);
    }

    fn wait(cx: &mut VisualTestContext, ms: u64) {
        cx.executor().advance_clock(Duration::from_millis(ms));
        settle(cx);
    }

    thread_local! {
        static PAINTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }

    /// Whether the marker is drawn in a fresh frame (`debug_bounds` is never
    /// cleared by gpui, so it can't tell us that something disappeared).
    fn shown(cx: &mut VisualTestContext) -> bool {
        PAINTED.with(|p| p.set(false));
        settle(cx);
        PAINTED.with(|p| p.get())
    }

    fn setup(cx: &mut TestAppContext, build: fn() -> AnyElement) -> &mut VisualTestContext {
        struct View(fn() -> AnyElement);
        impl Render for View {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div().size_full().p_20().child((self.0)()).child(PortalHost::new())
            }
        }
        cx.update(ui::init);
        let (_view, cx) = cx.add_window_view(move |_, _| View(build));
        settle(cx);
        cx
    }

    fn marker() -> gpui::Div {
        div()
            .debug_selector(|| "marker".into())
            .relative()
            .w(px(100.))
            .h(px(40.))
            .child("marker")
            .child(gpui::canvas(|_, _, _| {}, |_, _, _, _| PAINTED.with(|p| p.set(true))).absolute().size_full())
    }

    fn trigger() -> gpui::Div {
        div().debug_selector(|| "trigger".into()).w(px(80.)).h(px(30.)).child("trigger")
    }

    use gpui::AnyElement;

    #[gpui::test]
    fn hover_card_opens_stays_and_closes(cx: &mut TestAppContext) {
        let cx = setup(cx, || HoverCard::new("hc").trigger(trigger()).content(|_, _| marker()).into_any_element());
        let t = cx.debug_bounds("trigger").unwrap();
        mv(cx, t.center());
        assert!(!shown(cx), "opens only after the delay");
        wait(cx, 800);
        assert!(shown(cx), "open after hovering the trigger");

        // Move onto the content: stays open.
        let m = cx.debug_bounds("marker").unwrap();
        mv(cx, m.center());
        wait(cx, 500);
        assert!(shown(cx), "stays open while hovering the content");

        // Leave everything: closes after the close delay.
        mv(cx, point(px(5.), px(5.)));
        wait(cx, 400);
        assert!(!shown(cx), "closes after leaving");

        // And it can open again.
        mv(cx, t.center());
        wait(cx, 800);
        assert!(shown(cx), "reopens");
        mv(cx, point(px(5.), px(5.)));
        wait(cx, 400);
        assert!(!shown(cx), "closes again");
    }

    #[gpui::test]
    fn hover_card_quick_pass_does_not_open(cx: &mut TestAppContext) {
        let cx = setup(cx, || HoverCard::new("hc").trigger(trigger()).content(|_, _| marker()).into_any_element());
        let t = cx.debug_bounds("trigger").unwrap();
        mv(cx, t.center());
        wait(cx, 200);
        mv(cx, point(px(5.), px(5.)));
        wait(cx, 1000);
        assert!(!shown(cx));
    }

    #[gpui::test]
    fn tooltip_opens_and_closes(cx: &mut TestAppContext) {
        let cx = setup(cx, || {
            Popover::new("tt").unstyled().mode(PopoverTrigger::Hover { open_delay_ms: 300, close_delay_ms: 0 }).side(Side::Top).trigger(trigger()).content(|_, _| marker()).into_any_element()
        });
        let t = cx.debug_bounds("trigger").unwrap();
        mv(cx, t.center());
        wait(cx, 400);
        assert!(shown(cx), "tooltip open");
        mv(cx, point(px(5.), px(5.)));
        wait(cx, 50);
        assert!(!shown(cx), "tooltip closed");
    }

    #[gpui::test]
    fn tooltip_closes_when_leaving_window(cx: &mut TestAppContext) {
        let cx = setup(cx, || {
            Popover::new("tt").unstyled().mode(PopoverTrigger::Hover { open_delay_ms: 300, close_delay_ms: 0 }).side(Side::Top).trigger(trigger()).content(|_, _| marker()).into_any_element()
        });
        let t = cx.debug_bounds("trigger").unwrap();
        mv(cx, t.center());
        wait(cx, 400);
        assert!(shown(cx));
        cx.simulate_event(gpui::MouseExitEvent { position: point(px(-10.), px(-10.)), pressed_button: None, modifiers: Modifiers::none() });
        settle(cx);
        wait(cx, 50);
        assert!(!shown(cx), "closed after the mouse left the window");
    }

    #[gpui::test]
    fn hover_card_content_hover_survives_reopen(cx: &mut TestAppContext) {
        let cx = setup(cx, || HoverCard::new("hc").trigger(trigger()).content(|_, _| marker()).into_any_element());
        let t = cx.debug_bounds("trigger").unwrap();
        mv(cx, t.center());
        wait(cx, 800);
        let m = cx.debug_bounds("marker").unwrap();
        mv(cx, m.center());
        // Leave the window while over the content: closes.
        cx.simulate_event(gpui::MouseExitEvent { position: m.center(), pressed_button: None, modifiers: Modifiers::none() });
        wait(cx, 50);
        assert!(!shown(cx), "closed after leaving the window");
        // Come back, open again, rest on the content: must stay open.
        mv(cx, t.center());
        wait(cx, 800);
        assert!(shown(cx), "reopened");
        mv(cx, m.center());
        wait(cx, 600);
        assert!(shown(cx), "still open while hovering the content");
    }

    #[gpui::test]
    fn clicking_dismisses_until_pointer_leaves(cx: &mut TestAppContext) {
        let cx = setup(cx, || {
            Popover::new("tt").unstyled().mode(PopoverTrigger::Hover { open_delay_ms: 300, close_delay_ms: 0 }).side(Side::Top).trigger(trigger()).content(|_, _| marker()).into_any_element()
        });
        let t = cx.debug_bounds("trigger").unwrap();
        mv(cx, t.center());
        wait(cx, 400);
        assert!(shown(cx));
        cx.simulate_click(t.center(), Modifiers::none());
        settle(cx);
        mv(cx, t.center() + point(px(2.), px(0.)));
        wait(cx, 400);
        assert!(!shown(cx), "stays dismissed while still on the trigger");
        mv(cx, point(px(5.), px(5.)));
        mv(cx, t.center());
        wait(cx, 400);
        assert!(shown(cx), "opens again after leaving and coming back");
    }

    #[gpui::test]
    fn menubar_switches_menus_on_hover(cx: &mut TestAppContext) {
        use ui::menubar::{Menubar, MenubarMenu};
        thread_local! {
            static OPEN: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
        }
        let cx = setup(cx, || {
            Menubar::new("menubar")
                .menu(MenubarMenu::new("File").item(MenuItem::new("New")))
                .menu(MenubarMenu::new("Edit").item(MenuItem::new("Undo")))
                .on_value_change(|open, _, _| OPEN.with(|o| o.set(*open)))
                .into_any_element()
        });
        // The bar starts at (80, 80); "File" is its first trigger.
        let file = point(px(100.), px(98.));
        mv(cx, file);
        cx.simulate_click(file, Modifiers::none());
        settle(cx);
        assert_eq!(OPEN.with(|o| o.get()), Some(0), "File opened by click");
        // Slide right along the bar onto "Edit".
        for x in (100..=150).step_by(10) {
            mv(cx, point(px(x as f32), px(98.)));
        }
        assert_eq!(OPEN.with(|o| o.get()), Some(1), "hovering Edit switched menus");
        // Clicking outside closes.
        cx.simulate_click(point(px(600.), px(500.)), Modifiers::none());
        settle(cx);
        assert_eq!(OPEN.with(|o| o.get()), None, "outside click closed it");
    }

    #[gpui::test]
    fn navigation_menu_switches_instantly(cx: &mut TestAppContext) {
        use ui::navigation_menu::{NavigationMenu, NavigationMenuItem};
        thread_local! {
            static SHOWN: std::cell::RefCell<Vec<usize>> = const { std::cell::RefCell::new(Vec::new()) };
        }
        fn panel(ix: usize) -> impl IntoElement {
            div().w(px(200.)).h(px(100.)).child(gpui::canvas(|_, _, _| {}, move |_, _, _, _| SHOWN.with(|s| s.borrow_mut().push(ix))).size_full())
        }
        fn frame(cx: &mut VisualTestContext) -> Vec<usize> {
            SHOWN.with(|s| s.borrow_mut().clear());
            settle(cx);
            SHOWN.with(|s| s.borrow().clone())
        }
        let cx = setup(cx, || {
            NavigationMenu::new("nav")
                .item(NavigationMenuItem::new("One").content(|_, _| panel(0)))
                .item(NavigationMenuItem::new("Two").content(|_, _| panel(1)))
                .into_any_element()
        });
        let one = point(px(100.), px(98.));
        mv(cx, one);
        assert!(frame(cx).is_empty(), "waits for the open delay");
        wait(cx, 200);
        assert_eq!(frame(cx), vec![0]);
        // Slide to the second trigger: switches with no delay and no overlap.
        for x in (100..=200).step_by(10) {
            mv(cx, point(px(x as f32), px(98.)));
        }
        assert_eq!(frame(cx), vec![1]);
        // Leave: closes after the close delay.
        mv(cx, point(px(5.), px(5.)));
        wait(cx, 300);
        assert!(frame(cx).is_empty());
    }
}

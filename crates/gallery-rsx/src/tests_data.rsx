#![cfg(test)]
//! GPUI tests for the data batch: calendar, date picker, chart, sidebar.

use gpui::{Modifiers, TestAppContext, VisualTestContext};
use rsx::prelude::*;

use crate::tests::{center, mount, settle};
use crate::ui::calendar::{Calendar, CalendarMode, CalendarSelection, Date};

/// Where day `day` of `month` is in a calendar laid out at `cal` (one month shown).
///
/// The grid: `p-3`, a 32px header, `gap-4`, the 12.8px weekday row, then
/// 32px rows each with `mt-2`.
fn day_center(cal: rsx::Bounds, month: Date, day: u32) -> gpui::Point<gpui::Pixels> {
    let ix = month.first_of_month().weekday() + day - 1;
    let (row, col) = (ix / 7, ix % 7);
    let x = cal.origin.x + 12.0 + col as f32 * 32.0 + 16.0;
    let y = cal.origin.y + 12.0 + 32.0 + 16.0 + 12.8 + 8.0 + row as f32 * 40.0 + 16.0;
    gpui::point(gpui::px(x), gpui::px(y))
}

fn click(cx: &mut VisualTestContext, at: gpui::Point<gpui::Pixels>) {
    cx.simulate_click(at, Modifiers::default());
    settle(cx);
}

/// February 2026 starts on a Sunday and has exactly four weeks.
const FEB: Date = Date { year: 2026, month: 2, day: 1 };

#[gpui::test]
fn calendar_selects_a_day_and_navigates(cx: &mut TestAppContext) {
    let cal = cx.update(|_| NodeRef::new());
    let picked = cx.update(|_| signal(None::<CalendarSelection>));
    let cx = mount(cx, move || {
        <div class="flex items-start p-8">
            <div ref={cal} class="flex">
                <Calendar default_month={FEB} onChange={move |s: &CalendarSelection| picked.set(Some(s.clone()))} />
            </div>
        </div>
    });
    let bounds = cal.bounds().unwrap();
    click(cx, day_center(bounds, FEB, 10));
    assert_eq!(picked.get_untracked(), Some(CalendarSelection::Single(Some(Date::new(2026, 2, 10)))));
    // Clicking it again clears it.
    click(cx, day_center(bounds, FEB, 10));
    assert_eq!(picked.get_untracked(), Some(CalendarSelection::Single(None)));

    // Next month: March 2026 also starts on a Sunday.
    let next = gpui::point(gpui::px(bounds.origin.x + bounds.size.width - 12.0 - 16.0), gpui::px(bounds.origin.y + 12.0 + 16.0));
    click(cx, next);
    let march = Date::new(2026, 3, 1);
    click(cx, day_center(cal.bounds().unwrap(), march, 3));
    assert_eq!(picked.get_untracked(), Some(CalendarSelection::Single(Some(Date::new(2026, 3, 3)))));
}

#[gpui::test]
fn calendar_range_and_disabled_dates(cx: &mut TestAppContext) {
    let cal = cx.update(|_| NodeRef::new());
    let picked = cx.update(|_| signal(None::<CalendarSelection>));
    let cx = mount(cx, move || {
        <div class="flex items-start p-8">
            <div ref={cal} class="flex">
                <Calendar
                    mode={CalendarMode::Range}
                    default_month={FEB}
                    disabled={|d: Date| d.weekday() == 6}
                    onChange={move |s: &CalendarSelection| picked.set(Some(s.clone()))}
                />
            </div>
        </div>
    });
    let bounds = cal.bounds().unwrap();
    click(cx, day_center(bounds, FEB, 12));
    assert_eq!(picked.get_untracked(), Some(CalendarSelection::Range(Some(Date::new(2026, 2, 12)), None)));
    // An earlier end swaps the ends.
    click(cx, day_center(bounds, FEB, 3));
    assert_eq!(
        picked.get_untracked(),
        Some(CalendarSelection::Range(Some(Date::new(2026, 2, 3)), Some(Date::new(2026, 2, 12))))
    );
    // Saturdays are disabled: nothing happens.
    picked.set(None);
    click(cx, day_center(bounds, FEB, 7));
    assert_eq!(picked.get_untracked(), None);
}

#[gpui::test]
fn date_picker_opens_picks_and_closes(cx: &mut TestAppContext) {
    use crate::ui::date_picker::DatePicker;
    let trigger = cx.update(|_| NodeRef::new());
    let picked = cx.update(|_| signal(None::<CalendarSelection>));
    let cx = mount(cx, move || {
        <div class="flex items-start p-8">
            <div ref={trigger} class="flex">
                <DatePicker onChange={move |s: &CalendarSelection| picked.set(Some(s.clone()))} />
            </div>
        </div>
    });
    let today = Date::today();
    // The calendar opens below the trigger, start-aligned, 4px off, inside a 1px border.
    let calendar_at = |t: rsx::Bounds| rsx::Bounds {
        origin: rsx::Point { x: t.origin.x + 1.0, y: t.origin.y + t.size.height + 4.0 + 1.0 },
        size: t.size,
    };
    let day = day_center(calendar_at(trigger.bounds().unwrap()), today, 15);

    // Closed: clicking where the calendar would be does nothing.
    click(cx, day);
    assert_eq!(picked.get_untracked(), None);

    // Open, then Escape closes it.
    click(cx, center(trigger));
    cx.simulate_keystrokes("escape");
    settle(cx);
    click(cx, day);
    assert_eq!(picked.get_untracked(), None, "Escape closed the popover");

    // Open and pick: the value changes and the popover closes.
    click(cx, center(trigger));
    click(cx, day);
    let expected = Date::new(today.year, today.month, 15);
    assert_eq!(picked.get_untracked(), Some(CalendarSelection::Single(Some(expected))));
    picked.set(None);
    click(cx, day);
    assert_eq!(picked.get_untracked(), None, "picking a date closed the popover");
}

#[gpui::test]
fn sidebar_collapses_and_opens_sub_items(cx: &mut TestAppContext) {
    use crate::ui::icon::IconName;
    use crate::ui::sidebar::{is_sidebar_collapsed, Sidebar, SidebarGroup, SidebarInset, SidebarMenuItem, SidebarTrigger};
    let panel = cx.update(|_| NodeRef::new());
    let trigger = cx.update(|_| NodeRef::new());
    let settings = cx.update(|_| NodeRef::new());
    let general = cx.update(|_| NodeRef::new());
    let billing = cx.update(|_| NodeRef::new());
    let clicked = cx.update(|_| signal(0));
    let cx = mount(cx, move || {
        <div class="flex w-[800px] h-[400px]">
            <div ref={panel} class="flex h-full">
                <Sidebar id="test-sidebar">
                    <SidebarGroup label="Group">
                        <div ref={settings}>
                            <SidebarMenuItem label="Settings" icon={IconName::Settings} onClick={move |_| clicked.update(|n| *n += 1)}>
                                <div ref={general}><SidebarMenuItem label="General" /></div>
                            </SidebarMenuItem>
                        </div>
                        <SidebarMenuItem label="Account" icon={IconName::User}>
                            <div ref={billing}><SidebarMenuItem label="Billing" active /></div>
                        </SidebarMenuItem>
                    </SidebarGroup>
                </Sidebar>
            </div>
            <SidebarInset>
                <div class="flex"><div ref={trigger} class="flex"><SidebarTrigger sidebar="test-sidebar" /></div></div>
            </SidebarInset>
        </div>
    });
    assert_eq!(panel.bounds().unwrap().size.width, 256.0);
    assert!(general.bounds().is_none(), "sub items start closed");
    assert!(billing.bounds().is_some(), "an active sub item opens its parent");

    // Clicking the item runs its handler and opens its sub items.
    let s = settings.bounds().unwrap();
    click(cx, gpui::point(gpui::px(s.origin.x + s.size.width / 2.0), gpui::px(s.origin.y + 16.0)));
    assert_eq!(clicked.get_untracked(), 1);
    assert!(general.bounds().is_some(), "sub items opened");

    // The trigger collapses it to the icon strip, and back.
    click(cx, center(trigger));
    assert!(cx.update(|_, _| is_sidebar_collapsed("test-sidebar")));
    assert_eq!(panel.bounds().unwrap().size.width, 48.0);
    click(cx, center(trigger));
    assert_eq!(panel.bounds().unwrap().size.width, 256.0);
}

#[gpui::test]
fn chart_hover_shows_the_point(cx: &mut TestAppContext) {
    use crate::ui::chart::{BarChart, ChartSeries, LineChart};
    let bar = cx.update(|_| NodeRef::new());
    let line = cx.update(|_| NodeRef::new());
    let cx = mount(cx, move || {
        <div class="flex flex-col gap-4 w-[400px] p-8">
            <div ref={bar} class="flex">
                <BarChart labels={["A", "B", "C"]} series={vec![ChartSeries::new("X").data([1., 2., 3.]), ChartSeries::new("Y").data([3., 2., 1.])]} />
            </div>
            <div ref={line} class="flex">
                <LineChart labels={["A", "B", "C"]} series={vec![ChartSeries::new("X").data([1., 2., 3.])]} />
            </div>
        </div>
    });
    // Hovering the plots (the top 200px) highlights a point and shows the
    // tooltip; leaving clears it. Here we check it renders without panicking
    // and the plot keeps its size.
    for r in [bar, line] {
        let b = r.bounds().unwrap();
        assert_eq!(b.size.width, 336.0);
        cx.simulate_mouse_move(gpui::point(gpui::px(b.origin.x + 200.0), gpui::px(b.origin.y + 100.0)), None, Modifiers::default());
        settle(cx);
        assert_eq!(r.bounds().unwrap().size, b.size);
    }
    cx.simulate_mouse_move(gpui::point(gpui::px(1.0), gpui::px(1.0)), None, Modifiers::default());
    settle(cx);
}


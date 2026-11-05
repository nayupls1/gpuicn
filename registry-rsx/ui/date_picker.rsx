//! A date picker component with range and presets — a Popover holding a Calendar.
//!
//! ```ignore
//! <DatePicker placeholder="Pick a date" onChange={move |s: &CalendarSelection| println!("{:?}", s.single())} />
//! <DatePicker mode={CalendarMode::Range} months={2} class="w-full" />
//! ```

use rsx::prelude::*;
use rsx::Div;

use super::button::Button;
use super::calendar::{selection_label, Calendar, CalendarMode, CalendarSelection};
use super::icon::IconName;
use super::popover::{outer_layout_class, Popover, PopoverHandle};
use super::theme::theme;
use super::utils::Align;

#[component]
pub fn DatePicker(
    #[prop(optional)] mode: CalendarMode,
    #[prop(default = 1)] months: usize,
    /// Shown while nothing is picked (default "Pick a date").
    placeholder: Option<Accessor<String>>,
    /// Controlled selection.
    selection: Option<Accessor<CalendarSelection>>,
    on_change: Option<Callback<CalendarSelection>>,
    /// Styles for the trigger button; layout (width, flex) also sizes the popover's wrapper.
    attrs: Attributes,
) -> impl IntoElement {
    let empty = match mode {
        CalendarMode::Single => CalendarSelection::Single(None),
        CalendarMode::Range => CalendarSelection::Range(None, None),
        CalendarMode::Multiple => CalendarSelection::Multiple(Vec::new()),
    };
    let inner = signal(empty);
    let current = Accessor::derive(move || selection.map(|s| s.get()).unwrap_or_else(|| inner.get()));
    let placeholder = placeholder.unwrap_or_else(|| Accessor::value("Pick a date".to_string()));
    let wrapper = outer_layout_class(&attrs);
    let attrs = StoredValue::new(attrs);
    let popover = PopoverHandle::new();
    let t = theme();
    let width = if mode == CalendarMode::Range { 300.0 } else { 240.0 };

    let trigger = move || {
        let theme = t.get();
        let muted = theme.muted_foreground;
        let label = selection_label(&current.get());
        let empty = label.is_none();
        <Button
            outline
            icon={IconName::Calendar}
            class={format!("justify-start font-normal w-[{width}px]")}
            // Empty: the whole trigger (icon too) is `text-muted-foreground`.
            bind={move |el: Div| if empty { el.text_color(muted) } else { el }}
            {...attrs.with_value(|a| a.clone())}
        >
            {match label {
                Some(label) => <div>{label}</div>,
                None => <div text_color={muted}>{placeholder.get()}</div>,
            }}
        </Button>
    };

    <Popover
        handle={popover}
        wrapper_class={wrapper}
        align={Align::Start}
        class="w-auto p-0"
        trigger={move || trigger.into_element()}
        content={move |_| <div class="flex">
            <Calendar
                bg={t.get().popover}
                mode={mode}
                months={months}
                selection={current}
                onChange={move |next: &CalendarSelection| {
                    inner.set(next.clone());
                    let complete = matches!(next, CalendarSelection::Single(Some(_)) | CalendarSelection::Range(Some(_), Some(_)));
                    if complete {
                        popover.close();
                    }
                    if let Some(f) = on_change {
                        f.call(next);
                    }
                }}
            />
        </div>}
    />
}

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
use super::popover::{Popover, PopoverHandle};
use super::theme::theme;
use super::utils::Align;

/// The caller's layout classes that belong on the popover's wrapper rather
/// than on the trigger button (gpuicn's `popover::outer_layout`): width,
/// flex grow and self-alignment, so `class="w-full"` stretches the picker.
fn outer_layout_class(attrs: &Attributes) -> String {
    let style = rsx::div().spread(attrs).computed_style();
    let len = |prefix: &str, l: Length| match l {
        Length::Px(v) => format!("{prefix}-[{v}px]"),
        Length::Rems(v) => format!("{prefix}-[{v}rem]"),
        Length::Fraction(f) if f == 1.0 => format!("{prefix}-full"),
        Length::Fraction(f) => format!("{prefix}-[{}%]", f * 100.0),
        Length::Auto => format!("{prefix}-auto"),
    };
    let mut classes = Vec::new();
    if let Some(w) = style.size.width {
        classes.push(len("w", w));
    }
    if let Some(w) = style.min_size.width {
        classes.push(len("min-w", w));
    }
    if let Some(w) = style.max_size.width {
        classes.push(len("max-w", w));
    }
    if style.flex_grow.is_some_and(|g| g > 0.0) {
        classes.push("flex-1".into());
    }
    classes.join(" ")
}

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
        // Empty: the whole trigger (icon too) is `text-muted-foreground`.
        let muted_text = Attributes::new().with(move |el: Div| if empty { el.text_color(muted) } else { el });
        <Button
            outline
            icon={IconName::Calendar}
            class={format!("justify-start font-normal w-[{width}px]")}
            {...muted_text}
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
        // rsx#?: a mouse-down in the content also reaches the popover's
        // click-away backdrop underneath (sibling hitboxes aren't occluded),
        // which closes it before the click lands; stop it here.
        content={move |_| <div class="flex" onMouseDown={move |e: &MouseEvent| e.stop_propagation()}>
            <Calendar
                {...Attributes::new().with(move |el: Div| el.bg(t.get().popover))}
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

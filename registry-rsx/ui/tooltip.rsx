//! A popup that displays information related to an element when the element
//! receives hover.
//!
//! ```ignore
//! <Tooltip content="Add to library">
//!     <Button outline label="Hover" />
//! </Tooltip>
//! ```

use rsx::prelude::*;

use super::popover::{Popover, PopoverTrigger};
use super::theme::theme;
use super::utils::{Align, Side};

/// The `TooltipArrow`: a small triangle pointing from the bubble at the trigger.
fn arrow(side: Side, color: Color) -> Element {
    let (w, h, points) = match side {
        // Bubble above the trigger: arrow points down.
        Side::Top => (10, 5, "0,0 10,0 5,5"),
        Side::Bottom => (10, 5, "0,5 10,5 5,0"),
        Side::Left => (5, 10, "0,0 0,10 5,5"),
        Side::Right => (5, 10, "5,0 5,10 0,5"),
    };
    let (r, g, b, _) = color.to_rgba8();
    <svg width={w} height={h} viewBox={format!("0 0 {w} {h}")} class="flex-none">
        <polygon points={points} fill={format!("#{r:02x}{g:02x}{b:02x}")} />
    </svg>
}

#[component]
pub fn Tooltip(
    /// The tooltip text.
    content: Accessor<String>,
    #[prop(default = Side::Top)] side: Side,
    #[prop(optional)] align: Align,
    /// Hover time before showing, in milliseconds (default 300).
    #[prop(default = 300)] delay: u64,
    /// The element that shows the tooltip when hovered.
    children: Children,
    /// Styles for the bubble.
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let attrs = StoredValue::new(attrs);
    <Popover
        unstyled
        mode={PopoverTrigger::Hover { open_delay_ms: delay, close_delay_ms: 0 }}
        side={side}
        align={align}
        side_offset={4}
        trigger={children}
        content={move |_| {
            let bubble = move || {
                let theme = t.get();
                <div
                    class="px-3 py-1.5 text-xs leading-[1rem]"
                    corner_radius={theme.radius_md()}
                    bg={theme.primary}
                    text_color={theme.primary_foreground}
                    {...attrs.with_value(|a| a.clone())}
                >
                    {content}
                </div>
            };
            let arrow = move || arrow(side, t.get().primary);
            match side {
                Side::Top => <div class="flex flex-col items-center">{bubble}{arrow}</div>,
                Side::Bottom => <div class="flex flex-col items-center">{arrow}{bubble}</div>,
                Side::Left => <div class="flex items-center">{bubble}{arrow}</div>,
                Side::Right => <div class="flex items-center">{arrow}{bubble}</div>,
            }
        }}
    />
}

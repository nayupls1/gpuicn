//! Display additional information or actions next to an input or textarea.
//!
//! ```ignore
//! <InputGroup start={InputGroupAddon::icon(IconName::Search)} end={InputGroupAddon::text("12 results")}>
//!     <Input placeholder="Search..." />
//! </InputGroup>
//!
//! <InputGroup
//!     start={InputGroupAddon::text("https://")}
//!     end={InputGroupAddon::button(<Button ghost size={ButtonSize::IconSm} icon={IconName::Copy} />)}
//! >
//!     <Input placeholder="example.com" />
//! </InputGroup>
//! ```
//!
//! The `Input` inside drops its own border and ring (gpuicn's `.unstyled()`)
//! and the group draws them around everything.

use rsx::prelude::*;
use rsx::Div;

use super::icon::{Icon, IconName};
use super::input::InputGroupContext;
use super::theme::theme;
use super::utils::{control_surface, focus_ring, invalid_ring, to_px};

/// Content placed beside the input (`InputGroupAddon`).
#[derive(Clone)]
pub struct InputGroupAddon(Element);

impl InputGroupAddon {
    pub fn icon(icon: IconName) -> Self {
        Self(<Icon name={icon} />)
    }

    /// `InputGroupText`
    pub fn text(text: impl Into<SharedString>) -> Self {
        let text: SharedString = text.into();
        // rsx#9: no `whitespace-nowrap`; `flex-none` keeps it on one line.
        Self(<div class="flex-none">{text}</div>)
    }

    /// `InputGroupButton`
    pub fn button(button: impl IntoElement) -> Self {
        Self(button.into_element())
    }

    pub fn element(element: impl IntoElement) -> Self {
        Self(element.into_element())
    }
}

impl IntoElement for InputGroupAddon {
    fn into_element(self) -> Element {
        self.0
    }
}

/// So a prop takes one addon or a `vec![..]` of them.
impl From<InputGroupAddon> for Vec<InputGroupAddon> {
    fn from(addon: InputGroupAddon) -> Self {
        vec![addon]
    }
}

#[component]
pub fn InputGroup(
    /// `align="inline-start"`
    #[prop(optional)]
    start: Vec<InputGroupAddon>,
    /// `align="inline-end"`
    #[prop(optional)]
    end: Vec<InputGroupAddon>,
    /// `align="block-start"`: a row above the input (for textareas).
    #[prop(optional)]
    block_start: Vec<InputGroupAddon>,
    /// `align="block-end"`: a row below the input.
    #[prop(optional)]
    block_end: Vec<InputGroupAddon>,
    invalid: Option<Accessor<bool>>,
    /// The `Input` (rendered inside the group so it finds it).
    children: ChildrenFn,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let focused = signal(false);
    let invalid = memo(move || invalid.map(|i| i.get()).unwrap_or(false));
    let addon_row = move |items: Vec<InputGroupAddon>, class: &'static str| {
        <div class="flex items-center gap-2 px-3 text-sm leading-[1.25rem]" text_color={t.get().muted_foreground} {...Attributes::new().class(class)}>
            {items}
        </div>
    };
    let surface = move |el: Div| {
        let theme = t.get();
        let border = if invalid.get() {
            theme.destructive
        } else if focused.get() {
            theme.ring
        } else {
            theme.input
        };
        control_surface(&theme, el.corner_radius(theme.radius_md()).border_color(border))
    };
    let (has_start, has_end) = (!start.is_empty(), !end.is_empty());

    <div class="relative flex flex-col w-full min-w-0 border" bind={surface} {...&attrs}>
        {(!block_start.is_empty()).then(|| addon_row(block_start, "pt-3"))}
        <div class="flex items-center min-h-9">
            {has_start.then(|| addon_row(start, "pr-0"))}
            <div class="flex-1 min-w-0 px-3 py-1">
                <Provider value={InputGroupContext { focused }}>{children}</Provider>
            </div>
            {has_end.then(|| addon_row(end, "pl-0 pr-2"))}
        </div>
        {(!block_end.is_empty()).then(|| addon_row(block_end, "pb-3"))}
        {move || {
            let theme = t.get();
            let radius = to_px(theme.radius_md());
            (focused.get() || invalid.get()).then(|| if invalid.get() { invalid_ring(&theme, radius) } else { focus_ring(&theme, radius) })
        }}
    </div>
}

//! Extends the Dialog component to display content that complements the main
//! content of the screen, sliding in from an edge.
//!
//! ```ignore
//! <Sheet
//!     side={Side::Right}
//!     trigger={<Button outline label="Open" />}
//!     title="Edit profile"
//!     description="Make changes to your profile here."
//!     content={|_| form()}
//!     footer={|sheet: DialogHandle| <Button label="Save changes" onClick={move |_| sheet.close()} />}
//! />
//! ```

use rsx::prelude::*;

use super::dialog::{close_button, header, modal_trigger, use_modal, DialogHandle, Modal, ModalPlacement};
use super::theme::theme;
use super::utils::Side;

#[component]
pub fn Sheet(
    /// The edge the sheet slides in from (default `Right`).
    #[prop(default = Side::Right)]
    side: Side,
    /// The element that opens the sheet.
    trigger: Option<Children>,
    title: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// The body, under the header.
    content: Option<Render<DialogHandle>>,
    /// Stacked at the bottom of the sheet.
    footer: Option<Render<DialogHandle>>,
    /// Control the open state from outside.
    open: Option<Accessor<bool>>,
    on_open_change: Option<Callback<bool>>,
    /// Share state with the caller (see `DialogHandle`).
    handle: Option<DialogHandle>,
    /// Styles for the panel.
    attrs: Attributes,
) -> impl IntoElement {
    let handle = use_modal(open, on_open_change, handle);
    let t = theme();
    let attrs = StoredValue::new(attrs);
    let placement = match side {
        Side::Top => ModalPlacement::Top,
        Side::Right => ModalPlacement::Right,
        Side::Bottom => ModalPlacement::Bottom,
        Side::Left => ModalPlacement::Left,
    };
    let panel = move |handle: DialogHandle| {
        // `w-3/4 sm:max-w-sm`: 384px, at most 3/4 of the window (the wrapper caps it).
        let (size, edge) = match side {
            Side::Right => ("h-full w-[384px] max-w-full", "border-l"),
            Side::Left => ("h-full w-[384px] max-w-full", "border-r"),
            Side::Top => ("w-full", "border-b"),
            Side::Bottom => ("w-full", "border-t"),
        };
        // rsx#7: no `shadow-lg`.
        <div
            class={format!("relative flex flex-col gap-4 {size} {edge}")}
            bg={t.get().background}
            text_color={t.get().foreground}
            border_color={t.get().border}
            {...attrs.with_value(|a| a.clone())}
        >
            {header(title, description, "gap-1.5 p-4")}
            {content.map(|c| <div class="flex-1 min-w-0 px-4">{c.render(handle)}</div>)}
            {footer.map(|f| <div class="mt-auto flex flex-col gap-2 p-4">{f.render(handle)}</div>)}
            {close_button(handle)}
        </div>
    };
    let wrapper = match side {
        Side::Left | Side::Right => "max-w-[75%]",
        Side::Top | Side::Bottom => "",
    };
    <div class="flex">
        {modal_trigger(handle, trigger)}
        <Modal handle={handle} placement={placement} wrapper_class={wrapper.to_string()} panel={panel} />
    </div>
}

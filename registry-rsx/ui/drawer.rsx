//! A drawer component that slides up from the bottom of the screen
//! (the `vaul` drawer in shadcn).
//!
//! ```ignore
//! <Drawer
//!     trigger={<Button outline label="Open Drawer" />}
//!     title="Move Goal"
//!     description="Set your daily activity goal."
//!     content={|_| goal_picker()}
//!     footer={|drawer: DialogHandle| <Button label="Submit" onClick={move |_| drawer.close()} />}
//! />
//! ```

use rsx::prelude::*;

use super::dialog::{header, modal_trigger, use_modal, DialogHandle, Modal, ModalPlacement};
use super::theme::theme;

#[component]
pub fn Drawer(
    /// The element that opens the drawer.
    trigger: Option<Children>,
    title: Option<Accessor<String>>,
    description: Option<Accessor<String>>,
    /// The body, under the header.
    content: Option<Render<DialogHandle>>,
    /// Stacked under the content.
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
    let panel = move |handle: DialogHandle| {
        <div
            class="flex flex-col w-full max-h-full border-t"
            border_color={t.get().border}
            bg={t.get().background}
            text_color={t.get().foreground}
            bind={move |el: rsx::Div| el.rounded_t_radius(t.get().radius_lg())}
            {...attrs.with_value(|a| a.clone())}
        >
            {/* The drag handle. */}
            <div class="mx-auto mt-4 h-2 w-[100px] flex-none rounded-full" bg={t.get().muted} />
            <div class="flex flex-col w-full max-w-[384px] mx-auto">
                {header(title, description, "gap-1.5 p-4 items-center text-center")}
                {content.map(|c| <div class="px-4">{c.render(handle)}</div>)}
                {footer.map(|f| <div class="flex flex-col gap-2 p-4">{f.render(handle)}</div>)}
            </div>
        </div>
    };
    <div class="flex">
        {modal_trigger(handle, trigger)}
        {/* `max-h-[80vh]` */}
        <Modal handle={handle} placement={ModalPlacement::Bottom} wrapper_class={"max-h-[80%]".to_string()} panel={panel} />
    </div>
}

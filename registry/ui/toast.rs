//! A succinct message that is displayed temporarily.
//!
//! shadcn/ui deprecated its `toast` component in favor of `sonner`; this
//! module keeps the old names working on top of [`super::sonner`].
//!
//! ```ignore
//! .child(Toaster::new())
//! show(Toast::new("Scheduled: Catch up").description("Friday, February 10, 2023 at 5:57 PM"), cx);
//! ```

pub use super::sonner::{Toast, ToastVariant, Toaster, ToasterPosition, dismiss, toast as show, update};

/// `toast({ variant: "destructive" })`
pub fn destructive(title: impl Into<gpui::SharedString>) -> Toast {
    Toast::new(title).error()
}

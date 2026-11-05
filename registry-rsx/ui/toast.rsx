//! A succinct message that is displayed temporarily.
//!
//! shadcn/ui deprecated its `toast` component in favor of `sonner`; this
//! module keeps the old names working on top of [`super::sonner`].
//!
//! ```ignore
//! <Toaster />
//! show(Toast::new("Scheduled: Catch up").description("Friday, February 10, 2023 at 5:57 PM"));
//! ```

pub use super::sonner::{dismiss, toast as show, update, Toast, ToastVariant, Toaster, ToasterPosition};

/// `toast({ variant: "destructive" })`
pub fn destructive(title: impl Into<rsx::SharedString>) -> Toast {
    Toast::new(title).error()
}

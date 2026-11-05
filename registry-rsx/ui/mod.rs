//! UI components installed by `gpuicn-rsx`.
//!
//! These files are yours: edit them freely. Restyle everything from `theme.rsx`.
#![allow(dead_code, unused_imports)]

// gpuicn:begin — managed by `gpuicn-rsx add`; edits inside this block are overwritten.
rsx::module!(pub alert_dialog = "components/ui/alert_dialog");
rsx::module!(pub button = "components/ui/button");
rsx::module!(pub carousel = "components/ui/carousel");
rsx::module!(pub dialog = "components/ui/dialog");
rsx::module!(pub drawer = "components/ui/drawer");
rsx::module!(pub hover_card = "components/ui/hover_card");
rsx::module!(pub icon = "components/ui/icon");
rsx::module!(pub label = "components/ui/label");
rsx::module!(pub popover = "components/ui/popover");
rsx::module!(pub portal = "components/ui/portal");
rsx::module!(pub separator = "components/ui/separator");
rsx::module!(pub sheet = "components/ui/sheet");
rsx::module!(pub sonner = "components/ui/sonner");
rsx::module!(pub spinner = "components/ui/spinner");
rsx::module!(pub theme = "components/ui/theme");
rsx::module!(pub toast = "components/ui/toast");
rsx::module!(pub tooltip = "components/ui/tooltip");
rsx::module!(pub utils = "components/ui/utils");

/// Install the theme. Call once at startup, inside the root component.
pub fn init() {
    theme::init();
    utils::init();
}
// gpuicn:end

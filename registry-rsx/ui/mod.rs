//! UI components installed by `gpuicn-rsx`.
//!
//! These files are yours: edit them freely. Restyle everything from `theme.rsx`.
#![allow(dead_code, unused_imports)]

// gpuicn:begin — managed by `gpuicn-rsx add`; edits inside this block are overwritten.
rsx::module!(pub button = "components/ui/button");
rsx::module!(pub field = "components/ui/field");
rsx::module!(pub form = "components/ui/form");
rsx::module!(pub icon = "components/ui/icon");
rsx::module!(pub input = "components/ui/input");
rsx::module!(pub input_group = "components/ui/input_group");
rsx::module!(pub input_otp = "components/ui/input_otp");
rsx::module!(pub label = "components/ui/label");
rsx::module!(pub popover = "components/ui/popover");
rsx::module!(pub portal = "components/ui/portal");
rsx::module!(pub separator = "components/ui/separator");
rsx::module!(pub spinner = "components/ui/spinner");
rsx::module!(pub textarea = "components/ui/textarea");
rsx::module!(pub theme = "components/ui/theme");
rsx::module!(pub tooltip = "components/ui/tooltip");
rsx::module!(pub utils = "components/ui/utils");

/// Install the theme. Call once at startup, inside the root component.
pub fn init() {
    theme::init();
    utils::init();
    input::init();
}
// gpuicn:end

//! UI components installed by `gpuicn-rsx`.
//!
//! These files are yours: edit them freely. Restyle everything from `theme.rsx`.
#![allow(dead_code, unused_imports)]

// gpuicn:begin — managed by `gpuicn-rsx add`; edits inside this block are overwritten.
rsx::module!(pub aspect_ratio = "components/ui/aspect_ratio");
rsx::module!(pub avatar = "components/ui/avatar");
rsx::module!(pub badge = "components/ui/badge");
rsx::module!(pub button = "components/ui/button");
rsx::module!(pub calendar = "components/ui/calendar");
rsx::module!(pub chart = "components/ui/chart");
rsx::module!(pub checkbox = "components/ui/checkbox");
rsx::module!(pub date_picker = "components/ui/date_picker");
rsx::module!(pub icon = "components/ui/icon");
rsx::module!(pub kbd = "components/ui/kbd");
rsx::module!(pub label = "components/ui/label");
rsx::module!(pub popover = "components/ui/popover");
rsx::module!(pub portal = "components/ui/portal");
rsx::module!(pub progress = "components/ui/progress");
rsx::module!(pub radio_group = "components/ui/radio_group");
rsx::module!(pub separator = "components/ui/separator");
rsx::module!(pub sidebar = "components/ui/sidebar");
rsx::module!(pub skeleton = "components/ui/skeleton");
rsx::module!(pub slider = "components/ui/slider");
rsx::module!(pub spinner = "components/ui/spinner");
rsx::module!(pub switch = "components/ui/switch");
rsx::module!(pub theme = "components/ui/theme");
rsx::module!(pub toggle = "components/ui/toggle");
rsx::module!(pub toggle_group = "components/ui/toggle_group");
rsx::module!(pub tooltip = "components/ui/tooltip");
rsx::module!(pub utils = "components/ui/utils");

/// Install the theme. Call once at startup, inside the root component.
pub fn init() {
    theme::init();
    utils::init();
}
// gpuicn:end

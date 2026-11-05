//! UI components installed by `gpuicn-rsx`.
//!
//! These files are yours: edit them freely. Restyle everything from `theme.rsx`.
#![allow(dead_code, unused_imports)]

// gpuicn:begin — managed by `gpuicn-rsx add`; edits inside this block are overwritten.
rsx::module!(pub accordion = "components/ui/accordion");
rsx::module!(pub alert = "components/ui/alert");
rsx::module!(pub breadcrumb = "components/ui/breadcrumb");
rsx::module!(pub button = "components/ui/button");
rsx::module!(pub button_group = "components/ui/button_group");
rsx::module!(pub card = "components/ui/card");
rsx::module!(pub collapsible = "components/ui/collapsible");
rsx::module!(pub empty = "components/ui/empty");
rsx::module!(pub icon = "components/ui/icon");
rsx::module!(pub item = "components/ui/item");
rsx::module!(pub label = "components/ui/label");
rsx::module!(pub pagination = "components/ui/pagination");
rsx::module!(pub popover = "components/ui/popover");
rsx::module!(pub portal = "components/ui/portal");
rsx::module!(pub resizable = "components/ui/resizable");
rsx::module!(pub scroll_area = "components/ui/scroll_area");
rsx::module!(pub separator = "components/ui/separator");
rsx::module!(pub spinner = "components/ui/spinner");
rsx::module!(pub table = "components/ui/table");
rsx::module!(pub tabs = "components/ui/tabs");
rsx::module!(pub theme = "components/ui/theme");
rsx::module!(pub tooltip = "components/ui/tooltip");
rsx::module!(pub typography = "components/ui/typography");
rsx::module!(pub utils = "components/ui/utils");

/// Install the theme. Call once at startup, inside the root component.
pub fn init() {
    theme::init();
    utils::init();
}
// gpuicn:end

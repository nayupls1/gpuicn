//! UI components installed by `gpuicn-rsx`.
//!
//! These files are yours: edit them freely. Restyle everything from `theme.rsx`.
#![allow(dead_code, unused_imports)]

// gpuicn:begin — managed by `gpuicn-rsx add`; edits inside this block are overwritten.
rsx::module!(pub accordion = "components/ui/accordion");
rsx::module!(pub alert = "components/ui/alert");
rsx::module!(pub alert_dialog = "components/ui/alert_dialog");
rsx::module!(pub aspect_ratio = "components/ui/aspect_ratio");
rsx::module!(pub avatar = "components/ui/avatar");
rsx::module!(pub badge = "components/ui/badge");
rsx::module!(pub breadcrumb = "components/ui/breadcrumb");
rsx::module!(pub button = "components/ui/button");
rsx::module!(pub button_group = "components/ui/button_group");
rsx::module!(pub calendar = "components/ui/calendar");
rsx::module!(pub card = "components/ui/card");
rsx::module!(pub carousel = "components/ui/carousel");
rsx::module!(pub chart = "components/ui/chart");
rsx::module!(pub checkbox = "components/ui/checkbox");
rsx::module!(pub collapsible = "components/ui/collapsible");
rsx::module!(pub context_menu = "components/ui/context_menu");
rsx::module!(pub date_picker = "components/ui/date_picker");
rsx::module!(pub dialog = "components/ui/dialog");
rsx::module!(pub drawer = "components/ui/drawer");
rsx::module!(pub dropdown_menu = "components/ui/dropdown_menu");
rsx::module!(pub empty = "components/ui/empty");
rsx::module!(pub field = "components/ui/field");
rsx::module!(pub form = "components/ui/form");
rsx::module!(pub hover_card = "components/ui/hover_card");
rsx::module!(pub icon = "components/ui/icon");
rsx::module!(pub input = "components/ui/input");
rsx::module!(pub input_group = "components/ui/input_group");
rsx::module!(pub input_otp = "components/ui/input_otp");
rsx::module!(pub item = "components/ui/item");
rsx::module!(pub kbd = "components/ui/kbd");
rsx::module!(pub label = "components/ui/label");
rsx::module!(pub menubar = "components/ui/menubar");
rsx::module!(pub native_select = "components/ui/native_select");
rsx::module!(pub navigation_menu = "components/ui/navigation_menu");
rsx::module!(pub pagination = "components/ui/pagination");
rsx::module!(pub popover = "components/ui/popover");
rsx::module!(pub portal = "components/ui/portal");
rsx::module!(pub progress = "components/ui/progress");
rsx::module!(pub radio_group = "components/ui/radio_group");
rsx::module!(pub resizable = "components/ui/resizable");
rsx::module!(pub scroll_area = "components/ui/scroll_area");
rsx::module!(pub select = "components/ui/select");
rsx::module!(pub separator = "components/ui/separator");
rsx::module!(pub sheet = "components/ui/sheet");
rsx::module!(pub sidebar = "components/ui/sidebar");
rsx::module!(pub skeleton = "components/ui/skeleton");
rsx::module!(pub slider = "components/ui/slider");
rsx::module!(pub sonner = "components/ui/sonner");
rsx::module!(pub spinner = "components/ui/spinner");
rsx::module!(pub switch = "components/ui/switch");
rsx::module!(pub table = "components/ui/table");
rsx::module!(pub tabs = "components/ui/tabs");
rsx::module!(pub textarea = "components/ui/textarea");
rsx::module!(pub theme = "components/ui/theme");
rsx::module!(pub toast = "components/ui/toast");
rsx::module!(pub toggle = "components/ui/toggle");
rsx::module!(pub toggle_group = "components/ui/toggle_group");
rsx::module!(pub tooltip = "components/ui/tooltip");
rsx::module!(pub typography = "components/ui/typography");
rsx::module!(pub utils = "components/ui/utils");

/// Install the theme. Call once at startup, inside the root component.
pub fn init() {
    theme::init();
    utils::init();
    input::init();
}
// gpuicn:end

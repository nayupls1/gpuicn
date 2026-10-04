//! UI components installed by `gpuicn`.
//!
//! These files are yours: edit them freely. Restyle everything from `theme.rs`.
#![allow(dead_code, unused_imports)]

// gpuicn:begin — managed by `gpuicn add`; edits inside this block are overwritten.
pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod chart;
pub mod checkbox;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod data_table;
pub mod date_picker;
pub mod dialog;
pub mod drawer;
pub mod dropdown_menu;
pub mod empty;
pub mod field;
pub mod form;
pub mod hover_card;
pub mod icon;
pub mod input;
pub mod input_group;
pub mod input_otp;
pub mod item;
pub mod kbd;
pub mod label;
pub mod menubar;
pub mod native_select;
pub mod navigation_menu;
pub mod pagination;
pub mod popover;
pub mod portal;
pub mod progress;
pub mod radio_group;
pub mod resizable;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod sonner;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod theme;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod tooltip;
pub mod typography;
pub mod utils;

/// Install the theme and key bindings. Call once at startup.
pub fn init(cx: &mut gpui::App) {
    theme::init(cx);
    input::init(cx);
}
// gpuicn:end

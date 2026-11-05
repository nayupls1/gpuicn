//! One demo per component, mirroring the examples on ui.shadcn.com (and
//! crates/gallery/src/demos.rs, page for page).
//!
//! Each page is `demos/<name>.rsx` with `pub fn demo() -> Element`.
#![allow(dead_code, unused_imports)]

rsx::module!(pub aspect_ratio = "demos/aspect_ratio");
rsx::module!(pub avatar = "demos/avatar");
rsx::module!(pub badge = "demos/badge");
rsx::module!(pub button = "demos/button");
rsx::module!(pub calendar = "demos/calendar");
rsx::module!(pub chart = "demos/chart");
rsx::module!(pub checkbox = "demos/checkbox");
rsx::module!(pub combobox = "demos/combobox");
rsx::module!(pub command = "demos/command");
rsx::module!(pub date_picker = "demos/date_picker");
rsx::module!(pub field = "demos/field");
rsx::module!(pub form = "demos/form");
rsx::module!(pub input = "demos/input");
rsx::module!(pub input_group = "demos/input_group");
rsx::module!(pub input_otp = "demos/input_otp");
rsx::module!(pub kbd = "demos/kbd");
rsx::module!(pub label = "demos/label");
rsx::module!(pub popover = "demos/popover");
rsx::module!(pub progress = "demos/progress");
rsx::module!(pub radio_group = "demos/radio_group");
rsx::module!(pub separator = "demos/separator");
rsx::module!(pub sidebar = "demos/sidebar");
rsx::module!(pub skeleton = "demos/skeleton");
rsx::module!(pub slider = "demos/slider");
rsx::module!(pub spinner = "demos/spinner");
rsx::module!(pub switch = "demos/switch");
rsx::module!(pub textarea = "demos/textarea");
rsx::module!(pub toggle = "demos/toggle");
rsx::module!(pub toggle_group = "demos/toggle_group");
rsx::module!(pub tooltip = "demos/tooltip");

pub type Demo = fn() -> rsx::Element;

/// Every page, in sidebar order.
pub const DEMOS: &[(&str, Demo)] = &[
    ("Aspect Ratio", aspect_ratio::demo),
    ("Avatar", avatar::demo),
    ("Badge", badge::demo),
    ("Button", button::demo),
    ("Calendar", calendar::demo),
    ("Chart", chart::demo),
    ("Checkbox", checkbox::demo),
    ("Combobox", combobox::demo),
    ("Command", command::demo),
    ("Date Picker", date_picker::demo),
    ("Field", field::demo),
    ("Form", form::demo),
    ("Input", input::demo),
    ("Input Group", input_group::demo),
    ("Input OTP", input_otp::demo),
    ("Kbd", kbd::demo),
    ("Label", label::demo),
    ("Popover", popover::demo),
    ("Progress", progress::demo),
    ("Radio Group", radio_group::demo),
    ("Separator", separator::demo),
    ("Sidebar", sidebar::demo),
    ("Skeleton", skeleton::demo),
    ("Slider", slider::demo),
    ("Spinner", spinner::demo),
    ("Switch", switch::demo),
    ("Textarea", textarea::demo),
    ("Toggle", toggle::demo),
    ("Toggle Group", toggle_group::demo),
    ("Tooltip", tooltip::demo),
];

/// One-line summary shown under each page title (from the shadcn docs).
pub fn description(name: &str) -> &'static str {
    match name {
        "Overview" => "Many components working together, in the spirit of the examples on ui.shadcn.com.",
        "Accordion" => "A vertically stacked set of interactive headings that each reveal a section of content.",
        "Alert" => "Displays a callout for user attention.",
        "Alert Dialog" => "A modal dialog that interrupts the user with important content and expects a response.",
        "Aspect Ratio" => "Displays content within a desired ratio.",
        "Avatar" => "An image element with a fallback for representing the user.",
        "Badge" => "Displays a badge or a component that looks like a badge.",
        "Breadcrumb" => "Displays the path to the current resource using a hierarchy of links.",
        "Button" => "Displays a button or a component that looks like a button.",
        "Button Group" => "A container that groups related buttons together with consistent styling.",
        "Calendar" => "A date field component that allows users to enter and edit dates.",
        "Card" => "Displays a card with header, content, and footer.",
        "Carousel" => "A carousel that cycles through a set of slides.",
        "Chart" => "Bar, area, line and pie charts that pick up your theme's chart colors.",
        "Checkbox" => "A control that allows the user to toggle between checked and not checked.",
        "Collapsible" => "An interactive component which expands and collapses a panel.",
        "Combobox" => "Autocomplete input and command palette with a list of suggestions.",
        "Command" => "Fast, composable command menu with fuzzy filtering and keyboard navigation.",
        "Context Menu" => "Displays a menu of actions or functions, triggered by a right click.",
        "Data Table" => "A table with sorting, filtering, pagination, row selection and column visibility.",
        "Date Picker" => "A date picker component with single and range selection.",
        "Dialog" => "A window overlaid on the primary window, rendering the content underneath inert.",
        "Drawer" => "A panel that slides up from the bottom edge of the window.",
        "Dropdown Menu" => "Displays a menu of actions or functions, triggered by a button.",
        "Empty" => "Use the Empty component to display an empty state.",
        "Field" => "Combine labels, controls, and help text to compose accessible form fields.",
        "Form" => "Building forms with validation and error messages.",
        "Hover Card" => "For sighted users to preview content available behind a link.",
        "Input" => "Displays a form input field or a component that looks like an input field.",
        "Input Group" => "Display additional information or actions next to an input or textarea.",
        "Input OTP" => "Accessible one-time password component with copy paste functionality.",
        "Item" => "A versatile component that you can use to display any content.",
        "Kbd" => "Used to display textual user input from the keyboard.",
        "Label" => "Renders an accessible label associated with controls.",
        "Menubar" => "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands.",
        "Native Select" => "A styled select for simple option lists.",
        "Navigation Menu" => "A collection of links for navigating websites.",
        "Pagination" => "Pagination with page navigation, next and previous links.",
        "Popover" => "Displays rich content in a portal, triggered by a button.",
        "Progress" => "Displays an indicator showing the completion progress of a task.",
        "Radio Group" => "A set of checkable buttons where no more than one can be checked at a time.",
        "Resizable" => "Resizable panel groups and layouts.",
        "Scroll Area" => "Augments native scroll functionality with custom, themed scrollbars.",
        "Select" => "Displays a list of options for the user to pick from, triggered by a button.",
        "Separator" => "Visually or semantically separates content.",
        "Sheet" => "Extends the Dialog component to display content that complements the main content of the screen.",
        "Sidebar" => "A composable, themeable and customizable sidebar component.",
        "Skeleton" => "Use to show a placeholder while content is loading.",
        "Slider" => "An input where the user selects a value from within a given range.",
        "Sonner" => "An opinionated toast component.",
        "Spinner" => "An indicator that can be used to show a loading state.",
        "Switch" => "A control that allows the user to toggle between checked and not checked.",
        "Table" => "A responsive table component.",
        "Tabs" => "A set of layered sections of content, known as tab panels, that are displayed one at a time.",
        "Textarea" => "Displays a form textarea or a component that looks like a textarea.",
        "Toast" => "A succinct message that is displayed temporarily.",
        "Toggle" => "A two-state button that can be either on or off.",
        "Toggle Group" => "A set of two-state buttons that can be toggled on or off.",
        "Tooltip" => "A popup that displays information related to an element when the mouse hovers over it.",
        "Typography" => "Styles for headings, paragraphs, lists and more.",
        _ => "",
    }
}

//! One demo per component, mirroring the examples on ui.shadcn.com.

use gpui::{AnyElement, App, IntoElement, ParentElement, SharedString, Styled, Window, div, px};
use std::rc::Rc;

use crate::ui::{
    accordion::{Accordion, AccordionItem},
    alert::Alert,
    alert_dialog::AlertDialog,
    aspect_ratio::AspectRatio,
    avatar::Avatar,
    badge::Badge,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonSize, ButtonVariant},
    button_group::ButtonGroup,
    calendar::{Calendar, CalendarMode},
    card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle},
    carousel::Carousel,
    chart::{AreaChart, BarChart, ChartSeries, LineChart, PieChart},
    checkbox::Checkbox,
    collapsible::Collapsible,
    combobox::Combobox,
    command::{Command, CommandGroup, CommandItem},
    context_menu::ContextMenu,
    data_table::{DataTable, DataTableColumn},
    date_picker::DatePicker,
    dialog::Dialog,
    drawer::Drawer,
    dropdown_menu::{DropdownMenu, MenuBuilder, MenuItem},
    empty::Empty,
    field::{Field, FieldGroup, FieldSeparator, FieldSet},
    form::{Form, FormField},
    hover_card::HoverCard,
    icon::{Icon, IconName},
    input::Input,
    input_group::{InputGroup, InputGroupAddon},
    input_otp::InputOtp,
    item::{Item, ItemGroup},
    kbd::{Kbd, KbdGroup},
    label::Label,
    menubar::{Menubar, MenubarMenu},
    native_select::NativeSelect,
    navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuLink},
    pagination::Pagination,
    popover::Popover,
    progress::Progress,
    radio_group::{RadioGroup, RadioGroupItem},
    resizable::{ResizablePanel, ResizablePanelGroup},
    scroll_area::ScrollArea,
    select::{Select, SelectGroup, SelectItem},
    separator::Separator,
    sheet::Sheet,
    sidebar::{Sidebar, SidebarGroup, SidebarInset, SidebarMenuItem, SidebarTrigger},
    skeleton::Skeleton,
    slider::Slider,
    sonner::{Toast, toast},
    spinner::Spinner,
    switch::Switch,
    table::{Table, TableBody, TableCell, TableFooter, TableHead, TableHeader, TableRow},
    tabs::{Tab, Tabs},
    textarea::Textarea,
    theme::ActiveTheme,
    toggle::Toggle,
    toggle_group::{ToggleGroup, ToggleGroupItem},
    tooltip::Tooltip,
    typography::{List, blockquote, h1, h2, h3, h4, inline_code, large, lead, muted, p, small},
    utils::Side,
};

type Demo = fn(&mut Window, &mut App) -> AnyElement;

pub const DEMOS: &[(&str, Demo)] = &[
    ("Accordion", accordion),
    ("Alert", alert),
    ("Alert Dialog", alert_dialog),
    ("Aspect Ratio", aspect_ratio),
    ("Avatar", avatar),
    ("Badge", badge),
    ("Breadcrumb", breadcrumb),
    ("Button", button),
    ("Button Group", button_group),
    ("Calendar", calendar),
    ("Card", card),
    ("Carousel", carousel),
    ("Chart", chart),
    ("Checkbox", checkbox),
    ("Collapsible", collapsible),
    ("Combobox", combobox),
    ("Command", command),
    ("Context Menu", context_menu),
    ("Data Table", data_table),
    ("Date Picker", date_picker),
    ("Dialog", dialog),
    ("Drawer", drawer),
    ("Dropdown Menu", dropdown_menu),
    ("Empty", empty),
    ("Field", field),
    ("Form", form),
    ("Hover Card", hover_card),
    ("Input", input),
    ("Input Group", input_group),
    ("Input OTP", input_otp),
    ("Item", item),
    ("Kbd", kbd),
    ("Label", label),
    ("Menubar", menubar),
    ("Native Select", native_select),
    ("Navigation Menu", navigation_menu),
    ("Pagination", pagination),
    ("Popover", popover),
    ("Progress", progress),
    ("Radio Group", radio_group),
    ("Resizable", resizable),
    ("Scroll Area", scroll_area),
    ("Select", select),
    ("Separator", separator),
    ("Sheet", sheet),
    ("Sidebar", sidebar),
    ("Skeleton", skeleton),
    ("Slider", slider),
    ("Sonner", sonner),
    ("Spinner", spinner),
    ("Switch", switch),
    ("Table", table),
    ("Tabs", tabs),
    ("Textarea", textarea),
    ("Toast", toast_demo),
    ("Toggle", toggle),
    ("Toggle Group", toggle_group),
    ("Tooltip", tooltip),
    ("Typography", typography),
];

fn row() -> gpui::Div {
    div().flex().flex_wrap().items_center().gap_2()
}

fn col() -> gpui::Div {
    div().flex().flex_col().gap_4()
}

fn accordion(_: &mut Window, _: &mut App) -> AnyElement {
    Accordion::new("accordion")
        .default_value(["item-1"])
        .w(px(480.))
        .item(AccordionItem::new("item-1", "Product Information").content(
            col()
                .child("Our flagship product combines cutting-edge technology with sleek design. Built with premium materials, it offers unparalleled performance and reliability.")
                .child("Key features include advanced processing capabilities, and an intuitive user interface designed for both beginners and experts."),
        ))
        .item(AccordionItem::new("item-2", "Shipping Details").content(
            div().child("We offer worldwide shipping through trusted courier partners. Standard delivery takes 3-5 business days."),
        ))
        .item(AccordionItem::new("item-3", "Return Policy").content(
            div().child("We stand behind our products with a comprehensive 30-day return policy."),
        ))
        .into_any_element()
}

fn alert(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .w(px(560.))
        .child(
            Alert::new()
                .icon(IconName::CircleCheck)
                .title("Success! Your changes have been saved")
                .description("This is an alert with icon, title and description."),
        )
        .child(Alert::new().icon(IconName::Info).title("This Alert has a title and an icon. No description."))
        .child(
            Alert::new()
                .destructive()
                .icon(IconName::CircleAlert)
                .title("Unable to process your payment.")
                .description("Please verify your billing information and try again."),
        )
        .into_any_element()
}

fn alert_dialog(_: &mut Window, _: &mut App) -> AnyElement {
    AlertDialog::new("alert-dialog")
        .trigger(Button::new("open").outline().label("Show Dialog"))
        .title("Are you absolutely sure?")
        .description("This action cannot be undone. This will permanently delete your account and remove your data from our servers.")
        .action("Continue", |_, cx| {
            toast("Account deleted", cx);
        })
        .into_any_element()
}

fn aspect_ratio(_: &mut Window, cx: &mut App) -> AnyElement {
    let muted = cx.theme().muted;
    AspectRatio::new(16. / 9.)
        .w(px(450.))
        .rounded_lg()
        .bg(muted)
        .child(div().size_full().flex().items_center().justify_center().child("16 / 9"))
        .into_any_element()
}

fn avatar(_: &mut Window, _: &mut App) -> AnyElement {
    row()
        .gap_4()
        .child(Avatar::new().fallback("CN"))
        .child(Avatar::new().fallback("ER").rounded_lg())
        .child(Avatar::new().fallback("LR").size_12())
        .into_any_element()
}

fn badge(_: &mut Window, _: &mut App) -> AnyElement {
    row()
        .child(Badge::new("Badge"))
        .child(Badge::new("Secondary").secondary())
        .child(Badge::new("Destructive").destructive())
        .child(Badge::new("Outline").outline())
        .child(Badge::new("Verified").secondary().icon(IconName::CircleCheck))
        .child(Badge::new("8").rounded_full().px_1())
        .into_any_element()
}

fn breadcrumb(_: &mut Window, _: &mut App) -> AnyElement {
    Breadcrumb::new()
        .item(BreadcrumbItem::new("Home").on_click(|_, _, _| {}))
        .ellipsis()
        .item(BreadcrumbItem::new("Components").on_click(|_, _, _| {}))
        .item(BreadcrumbItem::new("Breadcrumb").current())
        .into_any_element()
}

fn button(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .child(
            row()
                .child(Button::new("default").label("Button"))
                .child(Button::new("secondary").secondary().label("Secondary"))
                .child(Button::new("destructive").destructive().label("Destructive"))
                .child(Button::new("outline").outline().label("Outline"))
                .child(Button::new("ghost").ghost().label("Ghost"))
                .child(Button::new("link").link().label("Link")),
        )
        .child(
            row()
                .child(Button::new("sm").size(ButtonSize::Sm).outline().label("Small"))
                .child(Button::new("lg").size(ButtonSize::Lg).outline().label("Large"))
                .child(Button::new("icon").outline().size(ButtonSize::Icon).icon(IconName::ArrowUp))
                .child(Button::new("with-icon").outline().icon(IconName::Mail).label("Login with Email"))
                .child(Button::new("loading").size(ButtonSize::Sm).outline().loading(true).label("Please wait"))
                .child(Button::new("disabled").disabled(true).label("Disabled"))
                .child(Button::new("toast").outline().label("Click me").tooltip("Shows a toast").on_click(|_, _, cx| {
                    toast("Button clicked", cx);
                })),
        )
        .into_any_element()
}

fn button_group(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .child(
            ButtonGroup::new()
                .button(Button::new("archive").outline().label("Archive"))
                .button(Button::new("report").outline().label("Report"))
                .button(Button::new("snooze").outline().label("Snooze"))
                .button(Button::new("more").outline().size(ButtonSize::Icon).icon(IconName::Ellipsis)),
        )
        .child(
            ButtonGroup::new()
                .text("https://")
                .element(Input::new("bg-url").placeholder("example.com").rounded_none().w(px(200.)))
                .button(Button::new("go").outline().label("Go")),
        )
        .child(
            ButtonGroup::new()
                .button(Button::new("dec").secondary().size(ButtonSize::Icon).icon(IconName::Minus))
                .separator()
                .button(Button::new("inc").secondary().size(ButtonSize::Icon).icon(IconName::Plus)),
        )
        .into_any_element()
}

fn calendar(_: &mut Window, cx: &mut App) -> AnyElement {
    let border = cx.theme().border;
    row()
        .items_start()
        .gap_6()
        .child(Calendar::new("calendar").rounded_md().border_1().border_color(border))
        .child(
            Calendar::new("range-calendar")
                .mode(CalendarMode::Range)
                .months(2)
                .rounded_md()
                .border_1()
                .border_color(border),
        )
        .into_any_element()
}

fn card(_: &mut Window, _: &mut App) -> AnyElement {
    Card::new()
        .w(px(380.))
        .child(
            CardHeader::new()
                .child(CardTitle::new("Login to your account"))
                .child(CardDescription::new("Enter your email below to login to your account"))
                .action(Button::new("signup").link().label("Sign Up")),
        )
        .child(
            CardContent::new().child(
                col()
                    .gap_6()
                    .child(Field::new().label("Email").child(Input::new("card-email").placeholder("m@example.com")))
                    .child(Field::new().label("Password").child(Input::new("card-password").masked(true))),
            ),
        )
        .child(
            CardFooter::new()
                .flex_col()
                .child(Button::new("login").label("Login").w_full())
                .child(Button::new("google").outline().label("Login with Google").w_full()),
        )
        .into_any_element()
}

fn carousel(_: &mut Window, _: &mut App) -> AnyElement {
    div()
        .px_12()
        .child(
            Carousel::new("carousel").w(px(320.)).items((1..=5).map(|n| {
                Card::new().child(
                    CardContent::new()
                        .flex()
                        .h(px(220.))
                        .items_center()
                        .justify_center()
                        .text_3xl()
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child(n.to_string()),
                )
            })),
        )
        .into_any_element()
}

fn chart(_: &mut Window, _: &mut App) -> AnyElement {
    let months = ["January", "February", "March", "April", "May", "June"];
    let desktop = [186., 305., 237., 73., 209., 214.];
    let mobile = [80., 200., 120., 190., 130., 140.];
    let card = |title: &'static str, chart: AnyElement| {
        Card::new()
            .w(px(400.))
            .child(CardHeader::new().child(CardTitle::new(title)).child(CardDescription::new("January - June 2024")))
            .child(CardContent::new().child(chart))
    };
    div()
        .flex()
        .flex_wrap()
        .gap_4()
        .child(card(
            "Bar Chart - Multiple",
            BarChart::new("bar")
                .labels(months)
                .series(ChartSeries::new("Desktop").data(desktop))
                .series(ChartSeries::new("Mobile").data(mobile))
                .into_any_element(),
        ))
        .child(card(
            "Area Chart - Stacked",
            AreaChart::new("area")
                .labels(months)
                .stacked()
                .series(ChartSeries::new("Desktop").data(desktop))
                .series(ChartSeries::new("Mobile").data(mobile))
                .into_any_element(),
        ))
        .child(card(
            "Line Chart - Dots",
            LineChart::new("line").labels(months).dots(true).series(ChartSeries::new("Desktop").data(desktop)).into_any_element(),
        ))
        .child(card(
            "Pie Chart - Donut with Text",
            PieChart::new("pie")
                .slice("Chrome", 275.)
                .slice("Safari", 200.)
                .slice("Firefox", 187.)
                .slice("Edge", 173.)
                .slice("Other", 90.)
                .donut(0.6)
                .size(px(220.))
                .center_label("925", "Visitors")
                .into_any_element(),
        ))
        .into_any_element()
}

fn checkbox(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .child(Checkbox::new("terms").label("Accept terms and conditions"))
        .child(Checkbox::new("checked").default_checked(true).label("Enable notifications"))
        .child(Checkbox::new("indeterminate").indeterminate(true).label("Select all"))
        .child(Checkbox::new("disabled").disabled(true).label("Disabled"))
        .into_any_element()
}

fn collapsible(_: &mut Window, cx: &mut App) -> AnyElement {
    let border = cx.theme().border;
    let repo = move |name: &'static str| {
        div()
            .rounded_md()
            .border_1()
            .border_color(border)
            .px_4()
            .py_2()
            .font_family("monospace")
            .text_sm()
            .child(name)
    };
    Collapsible::new("collapsible")
        .w(px(350.))
        .gap_2()
        .trigger(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_4()
                .px_4()
                .child(div().text_sm().font_weight(gpui::FontWeight::SEMIBOLD).child("@peduarte starred 3 repositories"))
                .child(Button::new("toggle").ghost().size(ButtonSize::IconSm).icon(IconName::ChevronsUpDown)),
        )
        .child(repo("@radix-ui/primitives"))
        .content(col().gap_2().child(repo("@radix-ui/colors")).child(repo("@stitches/react")))
        .into_any_element()
}

fn frameworks() -> [(&'static str, &'static str); 5] {
    [
        ("next.js", "Next.js"),
        ("sveltekit", "SvelteKit"),
        ("nuxt.js", "Nuxt.js"),
        ("remix", "Remix"),
        ("astro", "Astro"),
    ]
}

fn combobox(_: &mut Window, _: &mut App) -> AnyElement {
    let mut combobox = Combobox::new("framework")
        .placeholder("Select framework...")
        .search_placeholder("Search framework...")
        .empty("No framework found.");
    for (value, label) in frameworks() {
        combobox = combobox.item(value, label);
    }
    combobox.into_any_element()
}

fn command(_: &mut Window, cx: &mut App) -> AnyElement {
    let border = cx.theme().border;
    Command::new("command")
        .w(px(450.))
        .rounded_lg()
        .border_1()
        .border_color(border)
        .group(
            CommandGroup::new("Suggestions")
                .item(CommandItem::new("Calendar").icon(IconName::Calendar))
                .item(CommandItem::new("Search Emoji").icon(IconName::Search))
                .item(CommandItem::new("Calculator").icon(IconName::Terminal).disabled(true)),
        )
        .group(
            CommandGroup::new("Settings")
                .item(CommandItem::new("Profile").icon(IconName::User).shortcut("⌘P"))
                .item(CommandItem::new("Billing").icon(IconName::CreditCard).shortcut("⌘B"))
                .item(CommandItem::new("Settings").icon(IconName::Settings).shortcut("⌘S")),
        )
        .on_select(|value, _, cx| {
            toast(format!("Selected {value}"), cx);
        })
        .into_any_element()
}

fn context_menu(_: &mut Window, cx: &mut App) -> AnyElement {
    let border = cx.theme().border;
    ContextMenu::new("context")
        .trigger(
            div()
                .flex()
                .h(px(150.))
                .w(px(300.))
                .items_center()
                .justify_center()
                .rounded_md()
                .border_1()
                .border_dashed()
                .border_color(border)
                .text_sm()
                .child("Right click here"),
        )
        .item(MenuItem::new("Back").inset().shortcut("⌘["))
        .item(MenuItem::new("Forward").inset().shortcut("⌘]").disabled(true))
        .item(MenuItem::new("Reload").inset().shortcut("⌘R"))
        .submenu(
            "More Tools",
            vec![
                MenuItem::new("Save Page...").into(),
                MenuItem::new("Create Shortcut...").into(),
                MenuItem::new("Developer Tools").into(),
            ],
        )
        .separator()
        .item(MenuItem::checkbox("Show Bookmarks", true))
        .item(MenuItem::checkbox("Show Full URLs", false))
        .separator()
        .label("People")
        .item(MenuItem::radio("Pedro Duarte", true))
        .item(MenuItem::radio("Colm Tuite", false))
        .into_any_element()
}

#[derive(Clone)]
struct Payment {
    id: &'static str,
    amount: f64,
    status: &'static str,
    email: &'static str,
}

fn payments() -> Vec<Payment> {
    vec![
        Payment { id: "m5gr84i9", amount: 316., status: "success", email: "ken99@example.com" },
        Payment { id: "3u1reuv4", amount: 242., status: "success", email: "Abe45@example.com" },
        Payment { id: "derv1ws0", amount: 837., status: "processing", email: "Monserrat44@example.com" },
        Payment { id: "5kma53ae", amount: 874., status: "success", email: "Silas22@example.com" },
        Payment { id: "bhqecj4p", amount: 721., status: "failed", email: "carmella@example.com" },
        Payment { id: "p0r8sd2x", amount: 129., status: "pending", email: "jason@example.com" },
        Payment { id: "a1b2c3d4", amount: 450., status: "success", email: "olivia@example.com" },
    ]
}

fn data_table(_: &mut Window, _: &mut App) -> AnyElement {
    DataTable::new("payments", Rc::new(payments()))
        .w(px(720.))
        .selectable()
        .page_size(5)
        .filter("Filter emails...", |p: &Payment| p.email.to_string())
        .column(DataTableColumn::new("status", "Status", |p: &Payment, _, _| p.status).width(px(120.)))
        .column(DataTableColumn::new("email", "Email", |p: &Payment, _, _| p.email).sortable_by(|p| p.email.to_lowercase()))
        .column(
            DataTableColumn::new("amount", "Amount", |p: &Payment, _, _| format!("${:.2}", p.amount))
                .sortable_by(|p| (p.amount * 100.) as i64)
                .align_right()
                .width(px(120.)),
        )
        .row_actions(|p: &Payment| {
            let id = p.id;
            vec![
                MenuItem::new("Copy payment ID").on_select(move |_, cx| {
                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(id.to_string()));
                })
                .into(),
                crate::ui::dropdown_menu::MenuEntry::Separator,
                MenuItem::new("View customer").into(),
                MenuItem::new("View payment details").into(),
            ]
        })
        .into_any_element()
}

fn date_picker(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .child(DatePicker::new("date"))
        .child(DatePicker::new("range").mode(CalendarMode::Range).months(2).placeholder("Pick a date range"))
        .into_any_element()
}

fn dialog(_: &mut Window, _: &mut App) -> AnyElement {
    Dialog::new("dialog")
        .trigger(Button::new("open").outline().label("Edit Profile"))
        .title("Edit profile")
        .description("Make changes to your profile here. Click save when you're done.")
        .content(|_, _, _| {
            col()
                .child(Field::new().label("Name").child(Input::new("name").default_value("Pedro Duarte")))
                .child(Field::new().label("Username").child(Input::new("username").default_value("@peduarte")))
                .child(
                    Field::new().label("Theme").child(
                        Select::new("dialog-select")
                            .w_full()
                            .default_value("system")
                            .item(SelectItem::new("light", "Light"))
                            .item(SelectItem::new("dark", "Dark"))
                            .item(SelectItem::new("system", "System")),
                    ),
                )
        })
        .footer(|dialog, _, _| {
            let (cancel, save) = (dialog.clone(), dialog.clone());
            row()
                .child(Button::new("cancel").outline().label("Cancel").on_click(move |_, window, cx| cancel.close(window, cx)))
                .child(Button::new("save").label("Save changes").on_click(move |_, window, cx| {
                    save.close(window, cx);
                    toast(Toast::new("Profile updated").success(), cx);
                }))
        })
        .into_any_element()
}

fn drawer(_: &mut Window, _: &mut App) -> AnyElement {
    Drawer::new("drawer")
        .trigger(Button::new("open").outline().label("Open Drawer"))
        .title("Move Goal")
        .description("Set your daily activity goal.")
        .content(|_, _, cx| {
            let muted = cx.theme().muted_foreground;
            div()
                .flex()
                .flex_col()
                .items_center()
                .py_4()
                .child(div().text_size(px(72.)).font_weight(gpui::FontWeight::BOLD).child("350"))
                .child(div().text_xs().text_color(muted).child("CALORIES/DAY"))
        })
        .footer(|drawer, _, _| {
            let close = drawer.clone();
            col()
                .gap_2()
                .child(Button::new("submit").label("Submit").w_full())
                .child(Button::new("cancel").outline().label("Cancel").w_full().on_click(move |_, window, cx| close.close(window, cx)))
        })
        .into_any_element()
}

fn dropdown_menu(_: &mut Window, _: &mut App) -> AnyElement {
    DropdownMenu::new("dropdown")
        .trigger(Button::new("open").outline().label("Open"))
        .label("My Account")
        .item(MenuItem::new("Profile").shortcut("⇧⌘P"))
        .item(MenuItem::new("Billing").shortcut("⌘B"))
        .item(MenuItem::new("Settings").shortcut("⌘S"))
        .item(MenuItem::new("Keyboard shortcuts").shortcut("⌘K"))
        .separator()
        .item(MenuItem::new("Team"))
        .submenu("Invite users", vec![MenuItem::new("Email").into(), MenuItem::new("Message").into(), MenuItem::new("More...").into()])
        .item(MenuItem::new("New Team").shortcut("⌘+T"))
        .separator()
        .item(MenuItem::new("GitHub"))
        .item(MenuItem::new("Support"))
        .item(MenuItem::new("API").disabled(true))
        .separator()
        .item(MenuItem::new("Log out").icon(IconName::LogOut).destructive().shortcut("⇧⌘Q"))
        .w(px(224.))
        .into_any_element()
}

fn empty(_: &mut Window, _: &mut App) -> AnyElement {
    Empty::new()
        .outline()
        .w(px(560.))
        .icon(IconName::Folder)
        .title("No Projects Yet")
        .description("You haven't created any projects yet. Get started by creating your first project.")
        .child(row().child(Button::new("create").label("Create Project")).child(Button::new("import").outline().label("Import Project")))
        .into_any_element()
}

fn field(_: &mut Window, _: &mut App) -> AnyElement {
    div()
        .w(px(420.))
        .child(
            FieldSet::new()
                .legend("Payment Method")
                .description("All transactions are secure and encrypted")
                .child(
                    FieldGroup::new()
                        .child(Field::new().label("Name on Card").child(Input::new("field-name").placeholder("Evil Rabbit")))
                        .child(
                            Field::new()
                                .label("Card Number")
                                .description("Enter your 16-digit card number")
                                .child(Input::new("field-card").placeholder("1234 5678 9012 3456")),
                        )
                        .child(
                            row()
                                .gap_4()
                                .flex_nowrap()
                                .child(Field::new().label("Month").child(
                                    NativeSelect::new("field-month").option("01", "01").option("02", "02").option("03", "03"),
                                ))
                                .child(Field::new().label("CVV").child(Input::new("field-cvv").placeholder("123"))),
                        )
                        .child(FieldSeparator::new())
                        .child(Field::new().horizontal().label("Same as shipping address").child(Checkbox::new("field-same").default_checked(true))),
                ),
        )
        .into_any_element()
}

fn form(_: &mut Window, _: &mut App) -> AnyElement {
    Form::new("profile-form")
        .w(px(420.))
        .field(
            FormField::text("username")
                .label("Username")
                .placeholder("shadcn")
                .description("This is your public display name.")
                .validate(|value| (value.chars().count() < 2).then(|| "Username must be at least 2 characters.".into())),
        )
        .field(
            FormField::text("email")
                .label("Email")
                .placeholder("m@example.com")
                .validate(|value| (!value.contains('@')).then(|| "Please enter a valid email.".into())),
        )
        .field(FormField::textarea("bio").label("Bio").placeholder("Tell us a little bit about yourself"))
        .on_submit(|values, _, cx| {
            let summary: Vec<String> = values.iter().map(|(k, v)| format!("{k}: {v}")).collect();
            toast(Toast::new("You submitted the following values").description(summary.join(", ")), cx);
        })
        .into_any_element()
}

fn hover_card(_: &mut Window, cx: &mut App) -> AnyElement {
    let muted = cx.theme().muted_foreground;
    HoverCard::new("hover-card")
        .trigger(Button::new("trigger").link().label("@nextjs"))
        .content(move |_, _| {
            row()
                .items_start()
                .gap_4()
                .flex_nowrap()
                .child(Avatar::new().fallback("VC"))
                .child(
                    col()
                        .gap_1()
                        .child(div().text_sm().font_weight(gpui::FontWeight::SEMIBOLD).child("@nextjs"))
                        .child(div().text_sm().child("The React Framework – created and maintained by @vercel."))
                        .child(div().text_xs().text_color(muted).child("Joined December 2021")),
                )
        })
        .into_any_element()
}

fn input(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .w(px(320.))
        .child(Input::new("input-email").placeholder("Email"))
        .child(Input::new("input-password").placeholder("Password").masked(true))
        .child(Input::new("input-disabled").placeholder("Disabled").disabled(true))
        .child(Input::new("input-invalid").placeholder("Invalid").invalid(true))
        .child(
            row()
                .flex_nowrap()
                .child(Input::new("input-subscribe").placeholder("Email"))
                .child(Button::new("subscribe").outline().label("Subscribe")),
        )
        .into_any_element()
}

fn input_group(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .w(px(380.))
        .child(
            InputGroup::new(Input::new("ig-search").placeholder("Search..."))
                .start(InputGroupAddon::icon(IconName::Search))
                .end(InputGroupAddon::text("12 results")),
        )
        .child(
            InputGroup::new(Input::new("ig-url").placeholder("example.com"))
                .start(InputGroupAddon::text("https://"))
                .end(InputGroupAddon::button(Button::new("ig-copy").ghost().size(ButtonSize::IconSm).icon(IconName::Copy))),
        )
        .child(
            InputGroup::new(Input::new("ig-message").multiline(true).placeholder("Ask, Search or Chat..."))
                .block_end(InputGroupAddon::element(
                    row()
                        .w_full()
                        .justify_between()
                        .child(Button::new("ig-plus").outline().size(ButtonSize::IconSm).rounded_full().icon(IconName::Plus))
                        .child(Button::new("ig-send").size(ButtonSize::IconSm).rounded_full().icon(IconName::ArrowUp)),
                )),
        )
        .into_any_element()
}

fn input_otp(_: &mut Window, _: &mut App) -> AnyElement {
    InputOtp::new("otp")
        .length(6)
        .groups(&[3, 3])
        .on_complete(|code, _, cx| {
            toast(format!("Code entered: {code}"), cx);
        })
        .into_any_element()
}

fn item(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .w(px(480.))
        .child(
            Item::new("item-basic")
                .outline()
                .title("Basic Item")
                .description("A simple item with title and description.")
                .action(Button::new("action").outline().size(ButtonSize::Sm).label("Action")),
        )
        .child(
            Item::new("item-verified")
                .outline()
                .icon(IconName::CircleCheck)
                .title("Your profile has been verified.")
                .action(Icon::new(IconName::ChevronRight)),
        )
        .child(
            ItemGroup::new()
                .separated()
                .child(Item::new("shadcn").media(Avatar::new().fallback("S")).title("shadcn").description("shadcn@vercel.com"))
                .child(Item::new("maxleiter").media(Avatar::new().fallback("M")).title("maxleiter").description("maxleiter@vercel.com")),
        )
        .into_any_element()
}

fn kbd(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .child(KbdGroup::new().child(Kbd::new("⌘")).child(Kbd::new("⇧")).child(Kbd::new("⌥")).child(Kbd::new("⌃")))
        .child(KbdGroup::new().child(Kbd::new("Ctrl")).child("+").child(Kbd::new("B")))
        .into_any_element()
}

fn label(_: &mut Window, _: &mut App) -> AnyElement {
    row().gap_2().child(Checkbox::new("label-terms")).child(Label::new("Accept terms and conditions")).into_any_element()
}

fn menubar(_: &mut Window, _: &mut App) -> AnyElement {
    Menubar::new("menubar")
        .menu(
            MenubarMenu::new("File")
                .item(MenuItem::new("New Tab").shortcut("⌘T"))
                .item(MenuItem::new("New Window").shortcut("⌘N"))
                .item(MenuItem::new("New Incognito Window").disabled(true))
                .separator()
                .submenu("Share", vec![MenuItem::new("Email link").into(), MenuItem::new("Messages").into(), MenuItem::new("Notes").into()])
                .separator()
                .item(MenuItem::new("Print...").shortcut("⌘P")),
        )
        .menu(
            MenubarMenu::new("Edit")
                .item(MenuItem::new("Undo").shortcut("⌘Z"))
                .item(MenuItem::new("Redo").shortcut("⇧⌘Z"))
                .separator()
                .item(MenuItem::new("Cut"))
                .item(MenuItem::new("Copy"))
                .item(MenuItem::new("Paste")),
        )
        .menu(
            MenubarMenu::new("View")
                .item(MenuItem::checkbox("Always Show Bookmarks Bar", false))
                .item(MenuItem::checkbox("Always Show Full URLs", true))
                .separator()
                .item(MenuItem::new("Reload").inset().shortcut("⌘R"))
                .item(MenuItem::new("Toggle Fullscreen").inset()),
        )
        .menu(
            MenubarMenu::new("Profiles")
                .item(MenuItem::radio("Andy", false))
                .item(MenuItem::radio("Benoit", true))
                .item(MenuItem::radio("Luis", false))
                .separator()
                .item(MenuItem::new("Edit...").inset())
                .item(MenuItem::new("Add Profile...").inset()),
        )
        .into_any_element()
}

fn native_select(_: &mut Window, _: &mut App) -> AnyElement {
    NativeSelect::new("native")
        .w(px(240.))
        .placeholder("Select status")
        .option("todo", "Todo")
        .option("in-progress", "In Progress")
        .option("done", "Done")
        .disabled_option("cancelled", "Cancelled")
        .into_any_element()
}

fn navigation_menu(_: &mut Window, _: &mut App) -> AnyElement {
    NavigationMenu::new("nav")
        .item(NavigationMenuItem::new("Getting started").content(|_, _| {
            div()
                .w(px(420.))
                .grid()
                .grid_cols(2)
                .gap_1()
                .child(NavigationMenuLink::new("Introduction").description("Re-usable components built using Radix UI and Tailwind CSS."))
                .child(NavigationMenuLink::new("Installation").description("How to install dependencies and structure your app."))
                .child(NavigationMenuLink::new("Typography").description("Styles for headings, paragraphs, lists...etc"))
                .child(NavigationMenuLink::new("Theming").description("Restyle everything from a single file."))
        }))
        .item(NavigationMenuItem::new("Components").content(|_, _| {
            div()
                .w(px(420.))
                .grid()
                .grid_cols(2)
                .gap_1()
                .child(NavigationMenuLink::new("Alert Dialog").description("A modal dialog that interrupts the user."))
                .child(NavigationMenuLink::new("Hover Card").description("Preview content available behind a link."))
                .child(NavigationMenuLink::new("Progress").description("Displays an indicator showing completion."))
                .child(NavigationMenuLink::new("Tabs").description("Layered sections of content."))
        }))
        .item(NavigationMenuItem::new("Docs").on_click(|_, _, cx| {
            toast("Docs clicked", cx);
        }))
        .into_any_element()
}

fn pagination(window: &mut Window, cx: &mut App) -> AnyElement {
    let page = window.use_keyed_state("pagination-page", cx, |_, _| 2usize);
    let current = *page.read(cx);
    Pagination::new("pagination")
        .total(10)
        .page(current)
        .on_change(move |next, _, cx| {
            page.update(cx, |p, cx| {
                *p = *next;
                cx.notify();
            })
        })
        .into_any_element()
}

fn popover(_: &mut Window, cx: &mut App) -> AnyElement {
    let muted = cx.theme().muted_foreground;
    Popover::new("popover")
        .trigger(Button::new("open").outline().label("Open popover"))
        .w(px(320.))
        .content(move |_, _| {
            col()
                .child(
                    col()
                        .gap_2()
                        .child(div().font_weight(gpui::FontWeight::MEDIUM).child("Dimensions"))
                        .child(div().text_sm().text_color(muted).child("Set the dimensions for the layer.")),
                )
                .children(["Width", "Max. width", "Height", "Max. height"].map(|label| {
                    row()
                        .flex_nowrap()
                        .gap_4()
                        .child(Label::new(label).w(px(80.)))
                        .child(Input::new(SharedString::from(format!("popover-{label}"))).default_value("100%").h_8())
                }))
        })
        .into_any_element()
}

fn progress(_: &mut Window, _: &mut App) -> AnyElement {
    col().w(px(300.)).child(Progress::new(13.)).child(Progress::new(66.)).child(Progress::new(100.)).into_any_element()
}

fn radio_group(_: &mut Window, _: &mut App) -> AnyElement {
    RadioGroup::new("radio")
        .default_value("comfortable")
        .item(RadioGroupItem::new("default", "Default"))
        .item(RadioGroupItem::new("comfortable", "Comfortable"))
        .item(RadioGroupItem::new("compact", "Compact"))
        .into_any_element()
}

fn resizable(_: &mut Window, cx: &mut App) -> AnyElement {
    let border = cx.theme().border;
    let cell = |label: &'static str| div().flex().size_full().items_center().justify_center().font_weight(gpui::FontWeight::SEMIBOLD).child(label);
    div()
        .w(px(480.))
        .h(px(240.))
        .rounded_lg()
        .border_1()
        .border_color(border)
        .overflow_hidden()
        .child(
            ResizablePanelGroup::new("resizable")
                .with_handle()
                .panel(ResizablePanel::new(cell("One")).default_size(50.).min_size(20.))
                .panel(ResizablePanel::new(
                    ResizablePanelGroup::new("resizable-nested")
                        .vertical()
                        .panel(ResizablePanel::new(cell("Two")).default_size(25.))
                        .panel(ResizablePanel::new(cell("Three")).default_size(75.)),
                )
                .default_size(50.)),
        )
        .into_any_element()
}

fn scroll_area(_: &mut Window, cx: &mut App) -> AnyElement {
    let border = cx.theme().border;
    ScrollArea::new("scroll")
        .h(px(288.))
        .w(px(192.))
        .rounded_md()
        .border_1()
        .border_color(border)
        .child(
            div()
                .p_4()
                .child(div().mb_4().text_sm().font_weight(gpui::FontWeight::MEDIUM).child("Tags"))
                .children((1..=50).rev().map(|n| {
                    div().child(div().text_sm().child(format!("v1.2.0-beta.{n}"))).child(Separator::horizontal().my_2())
                })),
        )
        .into_any_element()
}

fn select(_: &mut Window, _: &mut App) -> AnyElement {
    Select::new("fruit")
        .w(px(180.))
        .placeholder("Select a fruit")
        .group(
            SelectGroup::new("Fruits")
                .item(SelectItem::new("apple", "Apple"))
                .item(SelectItem::new("banana", "Banana"))
                .item(SelectItem::new("blueberry", "Blueberry"))
                .item(SelectItem::new("grapes", "Grapes"))
                .item(SelectItem::new("pineapple", "Pineapple")),
        )
        .on_change(|value, _, cx| {
            toast(format!("Picked {value}"), cx);
        })
        .into_any_element()
}

fn separator(_: &mut Window, cx: &mut App) -> AnyElement {
    let muted = cx.theme().muted_foreground;
    div()
        .child(
            col()
                .gap_1()
                .child(div().text_sm().font_weight(gpui::FontWeight::MEDIUM).child("Radix Primitives"))
                .child(div().text_sm().text_color(muted).child("An open-source UI component library.")),
        )
        .child(Separator::horizontal().my_4())
        .child(
            row()
                .h_5()
                .gap_4()
                .text_sm()
                .child("Blog")
                .child(Separator::vertical())
                .child("Docs")
                .child(Separator::vertical())
                .child("Source"),
        )
        .into_any_element()
}

fn sheet(_: &mut Window, _: &mut App) -> AnyElement {
    let sheet = |side: Side, label: &'static str| {
        Sheet::new(SharedString::from(format!("sheet-{label}")))
            .side(side)
            .trigger(Button::new(SharedString::from(format!("open-{label}"))).outline().label(label))
            .title("Edit profile")
            .description("Make changes to your profile here. Click save when you're done.")
            .content(|_, _, _| {
                col()
                    .child(Field::new().label("Name").child(Input::new("sheet-name").default_value("Pedro Duarte")))
                    .child(Field::new().label("Username").child(Input::new("sheet-username").default_value("@peduarte")))
            })
            .footer(|sheet, _, _| {
                let close = sheet.clone();
                col()
                    .gap_2()
                    .child(Button::new("save").label("Save changes"))
                    .child(Button::new("close").outline().label("Close").on_click(move |_, window, cx| close.close(window, cx)))
            })
    };
    row()
        .child(sheet(Side::Top, "Top"))
        .child(sheet(Side::Right, "Right"))
        .child(sheet(Side::Bottom, "Bottom"))
        .child(sheet(Side::Left, "Left"))
        .into_any_element()
}

fn sidebar(_: &mut Window, cx: &mut App) -> AnyElement {
    let border = cx.theme().border;
    div()
        .flex()
        .w(px(720.))
        .h(px(420.))
        .rounded_lg()
        .border_1()
        .border_color(border)
        .overflow_hidden()
        .child(
            Sidebar::new("demo-sidebar")
                .header(row().px_2().py_1().font_weight(gpui::FontWeight::SEMIBOLD).child(Icon::new(IconName::Terminal)).child("Acme Inc."))
                .group(
                    SidebarGroup::new()
                        .label("Application")
                        .item(SidebarMenuItem::new("Home").icon(IconName::House).active(true))
                        .item(SidebarMenuItem::new("Inbox").icon(IconName::Inbox).badge("24"))
                        .item(SidebarMenuItem::new("Calendar").icon(IconName::Calendar))
                        .item(SidebarMenuItem::new("Search").icon(IconName::Search))
                        .item(
                            SidebarMenuItem::new("Settings")
                                .icon(IconName::Settings)
                                .sub_item(SidebarMenuItem::new("General"))
                                .sub_item(SidebarMenuItem::new("Team"))
                                .sub_item(SidebarMenuItem::new("Billing")),
                        ),
                )
                .footer(row().px_2().child(Avatar::new().fallback("CN").size_6()).child(div().text_sm().child("shadcn"))),
        )
        .child(
            SidebarInset::new().child(
                row()
                    .h_12()
                    .px_4()
                    .border_b_1()
                    .border_color(border)
                    .child(SidebarTrigger::new("demo-sidebar"))
                    .child(Separator::vertical().h_4())
                    .child(Breadcrumb::new().item(BreadcrumbItem::new("Building Your Application")).item(BreadcrumbItem::new("Data Fetching").current())),
            )
            .child(div().p_4().flex().flex_col().gap_4().child(Skeleton::new().h_24().w_full()).child(Skeleton::new().h_40().w_full())),
        )
        .into_any_element()
}

fn skeleton(_: &mut Window, _: &mut App) -> AnyElement {
    row()
        .gap_4()
        .child(Skeleton::new().size_12().rounded_full())
        .child(col().gap_2().child(Skeleton::new().h_4().w(px(250.))).child(Skeleton::new().h_4().w(px(200.))))
        .into_any_element()
}

fn slider(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .gap_8()
        .w(px(300.))
        .child(Slider::new("slider").default_value([50.]).max(100.).step(1.))
        .child(Slider::new("slider-range").default_value([25., 75.]))
        .child(div().h(px(160.)).child(Slider::new("slider-vertical").vertical().default_value([40.])))
        .into_any_element()
}

fn sonner(_: &mut Window, _: &mut App) -> AnyElement {
    row()
        .child(Button::new("show").outline().label("Show Toast").on_click(|_, _, cx| {
            toast(
                Toast::new("Event has been created")
                    .description("Sunday, December 03, 2023 at 9:00 AM")
                    .action("Undo", |_, _| println!("Undo")),
                cx,
            );
        }))
        .child(Button::new("success").outline().label("Success").on_click(|_, _, cx| {
            toast(Toast::new("Event has been created").success(), cx);
        }))
        .child(Button::new("error").outline().label("Error").on_click(|_, _, cx| {
            toast(Toast::new("Something went wrong").error(), cx);
        }))
        .child(Button::new("loading").outline().label("Loading").on_click(|_, _, cx| {
            let id = toast(Toast::new("Uploading…").loading(), cx);
            cx.spawn(async move |cx| {
                cx.background_executor().timer(std::time::Duration::from_secs(2)).await;
                cx.update(|cx| crate::ui::sonner::update(id, Toast::new("Upload complete").success(), cx)).ok();
            })
            .detach();
        }))
        .into_any_element()
}

fn spinner(_: &mut Window, _: &mut App) -> AnyElement {
    row()
        .gap_6()
        .child(Spinner::new())
        .child(Spinner::new().id("spinner-lg").size_8())
        .child(Button::new("spinner-btn").secondary().size(ButtonSize::Sm).loading(true).label("Loading..."))
        .child(Badge::empty().child(Spinner::new().id("spinner-badge").size_3()).child("Syncing"))
        .into_any_element()
}

fn switch(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .child(Switch::new("airplane").label("Airplane Mode"))
        .child(Switch::new("wifi").default_checked(true).label("Wi-Fi"))
        .child(Switch::new("switch-disabled").disabled(true).label("Disabled"))
        .into_any_element()
}

fn table(_: &mut Window, _: &mut App) -> AnyElement {
    let invoices = [
        ("INV001", "Paid", "Credit Card", "$250.00"),
        ("INV002", "Pending", "PayPal", "$150.00"),
        ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
        ("INV004", "Paid", "Credit Card", "$450.00"),
        ("INV005", "Paid", "PayPal", "$550.00"),
    ];
    Table::new()
        .w(px(600.))
        .caption("A list of your recent invoices.")
        .header(TableHeader::new().row(
            TableRow::new()
                .child(TableHead::new("Invoice").width(px(100.)))
                .child(TableHead::new("Status"))
                .child(TableHead::new("Method"))
                .child(TableHead::new("Amount").align_right()),
        ))
        .body(TableBody::new().rows(invoices.iter().map(|(invoice, status, method, amount)| {
            TableRow::new()
                .child(TableCell::new().width(px(100.)).font_weight(gpui::FontWeight::MEDIUM).child(*invoice))
                .child(TableCell::new().child(*status))
                .child(TableCell::new().child(*method))
                .child(TableCell::new().align_right().child(*amount))
        })))
        .footer(TableFooter::new().row(
            TableRow::new().child(TableCell::new().child("Total")).child(TableCell::new().align_right().child("$2,500.00")),
        ))
        .into_any_element()
}

fn tabs(_: &mut Window, _: &mut App) -> AnyElement {
    let account = Card::new()
        .child(CardHeader::new().child(CardTitle::new("Account")).child(CardDescription::new("Make changes to your account here. Click save when you're done.")))
        .child(CardContent::new().child(col().child(Field::new().label("Name").child(Input::new("tabs-name").default_value("Pedro Duarte")))))
        .child(CardFooter::new().child(Button::new("save-account").label("Save changes")));
    let password = Card::new()
        .child(CardHeader::new().child(CardTitle::new("Password")).child(CardDescription::new("Change your password here.")))
        .child(
            CardContent::new().child(
                col()
                    .child(Field::new().label("Current password").child(Input::new("tabs-current").masked(true)))
                    .child(Field::new().label("New password").child(Input::new("tabs-new").masked(true))),
            ),
        )
        .child(CardFooter::new().child(Button::new("save-password").label("Save password")));
    Tabs::new("tabs")
        .w(px(400.))
        .full_width()
        .default_value("account")
        .tab(Tab::new("account", "Account").content(account))
        .tab(Tab::new("password", "Password").content(password))
        .into_any_element()
}

fn textarea(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .w(px(400.))
        .child(Textarea::new("message").placeholder("Type your message here."))
        .child(Button::new("send").label("Send message"))
        .into_any_element()
}

fn toast_demo(_: &mut Window, _: &mut App) -> AnyElement {
    Button::new("toast")
        .outline()
        .label("Add to calendar")
        .on_click(|_, _, cx| {
            crate::ui::toast::show(
                Toast::new("Scheduled: Catch up").description("Friday, February 10, 2023 at 5:57 PM").action("Undo", |_, _| {}),
                cx,
            );
        })
        .into_any_element()
}

fn toggle(_: &mut Window, _: &mut App) -> AnyElement {
    row()
        .child(Toggle::new("bold").icon(IconName::Bold))
        .child(Toggle::new("italic").outline().icon(IconName::Italic))
        .child(Toggle::new("underline").icon(IconName::Underline).label("Underline"))
        .child(Toggle::new("bookmark").outline().default_pressed(true).icon(IconName::Star).label("Bookmark"))
        .into_any_element()
}

fn toggle_group(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .child(
            ToggleGroup::new("format")
                .multiple()
                .outline()
                .item(ToggleGroupItem::new("bold").icon(IconName::Bold))
                .item(ToggleGroupItem::new("italic").icon(IconName::Italic))
                .item(ToggleGroupItem::new("underline").icon(IconName::Underline)),
        )
        .child(
            ToggleGroup::new("align")
                .default_value(["left"])
                .item(ToggleGroupItem::new("left").icon(IconName::AlignLeft))
                .item(ToggleGroupItem::new("center").icon(IconName::AlignCenter))
                .item(ToggleGroupItem::new("right").icon(IconName::AlignRight)),
        )
        .into_any_element()
}

fn tooltip(_: &mut Window, _: &mut App) -> AnyElement {
    row()
        .gap_4()
        .child(Tooltip::new("tooltip").trigger(Button::new("hover").outline().label("Hover")).content("Add to library"))
        .child(
            Tooltip::new("tooltip-right")
                .side(Side::Right)
                .trigger(Button::new("hover-right").outline().label("Right"))
                .content("Shown on the right"),
        )
        .into_any_element()
}

fn typography(_: &mut Window, _: &mut App) -> AnyElement {
    col()
        .w(px(640.))
        .child(h1("Taxing Laughter: The Joke Tax Chronicles"))
        .child(lead("Once upon a time, in a far-off land, there was a very lazy king who spent all day lounging on his throne."))
        .child(h2("The King's Plan"))
        .child(p("The king thought long and hard, and finally came up with a brilliant plan: he would tax the jokes in the kingdom."))
        .child(blockquote("\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair that they should pay for the privilege.\""))
        .child(h3("The Joke Tax"))
        .child(List::new().item("1st level of puns: 5 gold coins").item("2nd level of jokes: 10 gold coins").item("3rd level of one-liners: 20 gold coins"))
        .child(h4("People stopped telling jokes"))
        .child(row().child(p("Run")).child(inline_code("gpuicn add typography")).child(p("to get these styles.")))
        .child(large("Are you absolutely sure?"))
        .child(small("Email address"))
        .child(muted("Enter your email address."))
        .into_any_element()
}

/// Keeps `ButtonVariant` referenced for docs that show variants by enum.
#[allow(dead_code)]
fn _variants() -> [ButtonVariant; 6] {
    [
        ButtonVariant::Default,
        ButtonVariant::Destructive,
        ButtonVariant::Outline,
        ButtonVariant::Secondary,
        ButtonVariant::Ghost,
        ButtonVariant::Link,
    ]
}

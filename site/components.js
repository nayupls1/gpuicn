// Every gpuicn component: name (as used by `gpuicn add`), a CSS preview that
// mirrors the gpui rendering, and a usage snippet taken from the gallery
// (crates/gallery/src/demos.rs). `cargo test -p gpuicn` checks this list
// against registry/registry.toml.

const ICONS = {
  check: '<path d="M20 6 9 17l-5-5"/>',
  "chevron-down": '<path d="m6 9 6 6 6-6"/>',
  "chevron-right": '<path d="m9 18 6-6-6-6"/>',
  "chevron-left": '<path d="m15 18-6-6 6-6"/>',
  "chevrons-up-down": '<path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>',
  search: '<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>',
  x: '<path d="M18 6 6 18"/><path d="m6 6 12 12"/>',
  copy: '<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>',
  sun: '<circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>',
  moon: '<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>',
  mail: '<rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>',
  "arrow-right": '<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>',
  "arrow-up": '<path d="m5 12 7-7 7 7"/><path d="M12 19V5"/>',
  bold: '<path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"/>',
  italic: '<line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/>',
  underline: '<path d="M6 4v6a6 6 0 0 0 12 0V4"/><line x1="4" x2="20" y1="20" y2="20"/>',
  info: '<circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/>',
  "circle-alert": '<circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>',
  "circle-check": '<circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/>',
  calendar: '<path d="M8 2v4"/><path d="M16 2v4"/><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M3 10h18"/>',
  terminal: '<polyline points="4 17 10 11 4 5"/><line x1="12" x2="20" y1="19" y2="19"/>',
  plus: '<path d="M5 12h14"/><path d="M12 5v14"/>',
  minus: '<path d="M5 12h14"/>',
  user: '<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>',
  settings: '<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>',
  ellipsis: '<circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/>',
  "panel-left": '<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/>',
  house: '<path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>',
  inbox: '<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>',
  folder: '<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>',
  star: '<path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>',
  loader: '<path d="M21 12a9 9 0 1 1-6.219-8.56"/>',
  "log-out": '<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" x2="9" y1="12" y2="12"/>',
  "credit-card": '<rect width="20" height="14" x="2" y="5" rx="2"/><line x1="2" x2="22" y1="10" y2="10"/>',
  "align-left": '<path d="M15 12H3"/><path d="M17 18H3"/><path d="M21 6H3"/>',
  "align-center": '<path d="M17 12H7"/><path d="M19 18H5"/><path d="M21 6H3"/>',
  "align-right": '<path d="M21 12H9"/><path d="M21 18H7"/><path d="M21 6H3"/>',
  palette: '<circle cx="13.5" cy="6.5" r=".5" fill="currentColor"/><circle cx="17.5" cy="10.5" r=".5" fill="currentColor"/><circle cx="8.5" cy="7.5" r=".5" fill="currentColor"/><circle cx="6.5" cy="12.5" r=".5" fill="currentColor"/><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"/>',
  code: '<polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/>',
  package: '<path d="M11 21.73a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73z"/><path d="M12 22V12"/><path d="m3.3 7 7.73 4.47a2 2 0 0 0 1.94 0L20.7 7"/>',
  zap: '<path d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"/>',
  layers: '<path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z"/><path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65"/><path d="m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65"/>',
  "grip-vertical": '<circle cx="9" cy="12" r="1"/><circle cx="9" cy="5" r="1"/><circle cx="9" cy="19" r="1"/><circle cx="15" cy="12" r="1"/><circle cx="15" cy="5" r="1"/><circle cx="15" cy="19" r="1"/>',
};

function I(name, cls = "") {
  return `<svg class="i ${cls}" viewBox="0 0 24 24">${ICONS[name] || ""}</svg>`;
}

const check = (on, label) => `<div class="hstack"><span class="check ${on ? "on" : ""}" data-toggle>${I("check")}</span><span class="label">${label}</span></div>`;
const sw = (on, label) => `<div class="hstack" style="gap:.6rem"><span class="switch ${on ? "on" : ""}" data-toggle></span><span class="label">${label}</span></div>`;
const menuItem = (label, sc = "", cls = "") => `<div class="mi ${cls}">${label}${sc ? `<span class="sc">${sc}</span>` : ""}</div>`;

function calendar(sel = [12], range = false) {
  const dows = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].map((d) => `<div class="dow">${d}</div>`).join("");
  let days = "";
  for (let i = 0; i < 35; i++) {
    const n = i - 2;
    const out = n < 1 || n > 31;
    const label = out ? (n < 1 ? 30 + n : n - 31) : n;
    let cls = out ? "out" : "";
    if (!out && range && n > sel[0] && n < sel[1]) cls = "mid";
    if (!out && sel.includes(n)) cls = "sel";
    if (!out && n === 5 && !sel.includes(n)) cls += " today";
    days += `<div class="d ${cls}">${label}</div>`;
  }
  return `<div class="cal"><div class="cal-head"><button class="btn outline icon sm" style="width:1.75rem;height:1.75rem">${I("chevron-left")}</button><span>October 2026</span><button class="btn outline icon sm" style="width:1.75rem;height:1.75rem">${I("chevron-right")}</button></div><div class="cal-grid">${dows}${days}</div></div>`;
}

function bars() {
  const d = [186, 305, 237, 73, 209, 214], m = [80, 200, 120, 190, 130, 140];
  const g = d.map((v, i) => `<div class="g"><i style="height:${v / 2.6}px;background:var(--chart-1)"></i><i style="height:${m[i] / 2.6}px;background:var(--chart-2)"></i></div>`).join("");
  const x = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"].map((l) => `<span>${l}</span>`).join("");
  return `<div><div class="chart-bars">${g}</div><div class="chart-x" style="margin-top:.35rem">${x}</div></div>`;
}

const CATEGORIES = ["Forms", "Overlays", "Navigation", "Data display", "Feedback", "Layout"];

const COMPONENTS = [
  {
    name: "accordion", title: "Accordion", category: "Layout",
    description: "A vertically stacked set of interactive headings that each reveal a section of content.",
    preview: `<div class="acc" style="width:300px" data-accordion>
      <div class="acc-item on"><div class="acc-trigger">Product Information ${I("chevron-down")}</div><div class="acc-content">Our flagship product combines cutting-edge technology with sleek design.</div></div>
      <div class="acc-item"><div class="acc-trigger">Shipping Details ${I("chevron-down")}</div><div class="acc-content">Worldwide shipping in 3-5 business days.</div></div>
      <div class="acc-item"><div class="acc-trigger">Return Policy ${I("chevron-down")}</div><div class="acc-content">A comprehensive 30-day return policy.</div></div></div>`,
    code: `Accordion::new("accordion")
    .default_value(["item-1"])
    .item(AccordionItem::new("item-1", "Product Information").content(
        div().child("Our flagship product combines cutting-edge technology with sleek design."),
    ))
    .item(AccordionItem::new("item-2", "Shipping Details").content(div().child("…")))
    .item(AccordionItem::new("item-3", "Return Policy").content(div().child("…")))`,
  },
  {
    name: "alert", title: "Alert", category: "Feedback",
    description: "Displays a callout for user attention.",
    preview: `<div class="vstack" style="width:320px">
      <div class="alert">${I("circle-check")}<div class="t">Success! Your changes have been saved</div><div class="d">This is an alert with icon, title and description.</div></div>
      <div class="alert destructive">${I("circle-alert")}<div class="t">Unable to process your payment.</div><div class="d">Please verify your billing information.</div></div></div>`,
    code: `Alert::new()
    .icon(IconName::CircleCheck)
    .title("Success! Your changes have been saved")
    .description("This is an alert with icon, title and description.")

Alert::new().destructive().icon(IconName::CircleAlert).title("Unable to process your payment.")`,
  },
  {
    name: "alert-dialog", title: "Alert Dialog", category: "Overlays",
    description: "A modal dialog that interrupts the user with important content and expects a response.",
    preview: `<div class="sheet-demo" style="width:330px;height:190px"><div class="scrim"></div><div class="panel center">
      <div style="font-weight:600;font-size:.9rem">Are you absolutely sure?</div><div class="muted">This action cannot be undone. This will permanently delete your account.</div>
      <div class="hstack" style="justify-content:flex-end"><button class="btn outline sm">Cancel</button><button class="btn sm">Continue</button></div></div></div>`,
    code: `AlertDialog::new("alert-dialog")
    .trigger(Button::new("open").outline().label("Show Dialog"))
    .title("Are you absolutely sure?")
    .description("This action cannot be undone. This will permanently delete your account.")
    .action("Continue", |_, cx| {
        toast("Account deleted", cx);
    })`,
  },
  {
    name: "aspect-ratio", title: "Aspect Ratio", category: "Layout",
    description: "Displays content within a desired ratio.",
    preview: `<div style="width:280px"><div class="aspect">16 / 9</div></div>`,
    code: `AspectRatio::new(16. / 9.)
    .w(px(450.))
    .rounded_lg()
    .bg(cx.theme().muted)
    .child(div().size_full().flex().items_center().justify_center().child("16 / 9"))`,
  },
  {
    name: "avatar", title: "Avatar", category: "Data display",
    description: "An image element with a fallback for representing the user.",
    preview: `<div class="hstack" style="gap:1rem"><div class="avatar photo">CN</div><div class="avatar" style="border-radius:var(--radius)">ER</div><div class="avatar" style="width:3rem;height:3rem">LR</div>
      <div class="hstack" style="gap:0"><div class="avatar photo" style="outline:2px solid var(--background)">A</div><div class="avatar" style="margin-left:-.5rem;outline:2px solid var(--background)">B</div><div class="avatar photo" style="margin-left:-.5rem;outline:2px solid var(--background);filter:hue-rotate(90deg)">C</div></div></div>`,
    code: `Avatar::new().fallback("CN")
Avatar::new().fallback("ER").rounded_lg()
Avatar::new().fallback("LR").size_12()`,
  },
  {
    name: "badge", title: "Badge", category: "Data display",
    description: "Displays a badge or a component that looks like a badge.",
    preview: `<div class="hstack wrap" style="max-width:260px"><span class="badge">Badge</span><span class="badge secondary">Secondary</span><span class="badge destructive">Destructive</span><span class="badge outline">Outline</span><span class="badge secondary">${I("circle-check")} Verified</span><span class="badge" style="border-radius:99px;padding:0 .4rem">8</span></div>`,
    code: `Badge::new("Badge")
Badge::new("Secondary").secondary()
Badge::new("Destructive").destructive()
Badge::new("Outline").outline()
Badge::new("Verified").secondary().icon(IconName::CircleCheck)
Badge::new("8").rounded_full().px_1()`,
  },
  {
    name: "breadcrumb", title: "Breadcrumb", category: "Navigation",
    description: "Displays the path to the current resource using a hierarchy of links.",
    preview: `<div class="crumbs"><span>Home</span>${I("chevron-right")}${I("ellipsis")}${I("chevron-right")}<span>Components</span>${I("chevron-right")}<span class="cur">Breadcrumb</span></div>`,
    code: `Breadcrumb::new()
    .item(BreadcrumbItem::new("Home").on_click(|_, _, _| {}))
    .ellipsis()
    .item(BreadcrumbItem::new("Components").on_click(|_, _, _| {}))
    .item(BreadcrumbItem::new("Breadcrumb").current())`,
  },
  {
    name: "button", title: "Button", category: "Forms",
    description: "Displays a button or a component that looks like a button.",
    preview: `<div class="vstack" style="align-items:center"><div class="hstack wrap"><button class="btn">Button</button><button class="btn secondary">Secondary</button><button class="btn outline">Outline</button></div>
      <div class="hstack wrap"><button class="btn destructive">Destructive</button><button class="btn ghost">Ghost</button><button class="btn link">Link</button></div>
      <div class="hstack wrap"><button class="btn outline icon">${I("arrow-up")}</button><button class="btn outline">${I("mail")} Login with Email</button><button class="btn sm" disabled>${I("loader", "spin")} Please wait</button></div></div>`,
    code: `Button::new("default").label("Button")
Button::new("secondary").secondary().label("Secondary")
Button::new("destructive").destructive().label("Destructive")
Button::new("outline").outline().label("Outline")
Button::new("ghost").ghost().label("Ghost")
Button::new("link").link().label("Link")

Button::new("icon").outline().size(ButtonSize::Icon).icon(IconName::ArrowUp)
Button::new("with-icon").outline().icon(IconName::Mail).label("Login with Email")
Button::new("loading").size(ButtonSize::Sm).loading(true).label("Please wait")
Button::new("toast").label("Click me").tooltip("Shows a toast").on_click(|_, _, cx| {
    toast("Button clicked", cx);
})`,
  },
  {
    name: "button-group", title: "Button Group", category: "Forms",
    description: "A container that groups related buttons together with consistent styling.",
    preview: `<div class="vstack" style="align-items:center"><div class="bgroup"><button class="btn outline">Archive</button><button class="btn outline">Report</button><button class="btn outline">Snooze</button><button class="btn outline icon">${I("ellipsis")}</button></div>
      <div class="bgroup"><button class="btn secondary icon">${I("minus")}</button><button class="btn secondary icon" style="border-left:1px solid var(--border)">${I("plus")}</button></div></div>`,
    code: `ButtonGroup::new()
    .button(Button::new("archive").outline().label("Archive"))
    .button(Button::new("report").outline().label("Report"))
    .button(Button::new("snooze").outline().label("Snooze"))
    .button(Button::new("more").outline().size(ButtonSize::Icon).icon(IconName::Ellipsis))

ButtonGroup::new()
    .text("https://")
    .element(Input::new("url").placeholder("example.com").rounded_none())
    .button(Button::new("go").outline().label("Go"))`,
  },
  {
    name: "calendar", title: "Calendar", category: "Forms",
    description: "A date field component that allows users to enter and edit date — single, multiple or range selection.",
    preview: calendar([8, 16], true),
    code: `Calendar::new("calendar")

Calendar::new("range-calendar")
    .mode(CalendarMode::Range)
    .months(2)
    .rounded_md()
    .border_1()
    .border_color(cx.theme().border)`,
  },
  {
    name: "card", title: "Card", category: "Layout",
    description: "Displays a card with header, content, and footer.",
    preview: `<div class="card" style="width:340px"><div class="card-head"><div class="hstack" style="justify-content:space-between"><div class="card-title">Login to your account</div><button class="btn link sm">Sign Up</button></div><div class="card-desc">Enter your email below to login</div></div>
      <div class="vstack"><div class="field"><span class="label">Email</span><div class="input"><span class="ph">m@example.com</span></div></div></div>
      <div class="vstack" style="gap:.5rem"><button class="btn w-full">Login</button><button class="btn outline w-full">Login with Google</button></div></div>`,
    code: `Card::new()
    .w(px(380.))
    .child(
        CardHeader::new()
            .child(CardTitle::new("Login to your account"))
            .child(CardDescription::new("Enter your email below to login to your account"))
            .action(Button::new("signup").link().label("Sign Up")),
    )
    .child(CardContent::new().child(
        Field::new().label("Email").child(Input::new("email").placeholder("m@example.com")),
    ))
    .child(
        CardFooter::new()
            .flex_col()
            .child(Button::new("login").label("Login").w_full())
            .child(Button::new("google").outline().label("Login with Google").w_full()),
    )`,
  },
  {
    name: "carousel", title: "Carousel", category: "Data display",
    description: "A carousel of slides with previous/next buttons, several slides per view, vertical mode and optional looping.",
    preview: `<div class="carousel"><button class="btn outline">${I("chevron-left")}</button><div class="slide">1</div><button class="btn outline">${I("chevron-right")}</button></div>`,
    code: `Carousel::new("carousel").w(px(320.)).items((1..=5).map(|n| {
    Card::new().child(
        CardContent::new()
            .flex()
            .h(px(220.))
            .items_center()
            .justify_center()
            .text_3xl()
            .child(n.to_string()),
    )
}))`,
  },
  {
    name: "chart", title: "Chart", category: "Data display",
    description: "Bar, line, area and pie charts drawn with gpui paths and colored by the theme's chart tokens.",
    preview: `<div class="hstack" style="gap:1.5rem">${bars()}<div class="pie"><div><div><b style="font-size:1rem">925</b><br><span class="muted" style="font-size:.6rem">Visitors</span></div></div></div></div>`,
    code: `BarChart::new("bar")
    .labels(["January", "February", "March", "April", "May", "June"])
    .series(ChartSeries::new("Desktop").data([186., 305., 237., 73., 209., 214.]))
    .series(ChartSeries::new("Mobile").data([80., 200., 120., 190., 130., 140.]))

AreaChart::new("area").labels(months).stacked().series(desktop).series(mobile)
LineChart::new("line").labels(months).dots(true).series(desktop)

PieChart::new("pie")
    .slice("Chrome", 275.)
    .slice("Safari", 200.)
    .slice("Firefox", 187.)
    .donut(0.6)
    .center_label("925", "Visitors")`,
  },
  {
    name: "checkbox", title: "Checkbox", category: "Forms",
    description: "A control that allows the user to toggle between checked and not checked.",
    preview: `<div class="vstack">${check(false, "Accept terms and conditions")}${check(true, "Enable notifications")}<div class="hstack" style="opacity:.5">${'<span class="check"></span>'}<span class="label">Disabled</span></div></div>`,
    code: `Checkbox::new("terms").label("Accept terms and conditions")
Checkbox::new("notifications").default_checked(true).label("Enable notifications")
Checkbox::new("all").indeterminate(true).label("Select all")

// Controlled
Checkbox::new("terms").checked(self.accepted).on_change(cx.listener(|this, checked, _, cx| {
    this.accepted = *checked;
    cx.notify();
}))`,
  },
  {
    name: "collapsible", title: "Collapsible", category: "Layout",
    description: "An interactive component which expands/collapses a panel.",
    preview: `<div class="collapsible" style="width:290px"><div class="hstack" style="justify-content:space-between;padding:0 .4rem"><b style="font-size:.8rem">@peduarte starred 3 repositories</b><button class="btn ghost icon sm">${I("chevrons-up-down")}</button></div>
      <div class="repo mono">@radix-ui/primitives</div><div class="repo mono">@radix-ui/colors</div><div class="repo mono">@stitches/react</div></div>`,
    code: `Collapsible::new("collapsible")
    .trigger(
        div().flex().justify_between()
            .child("@peduarte starred 3 repositories")
            .child(Button::new("toggle").ghost().size(ButtonSize::IconSm).icon(IconName::ChevronsUpDown)),
    )
    .child(repo("@radix-ui/primitives"))
    .content(div().child(repo("@radix-ui/colors")).child(repo("@stitches/react")))`,
  },
  {
    name: "combobox", title: "Combobox", category: "Forms",
    description: "Autocomplete input and command palette with a list of suggestions.",
    preview: `<div class="vstack" style="gap:.35rem;width:210px"><div class="select-trigger" style="width:100%">Next.js ${I("chevrons-up-down")}</div>
      <div class="cmdk"><div class="q">${I("search")} Search framework...</div><div class="g"><div class="ci hl">Next.js<span class="sc">${I("check")}</span></div><div class="ci">SvelteKit</div><div class="ci">Nuxt.js</div></div></div></div>`,
    code: `Combobox::new("framework")
    .placeholder("Select framework...")
    .search_placeholder("Search framework...")
    .empty("No framework found.")
    .item("next.js", "Next.js")
    .item("sveltekit", "SvelteKit")
    .item("nuxt.js", "Nuxt.js")
    .item("remix", "Remix")`,
  },
  {
    name: "command", title: "Command", category: "Navigation",
    description: "Fast, composable command menu with fuzzy search and keyboard navigation (shadcn's cmdk).",
    preview: `<div class="cmdk" style="width:290px"><div class="q">${I("search")} Type a command or search...</div>
      <div class="g"><div class="gh">Suggestions</div><div class="ci hl">${I("calendar")} Calendar</div><div class="ci">${I("search")} Search Emoji</div></div>
      <div class="g" style="border-top:1px solid var(--border)"><div class="gh">Settings</div><div class="ci">${I("user")} Profile<span class="sc">⌘P</span></div><div class="ci">${I("credit-card")} Billing<span class="sc">⌘B</span></div></div></div>`,
    code: `Command::new("command")
    .group(
        CommandGroup::new("Suggestions")
            .item(CommandItem::new("Calendar").icon(IconName::Calendar))
            .item(CommandItem::new("Search Emoji").icon(IconName::Search)),
    )
    .group(
        CommandGroup::new("Settings")
            .item(CommandItem::new("Profile").icon(IconName::User).shortcut("⌘P"))
            .item(CommandItem::new("Billing").icon(IconName::CreditCard).shortcut("⌘B")),
    )
    .on_select(|value, _, cx| {
        toast(format!("Selected {value}"), cx);
    })

// Or as a ⌘K palette:
CommandDialog::new("palette").group(…)`,
  },
  {
    name: "context-menu", title: "Context Menu", category: "Overlays",
    description: "Displays a menu to the user — such as a set of actions or functions — triggered by a right click.",
    preview: `<div style="position:relative;width:300px;height:170px;border:1px dashed var(--border);border-radius:var(--radius);display:grid;place-items:center" class="sm-text muted"><span style="position:absolute;left:1rem;top:1rem">Right click here</span>
      <div class="menu" style="position:absolute;left:5.5rem;top:2.2rem;min-width:12rem">${menuItem("Back", "⌘[")}${menuItem("Forward", "⌘]", "dim")}${menuItem("Reload", "⌘R", "hl")}<div class="ms"></div><div class="mi"><span class="ind">${I("check")}</span>Show Bookmarks</div></div></div>`,
    code: `ContextMenu::new("context")
    .trigger(div().size(px(300.)).border_1().border_dashed().child("Right click here"))
    .item(MenuItem::new("Back").shortcut("⌘["))
    .item(MenuItem::new("Forward").shortcut("⌘]").disabled(true))
    .item(MenuItem::new("Reload").shortcut("⌘R"))
    .submenu("More Tools", vec![MenuItem::new("Developer Tools").into()])
    .separator()
    .item(MenuItem::checkbox("Show Bookmarks", true))
    .label("People")
    .item(MenuItem::radio("Pedro Duarte", true))`,
  },
  {
    name: "data-table", title: "Data Table", category: "Data display",
    description: "Powerful tables: sorting, filtering, row selection, row actions and pagination on top of Table.",
    preview: `<div style="width:330px"><div class="dt-head"><div class="input" style="height:2rem;width:60%"><span class="ph">Filter emails...</span></div><button class="btn outline sm">Columns ${I("chevron-down")}</button></div>
      <div class="dt-wrap"><table class="tbl"><thead><tr><th style="width:1.5rem"><span class="check"></span></th><th>Status</th><th>Email</th><th class="r">Amount</th></tr></thead><tbody>
      <tr><td><span class="check on">${I("check")}</span></td><td>success</td><td>ken99@…</td><td class="r">$316.00</td></tr>
      <tr><td><span class="check"></span></td><td>processing</td><td>monserrat@…</td><td class="r">$837.00</td></tr>
      <tr><td><span class="check"></span></td><td>failed</td><td>carmella@…</td><td class="r">$721.00</td></tr></tbody></table></div></div>`,
    code: `DataTable::new("payments", Rc::new(payments))
    .selectable()
    .page_size(5)
    .filter("Filter emails...", |p: &Payment| p.email.to_string())
    .column(DataTableColumn::new("status", "Status", |p: &Payment, _, _| p.status))
    .column(
        DataTableColumn::new("email", "Email", |p: &Payment, _, _| p.email)
            .sortable_by(|p| p.email.to_lowercase()),
    )
    .column(
        DataTableColumn::new("amount", "Amount", |p: &Payment, _, _| format!("{:.2}", p.amount))
            .sortable_by(|p| (p.amount * 100.) as i64)
            .align_right(),
    )
    .row_actions(|p: &Payment| vec![MenuItem::new("Copy payment ID").into()])`,
  },
  {
    name: "date-picker", title: "Date Picker", category: "Forms",
    description: "A date picker with range and presets — a Popover holding a Calendar.",
    preview: `<div class="vstack" style="gap:.35rem;align-items:flex-start"><div class="select-trigger" style="width:13rem;justify-content:flex-start">${I("calendar")} October 12th, 2026</div>${calendar([12])}</div>`,
    code: `DatePicker::new("date")

DatePicker::new("range")
    .mode(CalendarMode::Range)
    .months(2)
    .placeholder("Pick a date range")`,
  },
  {
    name: "dialog", title: "Dialog", category: "Overlays",
    description: "A window overlaid on the primary window, rendering the content underneath inert.",
    preview: `<div class="sheet-demo" style="width:330px;height:200px"><div class="scrim"></div><div class="panel center"><div class="hstack" style="justify-content:space-between"><b>Edit profile</b>${I("x")}</div>
      <div class="field"><span class="label" style="font-size:.75rem">Name</span><div class="input" style="height:1.9rem">Pedro Duarte</div></div><div class="hstack" style="justify-content:flex-end"><button class="btn xs outline">Cancel</button><button class="btn xs">Save changes</button></div></div></div>`,
    code: `Dialog::new("dialog")
    .trigger(Button::new("open").outline().label("Edit Profile"))
    .title("Edit profile")
    .description("Make changes to your profile here. Click save when you're done.")
    .content(|_, _, _| {
        Field::new().label("Name").child(Input::new("name").default_value("Pedro Duarte"))
    })
    .footer(|dialog, _, _| {
        let dialog = dialog.clone();
        Button::new("save").label("Save changes").on_click(move |_, window, cx| {
            dialog.close(window, cx);
            toast(Toast::new("Profile updated").success(), cx);
        })
    })`,
  },
  {
    name: "drawer", title: "Drawer", category: "Overlays",
    description: "A drawer that slides up from the bottom of the window (shadcn's vaul drawer).",
    preview: `<div class="sheet-demo" style="width:300px;height:200px"><div class="scrim"></div><div class="panel bottom"><div class="handle"></div><b>Move Goal</b><div style="font-size:2rem;font-weight:700;line-height:1">350</div><div class="muted xs-text">CALORIES/DAY</div><button class="btn sm w-full">Submit</button></div></div>`,
    code: `Drawer::new("drawer")
    .trigger(Button::new("open").outline().label("Open Drawer"))
    .title("Move Goal")
    .description("Set your daily activity goal.")
    .content(|_, _, _| div().text_size(px(72.)).child("350"))
    .footer(|drawer, _, _| {
        let drawer = drawer.clone();
        Button::new("cancel").outline().label("Cancel").on_click(move |_, window, cx| drawer.close(window, cx))
    })`,
  },
  {
    name: "dropdown-menu", title: "Dropdown Menu", category: "Overlays",
    description: "Displays a menu — such as a set of actions or functions — triggered by a button.",
    preview: `<div class="vstack" style="gap:.35rem;align-items:flex-start"><button class="btn outline sm">Open</button><div class="menu" style="width:13rem"><div class="ml">My Account</div>${menuItem("Profile", "⇧⌘P", "hl")}${menuItem("Billing", "⌘B")}${menuItem("Settings", "⌘S")}<div class="ms"></div><div class="mi">Invite users<span class="sc">${I("chevron-right")}</span></div><div class="ms"></div><div class="mi danger">${I("log-out")} Log out<span class="sc">⇧⌘Q</span></div></div></div>`,
    code: `DropdownMenu::new("dropdown")
    .trigger(Button::new("open").outline().label("Open"))
    .label("My Account")
    .item(MenuItem::new("Profile").shortcut("⇧⌘P"))
    .item(MenuItem::new("Billing").shortcut("⌘B"))
    .item(MenuItem::new("Settings").shortcut("⌘S"))
    .separator()
    .submenu("Invite users", vec![MenuItem::new("Email").into(), MenuItem::new("Message").into()])
    .separator()
    .item(MenuItem::new("Log out").icon(IconName::LogOut).destructive().shortcut("⇧⌘Q"))`,
  },
  {
    name: "empty", title: "Empty", category: "Feedback",
    description: "Use the Empty component to display an empty state.",
    preview: `<div class="empty" style="width:300px"><div class="ico">${I("folder")}</div><b>No Projects Yet</b><div class="muted xs-text">You haven't created any projects yet. Get started by creating your first project.</div><div class="hstack" style="margin-top:.4rem"><button class="btn sm">Create Project</button><button class="btn sm outline">Import Project</button></div></div>`,
    code: `Empty::new()
    .outline()
    .icon(IconName::Folder)
    .title("No Projects Yet")
    .description("You haven't created any projects yet. Get started by creating your first project.")
    .child(Button::new("create").label("Create Project"))`,
  },
  {
    name: "field", title: "Field", category: "Forms",
    description: "Combine labels, controls, and help text to compose accessible form fields.",
    preview: `<div class="vstack" style="width:280px;text-align:left"><b>Payment Method</b><div class="muted xs-text" style="margin-top:-.5rem">All transactions are secure and encrypted</div>
      <div class="field"><span class="label">Card Number</span><div class="input"><span class="ph">1234 5678 9012 3456</span></div><span class="hint">Enter your 16-digit card number</span></div>
      <div class="sep h"></div><div class="hstack" style="justify-content:space-between"><span class="label">Same as shipping address</span><span class="check on">${I("check")}</span></div></div>`,
    code: `FieldSet::new()
    .legend("Payment Method")
    .description("All transactions are secure and encrypted")
    .child(
        FieldGroup::new()
            .child(
                Field::new()
                    .label("Card Number")
                    .description("Enter your 16-digit card number")
                    .child(Input::new("card").placeholder("1234 5678 9012 3456")),
            )
            .child(FieldSeparator::new())
            .child(Field::new().horizontal().label("Same as shipping address").child(Checkbox::new("same"))),
    )`,
  },
  {
    name: "form", title: "Form", category: "Forms",
    description: "Forms with validation: declare fields with validators, get the values on a valid submit.",
    preview: `<div class="vstack" style="width:280px"><div class="field error"><span class="label" style="color:var(--destructive)">Username</span><div class="input invalid">s</div><span class="hint">Username must be at least 2 characters.</span></div>
      <div class="field"><span class="label">Email</span><div class="input"><span class="ph">m@example.com</span></div></div><button class="btn" style="align-self:flex-start">Submit</button></div>`,
    code: `Form::new("profile-form")
    .field(
        FormField::text("username")
            .label("Username")
            .description("This is your public display name.")
            .validate(|value| (value.chars().count() < 2).then(|| "Username must be at least 2 characters.".into())),
    )
    .field(FormField::text("email").label("Email").validate(|v| (!v.contains('@')).then(|| "Invalid email.".into())))
    .field(FormField::textarea("bio").label("Bio"))
    .on_submit(|values, _, cx| {
        toast("Profile saved", cx);
    })`,
  },
  {
    name: "hover-card", title: "Hover Card", category: "Overlays",
    description: "For sighted users to preview content available behind a link.",
    preview: `<div class="vstack" style="gap:.4rem;align-items:center"><button class="btn link">@nextjs</button><div class="pop hcard"><div class="avatar photo">VC</div><div><b class="sm-text">@nextjs</b><div class="sm-text">The React Framework – created and maintained by @vercel.</div><div class="muted xs-text" style="margin-top:.3rem">Joined December 2021</div></div></div></div>`,
    code: `HoverCard::new("hover-card")
    .trigger(Button::new("trigger").link().label("@nextjs"))
    .content(|_, _| {
        div().flex().gap_4()
            .child(Avatar::new().fallback("VC"))
            .child(div().child("@nextjs").child("The React Framework – created and maintained by @vercel."))
    })`,
  },
  {
    name: "input", title: "Input", category: "Forms",
    description: "A text input with selection, clipboard, undo, IME and password masking — built on gpui's input handler.",
    preview: `<div class="vstack" style="width:260px"><input class="input" placeholder="Email"><input class="input" type="password" value="hunter22"><input class="input invalid" placeholder="Invalid"><div class="hstack"><input class="input" placeholder="Email"><button class="btn outline">Subscribe</button></div></div>`,
    code: `Input::new("email").placeholder("Email")
Input::new("password").placeholder("Password").masked(true)
Input::new("invalid").invalid(true)
Input::new("name")
    .default_value("Pedro")
    .on_change(|value, _, _| println!("{value}"))
    .on_submit(|value, _, cx| { toast(format!("Hi {value}"), cx); })`,
  },
  {
    name: "input-group", title: "Input Group", category: "Forms",
    description: "Display additional information or actions next to an input or textarea.",
    preview: `<div class="vstack" style="width:280px"><div class="igroup">${I("search")}<span class="ph">Search...</span><span class="muted xs-text">12 results</span></div><div class="igroup"><span class="muted">https://</span><span class="ph">example.com</span>${I("copy")}</div></div>`,
    code: `InputGroup::new(Input::new("search").placeholder("Search..."))
    .start(InputGroupAddon::icon(IconName::Search))
    .end(InputGroupAddon::text("12 results"))

InputGroup::new(Input::new("url").placeholder("example.com"))
    .start(InputGroupAddon::text("https://"))
    .end(InputGroupAddon::button(Button::new("copy").ghost().size(ButtonSize::IconSm).icon(IconName::Copy)))`,
  },
  {
    name: "input-otp", title: "Input OTP", category: "Forms",
    description: "Accessible one-time password component with copy paste functionality.",
    preview: `<div class="otp"><div class="otp-g"><span>4</span><span>2</span><span class="focus">0</span></div><span class="muted">${I("minus")}</span><div class="otp-g"><span></span><span></span><span></span></div></div>`,
    code: `InputOtp::new("otp")
    .length(6)
    .groups(&[3, 3])
    .on_complete(|code, _, cx| {
        toast(format!("Code entered: {code}"), cx);
    })`,
  },
  {
    name: "item", title: "Item", category: "Data display",
    description: "A versatile component that you can use to display any content.",
    preview: `<div class="vstack" style="width:310px"><div class="item"><div class="grow"><div class="t">Basic Item</div><div class="d">A simple item with title and description.</div></div><button class="btn outline sm">Action</button></div>
      <div class="item">${I("circle-check")}<div class="grow t">Your profile has been verified.</div>${I("chevron-right")}</div></div>`,
    code: `Item::new("basic")
    .outline()
    .title("Basic Item")
    .description("A simple item with title and description.")
    .action(Button::new("action").outline().size(ButtonSize::Sm).label("Action"))

Item::new("user").media(Avatar::new().fallback("S")).title("shadcn").description("shadcn@vercel.com")`,
  },
  {
    name: "kbd", title: "Kbd", category: "Data display",
    description: "Used to display textual user input from keyboard.",
    preview: `<div class="vstack" style="align-items:center"><div class="hstack" style="gap:.25rem"><span class="kbd">⌘</span><span class="kbd">⇧</span><span class="kbd">⌥</span><span class="kbd">⌃</span></div><div class="hstack" style="gap:.25rem"><span class="kbd">Ctrl</span>+<span class="kbd">B</span></div></div>`,
    code: `KbdGroup::new().child(Kbd::new("⌘")).child(Kbd::new("⇧")).child(Kbd::new("⌥"))
KbdGroup::new().child(Kbd::new("Ctrl")).child("+").child(Kbd::new("B"))`,
  },
  {
    name: "label", title: "Label", category: "Forms",
    description: "Renders an accessible label associated with controls.",
    preview: check(true, "Accept terms and conditions"),
    code: `div().flex().gap_2()
    .child(Checkbox::new("terms"))
    .child(Label::new("Accept terms and conditions"))`,
  },
  {
    name: "menubar", title: "Menubar", category: "Navigation",
    description: "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands.",
    preview: `<div class="vstack" style="gap:.3rem;align-items:flex-start"><div class="menubar"><span class="on">File</span><span>Edit</span><span>View</span><span>Profiles</span></div><div class="menu" style="width:13rem">${menuItem("New Tab", "⌘T", "hl")}${menuItem("New Window", "⌘N")}${menuItem("New Incognito Window", "", "dim")}<div class="ms"></div>${menuItem("Print...", "⌘P")}</div></div>`,
    code: `Menubar::new("menubar")
    .menu(
        MenubarMenu::new("File")
            .item(MenuItem::new("New Tab").shortcut("⌘T"))
            .item(MenuItem::new("New Window").shortcut("⌘N"))
            .separator()
            .item(MenuItem::new("Print...").shortcut("⌘P")),
    )
    .menu(MenubarMenu::new("Edit").item(MenuItem::new("Undo").shortcut("⌘Z")))`,
  },
  {
    name: "native-select", title: "Native Select", category: "Forms",
    description: "A styled select with a flat list of options.",
    preview: `<div class="vstack" style="gap:.35rem"><div class="select-trigger">Todo ${I("chevron-down")}</div><div class="menu" style="width:11rem">${menuItem("Todo", "", "hl")}${menuItem("In Progress")}${menuItem("Done")}${menuItem("Cancelled", "", "dim")}</div></div>`,
    code: `NativeSelect::new("status")
    .placeholder("Select status")
    .option("todo", "Todo")
    .option("in-progress", "In Progress")
    .option("done", "Done")
    .disabled_option("cancelled", "Cancelled")`,
  },
  {
    name: "navigation-menu", title: "Navigation Menu", category: "Navigation",
    description: "A collection of links for navigating between app sections.",
    preview: `<div class="vstack" style="gap:.35rem;align-items:flex-start"><div class="nav-demo"><span class="on">Getting started ${I("chevron-down")}</span><span>Components ${I("chevron-down")}</span><span>Docs</span></div>
      <div class="pop nav-panel"><div><b>Introduction</b><small>Re-usable components for gpui.</small></div><div><b>Installation</b><small>How to install and structure your app.</small></div><div><b>Typography</b><small>Styles for headings, paragraphs, lists.</small></div><div><b>Theming</b><small>Restyle everything from one file.</small></div></div></div>`,
    code: `NavigationMenu::new("nav")
    .item(NavigationMenuItem::new("Getting started").content(|_, _| {
        div().grid().grid_cols(2)
            .child(NavigationMenuLink::new("Introduction").description("Re-usable components for gpui."))
            .child(NavigationMenuLink::new("Theming").description("Restyle everything from one file."))
    }))
    .item(NavigationMenuItem::new("Docs").on_click(|_, _, cx| { toast("Docs clicked", cx); }))`,
  },
  {
    name: "pagination", title: "Pagination", category: "Navigation",
    description: "Pagination with page navigation, next and previous links.",
    preview: `<div class="pager"><span class="pg">${I("chevron-left")} Previous</span><span class="pg">1</span><span class="pg on">2</span><span class="pg">3</span><span class="pg">${I("ellipsis")}</span><span class="pg">Next ${I("chevron-right")}</span></div>`,
    code: `Pagination::new("pagination")
    .total(10)
    .page(self.page)
    .on_change(cx.listener(|this, page, _, cx| {
        this.page = *page;
        cx.notify();
    }))`,
  },
  {
    name: "popover", title: "Popover", category: "Overlays",
    description: "Displays rich content in a portal, triggered by a button — click, hover or manual, positioned on any side.",
    preview: `<div class="vstack" style="gap:.4rem;align-items:center"><button class="btn outline sm">Open popover</button><div class="pop" style="width:250px"><b class="sm-text">Dimensions</b><div class="muted xs-text" style="margin-bottom:.6rem">Set the dimensions for the layer.</div>
      <div class="vstack" style="gap:.4rem"><div class="hstack"><span class="label" style="width:5rem;font-size:.75rem">Width</span><div class="input" style="height:1.75rem">100%</div></div><div class="hstack"><span class="label" style="width:5rem;font-size:.75rem">Height</span><div class="input" style="height:1.75rem">25px</div></div></div></div></div>`,
    code: `Popover::new("popover")
    .trigger(Button::new("open").outline().label("Open popover"))
    .side(Side::Bottom)
    .align(Align::Center)
    .content(|_popover, _| {
        div()
            .child("Dimensions")
            .child(Input::new("width").default_value("100%"))
    })

// Hover mode, with delays
Popover::new("info").mode(PopoverTrigger::Hover { open_delay_ms: 300, close_delay_ms: 150 })`,
  },
  {
    name: "progress", title: "Progress", category: "Feedback",
    description: "Displays an indicator showing the completion progress of a task.",
    preview: `<div class="vstack" style="width:260px"><div class="progress"><i style="width:13%"></i></div><div class="progress"><i style="width:66%"></i></div><div class="progress"><i style="width:100%"></i></div></div>`,
    code: `Progress::new(13.)
Progress::new(66.)
Progress::new(100.)`,
  },
  {
    name: "radio-group", title: "Radio Group", category: "Forms",
    description: "A set of checkable buttons where no more than one can be checked at a time.",
    preview: `<div class="vstack" data-radio><div class="hstack"><span class="radio"></span><span class="label">Default</span></div><div class="hstack"><span class="radio on"></span><span class="label">Comfortable</span></div><div class="hstack"><span class="radio"></span><span class="label">Compact</span></div></div>`,
    code: `RadioGroup::new("density")
    .default_value("comfortable")
    .item(RadioGroupItem::new("default", "Default"))
    .item(RadioGroupItem::new("comfortable", "Comfortable"))
    .item(RadioGroupItem::new("compact", "Compact"))`,
  },
  {
    name: "resizable", title: "Resizable", category: "Layout",
    description: "Resizable panel groups and layouts, nestable horizontally and vertically.",
    preview: `<div class="resizable" style="width:300px;height:170px"><div style="flex:1">One</div><div class="rh v"><span class="grip"></span></div><div class="col" style="flex:1"><div style="flex:1">Two</div><div class="rh h"></div><div style="flex:3">Three</div></div></div>`,
    code: `ResizablePanelGroup::new("resizable")
    .with_handle()
    .panel(ResizablePanel::new(cell("One")).default_size(50.).min_size(20.))
    .panel(
        ResizablePanel::new(
            ResizablePanelGroup::new("nested")
                .vertical()
                .panel(ResizablePanel::new(cell("Two")).default_size(25.))
                .panel(ResizablePanel::new(cell("Three")).default_size(75.)),
        )
        .default_size(50.),
    )`,
  },
  {
    name: "scroll-area", title: "Scroll Area", category: "Layout",
    description: "A scroll container with a thin, draggable overlay scrollbar.",
    preview: `<div class="scroll-demo" style="height:180px"><div class="inner"><b>Tags</b>${[50, 49, 48, 47, 46, 45, 44].map((n) => `<div>v1.2.0-beta.${n}</div>`).join("")}</div><div class="bar"><i></i></div></div>`,
    code: `ScrollArea::new("scroll")
    .h(px(288.))
    .w(px(192.))
    .rounded_md()
    .border_1()
    .child(div().p_4().children((1..=50).map(|n| div().child(format!("v1.2.0-beta.{n}")))))`,
  },
  {
    name: "select", title: "Select", category: "Forms",
    description: "Displays a list of options for the user to pick from — triggered by a button.",
    preview: `<div class="vstack" style="gap:.35rem"><div class="select-trigger">Banana ${I("chevron-down")}</div><div class="menu" style="width:11rem"><div class="ml muted" style="font-size:.72rem">Fruits</div>${menuItem("Apple")}<div class="mi hl">Banana<span class="sc">${I("check")}</span></div>${menuItem("Blueberry")}${menuItem("Grapes")}</div></div>`,
    code: `Select::new("fruit")
    .placeholder("Select a fruit")
    .group(
        SelectGroup::new("Fruits")
            .item(SelectItem::new("apple", "Apple"))
            .item(SelectItem::new("banana", "Banana"))
            .item(SelectItem::new("blueberry", "Blueberry")),
    )
    .on_change(|value, _, cx| {
        toast(format!("Picked {value}"), cx);
    })`,
  },
  {
    name: "separator", title: "Separator", category: "Layout",
    description: "Visually or semantically separates content.",
    preview: `<div style="width:260px;text-align:left"><b class="sm-text">Radix Primitives</b><div class="muted sm-text">An open-source UI component library.</div><div class="sep h" style="margin:1rem 0"></div><div class="hstack sm-text" style="gap:1rem;height:1.25rem">Blog<div class="sep v"></div>Docs<div class="sep v"></div>Source</div></div>`,
    code: `Separator::horizontal().my_4()

div().flex().h_5().gap_4()
    .child("Blog")
    .child(Separator::vertical())
    .child("Docs")`,
  },
  {
    name: "sheet", title: "Sheet", category: "Overlays",
    description: "Extends Dialog to display content that complements the main content, sliding in from an edge.",
    preview: `<div class="sheet-demo" style="width:320px;height:200px"><div class="scrim"></div><div class="panel right"><div class="hstack" style="justify-content:space-between"><b>Edit profile</b>${I("x")}</div><div class="muted xs-text">Make changes to your profile here.</div><div class="field"><span class="label" style="font-size:.75rem">Name</span><div class="input" style="height:1.9rem">Pedro Duarte</div></div><button class="btn xs" style="margin-top:auto">Save changes</button></div></div>`,
    code: `Sheet::new("sheet")
    .side(Side::Right)
    .trigger(Button::new("open").outline().label("Open"))
    .title("Edit profile")
    .description("Make changes to your profile here. Click save when you're done.")
    .content(|_, _, _| Field::new().label("Name").child(Input::new("name")))
    .footer(|sheet, _, _| {
        let sheet = sheet.clone();
        Button::new("close").outline().label("Close").on_click(move |_, window, cx| sheet.close(window, cx))
    })`,
  },
  {
    name: "sidebar", title: "Sidebar", category: "Navigation",
    description: "A composable, themeable and customizable sidebar that collapses to icons or off-canvas.",
    preview: `<div class="sidebar-demo" style="width:330px;height:200px"><div class="sb"><div class="hstack" style="padding:.3rem .5rem;font-weight:600">${I("terminal")} Acme Inc.</div><div class="grp">Application</div><div class="it on">${I("house")} Home</div><div class="it">${I("inbox")} Inbox<span class="badge secondary" style="margin-left:auto;padding:0 .35rem">24</span></div><div class="it">${I("calendar")} Calendar</div><div class="it">${I("settings")} Settings</div></div>
      <div class="main"><div class="bar">${I("panel-left")}<div class="sep v" style="margin:.6rem 0"></div><span class="muted">Data Fetching</span></div><div class="vstack" style="padding:.6rem;gap:.5rem"><div class="skel" style="height:3rem"></div><div class="skel" style="height:4.5rem"></div></div></div></div>`,
    code: `Sidebar::new("app-sidebar")
    .header(div().child("Acme Inc."))
    .group(
        SidebarGroup::new()
            .label("Application")
            .item(SidebarMenuItem::new("Home").icon(IconName::House).active(true))
            .item(SidebarMenuItem::new("Inbox").icon(IconName::Inbox).badge("24"))
            .item(
                SidebarMenuItem::new("Settings")
                    .icon(IconName::Settings)
                    .sub_item(SidebarMenuItem::new("General"))
                    .sub_item(SidebarMenuItem::new("Team")),
            ),
    )

SidebarInset::new().child(SidebarTrigger::new("app-sidebar"))`,
  },
  {
    name: "skeleton", title: "Skeleton", category: "Feedback",
    description: "Use to show a placeholder while content is loading.",
    preview: `<div class="hstack" style="gap:1rem"><div class="skel" style="width:3rem;height:3rem;border-radius:99px"></div><div class="vstack" style="gap:.5rem"><div class="skel" style="height:1rem;width:200px"></div><div class="skel" style="height:1rem;width:160px"></div></div></div>`,
    code: `Skeleton::new().size_12().rounded_full()
Skeleton::new().h_4().w(px(250.))
Skeleton::new().h_4().w(px(200.))`,
  },
  {
    name: "slider", title: "Slider", category: "Forms",
    description: "An input where the user selects a value — or a range — from within a given range.",
    preview: `<div class="vstack" style="width:260px;gap:2rem"><div class="slider"><div class="track"><div class="range" style="width:50%"></div><div class="thumb" style="left:50%"></div></div></div><div class="slider"><div class="track"><div class="range" style="left:25%;width:50%"></div><div class="thumb" style="left:25%"></div><div class="thumb" style="left:75%"></div></div></div></div>`,
    code: `Slider::new("volume").default_value([50.]).max(100.).step(1.)
Slider::new("range").default_value([25., 75.])
Slider::new("vertical").vertical().default_value([40.])`,
  },
  {
    name: "sonner", title: "Sonner", category: "Feedback",
    description: "An opinionated toast component: stacked, with variants, actions, promises and updates.",
    preview: `<div class="toast-stack"><div class="toast back" style="transform:translateY(-2.4rem) scale(.9);opacity:.6"><div class="t">Upload complete</div></div><div class="toast back" style="transform:translateY(-1.7rem) scale(.95);opacity:.85"><div class="t">Saved</div></div><div class="toast"><div style="flex:1"><div class="t">Event has been created</div><div class="d">Sunday, December 03, 2023 at 9:00 AM</div></div><button class="btn xs">Undo</button></div></div>`,
    code: `// Once, in your root view:
.child(Toaster::new())

// Anywhere:
toast("Event has been created", cx);
toast(Toast::new("Something went wrong").error(), cx);
toast(
    Toast::new("Event has been created")
        .description("Sunday, December 03, 2023 at 9:00 AM")
        .action("Undo", |_, _| println!("Undo")),
    cx,
);

let id = toast(Toast::new("Uploading…").loading(), cx);
sonner::update(id, Toast::new("Upload complete").success(), cx);`,
  },
  {
    name: "spinner", title: "Spinner", category: "Feedback",
    description: "An indicator that can be used to show a loading state.",
    preview: `<div class="spinner-row">${I("loader", "spin")}<svg class="i spin" viewBox="0 0 24 24" style="width:2rem;height:2rem">${ICONS.loader}</svg><button class="btn secondary sm" disabled>${I("loader", "spin")} Loading...</button><span class="badge secondary">${I("loader", "spin")} Syncing</span></div>`,
    code: `Spinner::new()
Spinner::new().id("large").size_8()
Button::new("saving").secondary().loading(true).label("Loading...")`,
  },
  {
    name: "switch", title: "Switch", category: "Forms",
    description: "A control that allows the user to toggle between checked and not checked.",
    preview: `<div class="vstack">${sw(false, "Airplane Mode")}${sw(true, "Wi-Fi")}</div>`,
    code: `Switch::new("airplane").label("Airplane Mode")
Switch::new("wifi").default_checked(true).label("Wi-Fi")

// Controlled
Switch::new("dark").checked(self.dark).on_change(cx.listener(|this, on, _, cx| {
    this.dark = *on;
    cx.notify();
}))`,
  },
  {
    name: "table", title: "Table", category: "Data display",
    description: "A responsive table component with header, body, footer and caption.",
    preview: `<table class="tbl" style="width:320px"><thead><tr><th>Invoice</th><th>Status</th><th>Method</th><th class="r">Amount</th></tr></thead><tbody><tr><td><b>INV001</b></td><td>Paid</td><td>Credit Card</td><td class="r">$250.00</td></tr><tr><td><b>INV002</b></td><td>Pending</td><td>PayPal</td><td class="r">$150.00</td></tr><tr><td><b>INV003</b></td><td>Unpaid</td><td>Transfer</td><td class="r">$350.00</td></tr></tbody><tfoot><tr><td colspan="3">Total</td><td class="r">$750.00</td></tr></tfoot></table>`,
    code: `Table::new()
    .caption("A list of your recent invoices.")
    .header(TableHeader::new().row(
        TableRow::new()
            .child(TableHead::new("Invoice"))
            .child(TableHead::new("Status"))
            .child(TableHead::new("Amount").align_right()),
    ))
    .body(TableBody::new().rows(invoices.iter().map(|(invoice, status, amount)| {
        TableRow::new()
            .child(TableCell::new().child(*invoice))
            .child(TableCell::new().child(*status))
            .child(TableCell::new().align_right().child(*amount))
    })))`,
  },
  {
    name: "tabs", title: "Tabs", category: "Navigation",
    description: "A set of layered sections of content — known as tab panels — that are displayed one at a time.",
    preview: `<div class="vstack" style="width:290px" data-tabs><div class="tabs-list"><span class="tab on">Account</span><span class="tab">Password</span></div><div class="card" style="gap:.8rem;padding:1rem 0"><div class="card-head"><div class="card-title">Account</div><div class="card-desc">Make changes to your account here.</div></div><div class="field"><span class="label">Name</span><div class="input">Pedro Duarte</div></div></div></div>`,
    code: `Tabs::new("tabs")
    .default_value("account")
    .tab(Tab::new("account", "Account").content(account_card))
    .tab(Tab::new("password", "Password").content(password_card))`,
  },
  {
    name: "textarea", title: "Textarea", category: "Forms",
    description: "Displays a form textarea or a component that looks like a textarea.",
    preview: `<div class="vstack" style="width:280px"><textarea class="input" placeholder="Type your message here."></textarea><button class="btn">Send message</button></div>`,
    code: `Textarea::new("message").placeholder("Type your message here.")`,
  },
  {
    name: "toast", title: "Toast", category: "Feedback",
    description: "A succinct message that is displayed temporarily (the legacy API, backed by Sonner).",
    preview: `<div class="toast"><div style="flex:1"><div class="t">Scheduled: Catch up</div><div class="d">Friday, February 10, 2023 at 5:57 PM</div></div><button class="btn outline xs">Undo</button></div>`,
    code: `toast::show(
    Toast::new("Scheduled: Catch up")
        .description("Friday, February 10, 2023 at 5:57 PM")
        .action("Undo", |_, _| {}),
    cx,
);`,
  },
  {
    name: "toggle", title: "Toggle", category: "Forms",
    description: "A two-state button that can be either on or off.",
    preview: `<div class="hstack"><span class="toggle on" data-toggle>${I("bold")}</span><span class="toggle outline" data-toggle>${I("italic")}</span><span class="toggle" data-toggle>${I("underline")} Underline</span><span class="toggle outline on" data-toggle>${I("star")} Bookmark</span></div>`,
    code: `Toggle::new("bold").icon(IconName::Bold)
Toggle::new("italic").outline().icon(IconName::Italic)
Toggle::new("bookmark").outline().default_pressed(true).icon(IconName::Star).label("Bookmark")`,
  },
  {
    name: "toggle-group", title: "Toggle Group", category: "Forms",
    description: "A set of two-state buttons that can be toggled on or off — single or multiple.",
    preview: `<div class="vstack" style="align-items:center"><div class="tgroup outline"><span class="toggle on" data-toggle>${I("bold")}</span><span class="toggle" data-toggle>${I("italic")}</span><span class="toggle on" data-toggle>${I("underline")}</span></div><div class="tgroup"><span class="toggle on">${I("align-left")}</span><span class="toggle">${I("align-center")}</span><span class="toggle">${I("align-right")}</span></div></div>`,
    code: `ToggleGroup::new("format")
    .multiple()
    .outline()
    .item(ToggleGroupItem::new("bold").icon(IconName::Bold))
    .item(ToggleGroupItem::new("italic").icon(IconName::Italic))
    .item(ToggleGroupItem::new("underline").icon(IconName::Underline))

ToggleGroup::new("align").default_value(["left"]).item(…)`,
  },
  {
    name: "tooltip", title: "Tooltip", category: "Overlays",
    description: "A popup that displays information related to an element when the element receives hover.",
    preview: `<div class="vstack" style="gap:.4rem;align-items:center"><span class="tip">Add to library</span><button class="btn outline">Hover</button></div>`,
    code: `Tooltip::new("tooltip")
    .trigger(Button::new("hover").outline().label("Hover"))
    .content("Add to library")

// Or on any element, via gpui's tooltip API:
div().id("x").tooltip(tooltip("Add to library"))`,
  },
  {
    name: "typography", title: "Typography", category: "Data display",
    description: "Styles for headings, paragraphs, lists, quotes and inline code — shadcn's typography recipes.",
    preview: `<div class="typo" style="width:300px;text-align:left"><h4>The Joke Tax Chronicles</h4><p class="muted">Once upon a time, in a far-off land, there was a very lazy king.</p><blockquote>"After all," he said, "everyone enjoys a good joke."</blockquote><p style="margin-top:.6rem">Run <code>gpuicn add typography</code></p></div>`,
    code: `h1("Taxing Laughter: The Joke Tax Chronicles")
lead("Once upon a time, in a far-off land, there was a very lazy king…")
h2("The King's Plan")
p("The king thought long and hard…")
blockquote("\\"After all,\\" he said, \\"everyone enjoys a good joke.\\"")
List::new().item("1st level of puns: 5 gold coins")
inline_code("gpuicn add typography")
muted("Enter your email address.")`,
  },
];

// Building blocks that every component pulls in; listed separately on the site.
const FOUNDATIONS = [
  { name: "theme", title: "theme", description: "The single file that styles everything: colors (oklch, light and dark), radius, fonts and shadows." },
  { name: "utils", title: "utils", description: "Style refinement (shadcn's cn()), focus rings and shared enums." },
  { name: "icon", title: "icon", description: "~55 Lucide icons embedded as source, served through IconAssets." },
  { name: "portal", title: "portal", description: "Draws popovers, menus, dialogs and toasts on top of the window — like a React portal." },
];

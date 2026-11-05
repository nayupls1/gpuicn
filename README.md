# gpuicn

[shadcn/ui](https://ui.shadcn.com) for [gpui](https://gpui.rs). It isn't a
component library you depend on: the `gpuicn` CLI **copies the source of each
component into your crate**, where it belongs to you. You can read it, edit it
or delete it, and one file (`theme.rs`) restyles all of it.

```
crates/gpuicn        the CLI
crates/gallery       a gpui app that renders every component (`cargo run -p gallery`)
registry/            the component sources (`ui/*.rs`) and their manifest (`registry.toml`)
site/                a static website with an overview of every component

crates/gpuicn-rsx    the same CLI for gpuicn-rsx
crates/gallery-rsx   the gallery, written in rsx (`cargo run -p gallery-rsx`)
registry-rsx/        the same components written in rsx (`ui/*.rsx`)
```

Every component also exists as **gpuicn-rsx**, written in
[rsx](https://github.com/nayupls1/rsx) (JSX for Rust, SolidJS-style signals,
rendered with GPUI). It's a separate package next to the gpui one, with the
same components, options and look; see [gpuicn-rsx](#gpuicn-rsx).

## Quick start

```sh
cargo install --path crates/gpuicn     # from a checkout of this repo

cd my-app                              # any Cargo package
gpuicn init                            # gpuicn.toml + theme.rs + utils.rs, wires up `mod components;`
gpuicn add button dialog select        # copies them (and their dependencies) into src/components/ui
```

Then set up your app once:

```rust
mod components;
use components::ui::{self, button::Button, portal::PortalHost, sonner::{Toaster, toast}};
use gpui::{prelude::*, *};

struct Root;

impl Render for Root {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(Button::new("save").label("Save").on_click(|_, _, cx| { toast("Saved", cx); }))
            .child(Toaster::new())    // if you added `sonner`
            .child(PortalHost::new()) // last child: popovers, menus and dialogs draw here
    }
}

fn main() {
    Application::new().with_assets(ui::icon::IconAssets).run(|cx| {
        ui::init(cx); // theme + input key bindings
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Root)).unwrap();
    });
}
```

(`cx.theme()` comes from `ui::theme::ActiveTheme`.)

## Using components

Components are `RenderOnce` builders that implement `Styled`. Any style you set
is layered over the defaults, the way `className` / `cn()` works in shadcn:

```rust
Button::new("go").outline().size(ButtonSize::Sm).icon(IconName::ArrowRight).label("Continue")
Button::new("wide").label("Full width").w_full().rounded_full()   // override anything

Card::new()
    .w(px(350.))
    .child(CardHeader::new().child(CardTitle::new("Login")).child(CardDescription::new("Welcome back")))
    .child(CardContent::new().child(Input::new("email").placeholder("Email")))
```

Stateful components are **uncontrolled** by default: their state is keyed by
the id you pass. To **control** them, pass the value and listen for changes:

```rust
Switch::new("airplane").checked(self.airplane).on_change(cx.listener(|this, checked, _, cx| {
    this.airplane = *checked;
    cx.notify();
}))

Dialog::new("edit").open(self.editing).on_open_change(cx.listener(|this, open, _, _| this.editing = *open))
```

`Input` and `Popover` also have an `unstyled()` escape hatch for building your
own look on top of their behavior. And since you own the source, you can
always just change the file.

Floating content (popovers, menus, selects) behaves like Radix's: it flips to
the other side of its trigger when it doesn't fit, and when it fits on neither
side it takes the roomier one, caps its height there and scrolls. It never
slides over its trigger. Unstyled popover content gets that room from
`PopoverHandle::available_height(cx)`.

## Theming

Everything reads its colors, radius, fonts and shadows from
`src/components/ui/theme.rs`. Colors use `oklch`, the same values as shadcn's
CSS variables, so you can paste themes from
[ui.shadcn.com/themes](https://ui.shadcn.com/themes) or [tweakcn](https://tweakcn.com):

```css
--primary: oklch(0.205 0 0);           /* shadcn CSS */
--border: oklch(1 0 0 / 10%);
```
```rust
primary: oklch(0.205, 0.0, 0.0),       // theme.rs
border: oklcha(1.0, 0.0, 0.0, 0.1),
```

shadcn's base colors and color themes are built in, and survive switching
between light and dark:

```rust
Theme::change(ThemeMode::Dark, cx);          // switch the color scheme
Theme::sync_system_appearance(window, cx);   // follow the OS
Theme::set_base(BaseColor::Zinc, cx);        // neutral, zinc, slate, stone, gray
Theme::set_accent(AccentColor::Blue, cx);    // blue, green, rose, orange, violet, red, yellow
Theme::set_radius(px(6.), cx);               // --radius; sm/md/lg/xl derive from it
Theme::global_mut(cx).primary = oklch(0.6, 0.2, 30.0); // or tweak any single token
```

The gallery's **Customize** button drives exactly these setters.

### Fonts and line height

`theme.font_family` defaults to the platform UI font. shadcn uses
[Geist](https://vercel.com/font); the gallery bundles it (see `load_fonts` in
`crates/gallery/src/main.rs`) and you can do the same with
`cx.text_system().add_fonts(..)`. Components set Tailwind's line heights for
every text size; for loose text in your own views, put
`.line_height(relative(1.43))` on your root (gpui's default is 1.618).

### Helpers for your own components

`utils.rs` has the pieces the components are built from:
`control_surface(theme)` (input-like background and shadow),
`focus_ring` / `invalid_ring` / `ring` (rings drawn outside an element),
`use_tab_stop` + `is_focus_visible` (keyboard focus), and `refine_style`
(layers a caller's `Styled` overrides over your defaults, merging text styles
field by field). `Theme::faded` fades a color the way CSS `opacity` fades a
whole element.

## CLI

| Command | |
|---|---|
| `gpuicn init [--path src/components/ui]` | Create `gpuicn.toml`, install `theme` + `utils`, add `gpui` and wire the module tree |
| `gpuicn add <names…>` / `--all` | Copy components and their dependencies. Existing files are skipped unless you pass `--overwrite`. `--dry-run` previews the changes |
| `gpuicn list` | Show every component and whether it's installed |
| `gpuicn view <name>` | Print a component's source |
| `gpuicn diff [name]` | Show how your copies differ from the registry |
| `gpuicn remove <names…>` | Delete components, refusing if others still depend on them (unless you pass `--force`) |

Names are forgiving: `alert-dialog`, `alert_dialog` and `AlertDialog` all work.
`-C <dir>` runs the CLI in another directory.

`gpuicn add` regenerates the region of `mod.rs` between `// gpuicn:begin` and
`// gpuicn:end`, which holds the module list and `ui::init`. Anything you write
outside that region is kept.

### Custom registries

A registry is a directory containing `registry.toml` and `ui/*.rs` (see
`registry/`). To use one, pass `--registry <dir>` on the command line, or put
`registry = "path/to/registry"` in `gpuicn.toml`. That lets you keep your
company's design system in a repo and install from it just like from the
built-in registry.

## Components

Every component from ui.shadcn.com:

Accordion, Alert, Alert Dialog, Aspect Ratio, Avatar, Badge, Breadcrumb, Button,
Button Group, Calendar, Card, Carousel, Chart (bar, line, area, pie), Checkbox,
Collapsible, Combobox, Command, Context Menu, Data Table, Date Picker, Dialog,
Drawer, Dropdown Menu, Empty, Field, Form, Hover Card, Input, Input Group,
Input OTP, Item, Kbd, Label, Menubar, Native Select, Navigation Menu,
Pagination, Popover, Progress, Radio Group, Resizable, Scroll Area, Select,
Separator, Sheet, Sidebar, Skeleton, Slider, Sonner, Spinner, Switch, Table,
Tabs, Textarea, Toast, Toggle, Toggle Group, Tooltip, Typography.

They're built on these infrastructure modules: `theme`, `utils`, `icon` (lucide icons
embedded as source), `portal` (overlay layer) and `popover` (floating
positioning).

Run `cargo run -p gallery` to see all of them, or open `site/index.html`.
`GALLERY_PAGE=select GALLERY_THEME=dark GALLERY_ACCENT=blue GALLERY_BASE=zinc`
opens the gallery in a given state (handy for screenshots).

## Troubleshooting

**`cannot find value ENOATTR in crate libc`** (Linux, gpui 0.2.2): gpui pulls in
`xattr 0.2.3`, which doesn't build against current `libc`. Copy `patches/xattr`
from this repo into your project and add:

```toml
[patch.crates-io]
xattr = { path = "patches/xattr" }
```

**Popovers/menus don't show up**: render `PortalHost::new()` as the last child of
your root view.

**Icons are blank**: register the assets with
`Application::new().with_assets(ui::icon::IconAssets)`. If you already have an
`AssetSource`, chain them with `IconAssets.with_fallback(yours)`.

## Keyboard

Tab / Shift-Tab move focus between buttons, inputs and select triggers (with a
focus-visible ring, like the web). Enter / Space activate buttons; Enter, Space
or ↓ open menus, selects and popovers from their trigger; arrows move through
menus, lists and the command palette; Escape closes and returns focus to the
trigger.

## Notes on gpui

A few gpui behaviors that the components work around, useful if you write
your own:

- **Box shadows are painted under the element, not just outside it**, so a
  shadow on a transparent element tints its interior, and a zero-blur shadow
  isn't drawn at all. Give it a background, and draw rings with `utils::ring`.
- **`overflow_hidden` clips to a rectangle**, not to rounded corners: round the
  children too (see `Progress`, `Avatar`).
- **`cx.notify()` during prepaint/paint is dropped**: if a `canvas` measurement
  changes what you render, call `window.request_animation_frame()`.
- **`.hover(..)` can't change text**: hover styles are applied after text is
  laid out, so a hover text color or underline never shows (backgrounds do).
  Use `utils::use_hover` and style from its state (see `Button`'s link variant).
- **Element opacity fades each primitive separately**: text on a 50%-opaque
  fill looks washed out twice. Fade colors with `Theme::faded` instead.
- **Keyed state is namespaced by the surrounding element ids**: if a parent
  needs an `Input`'s state, resolve it with `Input::state(&mut self, ..)`,
  which pins it for the input's own render.

## gpuicn-rsx

The same components written as `.rsx` files for [rsx](https://github.com/nayupls1/rsx)
(crates.io: `rsxui*`). Components run once, state lives in signals, and
markup that reads a signal updates by itself. You install them the same way,
with their own CLI:

```sh
cargo install --path crates/gpuicn-rsx

cd my-app
gpuicn-rsx init                  # gpuicn-rsx.toml, theme.rsx + utils.rsx, rsxui crates, build.rs, `mod components;`
gpuicn-rsx add button dialog select
```

`init` adds `rsxui`, `rsxui-gpui` and the `rsxui-build` build dependency (as
`rsx`, `rsx-gpui`, `rsx-build`), and makes `build.rs` call
`rsx_build::compile()`. The generated `components/ui/mod.rs` declares each
file with `rsx::module!`. `list`, `view`, `diff`, `remove` and custom
registries work as in `gpuicn`.

```rust
// src/app.rsx
use rsx::prelude::*;
use crate::components::ui::{self, button::{Button, ButtonSize}, icon::IconName, switch::Switch, sonner::{toast, Toaster}, theme::theme};

#[component]
pub fn App() -> impl IntoElement {
    ui::init();
    let t = theme();
    let airplane = signal(false);

    <div class="size-full flex flex-col gap-4 p-8" bg={t.get().background} text_color={t.get().foreground}>
        <Button label="Save" onClick={move |_| { toast("Saved"); }} />
        <Button outline size={ButtonSize::Sm} icon={IconName::ArrowRight} label="Continue" class="w-full" />
        <Switch label="Airplane mode" checked={airplane} onChange={move |on: &bool| airplane.set(*on)} />
        <Toaster />
    </div>
}
```

```rust
// src/main.rs
mod components;
rsx::module!(app);

fn main() {
    rsx_gpui::run(rsx::WindowOptions::new("My app"), app::App::new);
}
```

How the gpui API maps to rsx:

- **Builders are props**: `Button::new("x").outline().label("Go")` is
  `<Button outline label="Go" />`. Item lists are child components
  (`<Tabs><Tab .../></Tabs>`) or `items={vec![..]}` props, as each file's doc
  comment shows.
- **`Styled` overrides** are `class`/style attributes on the component, layered
  over its defaults like `cn()`: `<Card class="w-[350px]">`.
- **Uncontrolled by default**: state is a signal inside the component. To control
  it, pass a value (or signal) and a change callback:
  `checked={on} onChange={move |v: &bool| on.set(*v)}`.
- **Theme**: `theme()` is a global signal (`t.get().primary`); `Theme::change(ThemeMode::Dark)`,
  `Theme::set_base(..)`, `Theme::set_accent(..)`, `Theme::set_radius(6.0)` and
  `Theme::update(|t| ..)` replace the `cx` setters.
- **No `PortalHost` needed**: rsx draws `<Portal>` content above the window
  (`PortalHost` still exists and renders nothing). `toast(..)` needs no `cx`.

`cargo run -p gallery-rsx` shows every component (same `GALLERY_PAGE`,
`GALLERY_THEME`, `GALLERY_ACCENT`, `GALLERY_BASE` variables), and
`cargo run -p gallery-rsx -- --html` prints the current page as HTML.
`cargo test -p gallery-rsx` renders every page in light and dark mode and
drives the interactive components on GPUI's test platform.
`registry-rsx/PORTING.md` has the conventions the port follows.

The port drove a round of rsx fixes ([rsx#7–#36](https://github.com/nayupls1/rsx/issues?q=is%3Aissue));
gpuicn-rsx needs rsx 0.0.3 or newer.

## Releases

`.github/workflows/release.yml` builds both galleries and both CLIs on every
push to `master`: an AppImage per gallery and a CLI tarball for Linux, and a
zip for Windows, uploaded as workflow artifacts. Pushing a `v*` tag also
attaches them to a GitHub release.

## License

MIT. Icons are from [Lucide](https://lucide.dev) (ISC).

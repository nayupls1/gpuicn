# gpuicn

[shadcn/ui](https://ui.shadcn.com) for [gpui](https://gpui.rs). It isn't a
component library you depend on: the `gpuicn` CLI **copies the source of each
component into your crate**, where it belongs to you. You can read it, edit it
or delete it, and one file (`theme.rs`) restyles all of it.

```
crates/gpuicn    the CLI
crates/gallery   a gpui app that renders every component (`cargo run -p gallery`)
registry/        the component sources (`ui/*.rs`) and their manifest (`registry.toml`)
site/            a static website with an overview of every component
```

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

At runtime:

```rust
Theme::change(ThemeMode::Dark, cx);          // switch the color scheme
Theme::sync_system_appearance(window, cx);   // follow the OS
Theme::global_mut(cx).radius = px(6.);       // tweak any token
```

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

## Known limitations

- Buttons can't be focused with the keyboard yet. Menus, selects, the command palette and inputs can.
- Menubar doesn't switch between open menus on hover. You have to click each one.

## License

MIT. Icons are from [Lucide](https://lucide.dev) (ISC).

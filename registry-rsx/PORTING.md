# Porting gpuicn components to rsx

`registry-rsx/ui/*.rsx` is a 1:1 port of `registry/ui/*.rs`: same components,
same props/options, same look and behavior, written as rsx components. These
notes are the conventions the port follows.

## Shape of a component

```rust
use rsx::prelude::*;
use super::theme::{theme, ColorExt};

#[component]
pub fn Badge(
    #[prop(optional)] variant: BadgeVariant,   // builder `.variant(..)`
    secondary: bool,                           // builder shorthand `.secondary()` -> `<Badge secondary>`
    label: Option<Accessor<String>>,           // text that may change
    disabled: Option<Accessor<bool>>,          // any flag that may change
    on_click: Option<Callback<ClickEvent>>,    // `onClick={move |e| ..}`
    children: Option<Children>,                // gpui `ParentElement` -> children
    attrs: Attributes,                         // gpui `Styled` overrides -> spread last on the root
) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();                   // re-renders this closure when the theme changes
        <div class="inline-flex items-center gap-1 px-2 py-0.5 text-xs font-medium border"
             corner_radius={theme.radius_md()} bg={theme.primary} text_color={theme.primary_foreground}
             {...&attrs}>
            {label.map(|l| l.get())}
            {children}
        </div>
    }
}
```

* Builders become props; each builder method maps to a prop of the same name.
  Boolean shorthands (`.outline()`) become `bool` props (`<Button outline>`).
* `Styled` (caller style overrides) becomes `attrs: Attributes`, spread **last**
  on the root element so the caller's classes win (`{...&attrs}` inside a `move ||`).
* Stateful components: uncontrolled by default (state is a `signal` created in
  the component body), controlled with `checked: Option<Accessor<T>>` plus
  `on_change: Option<Callback<T>>`, like the gpui version's `.checked(..)` / `.on_change(..)`.
  Create signals in the component body, **outside** any `move ||` closure,
  or they're recreated (and reset) every time the closure re-runs.
* `cx.theme()` becomes `let t = theme();` then `t.get()` inside a reactive
  closure or attribute (`bg={t.get().primary}` is reactive by itself).
* gpui colors (`Hsla`) are `rsx::Color`; `.opacity(x)` comes from
  `theme::ColorExt`. Radii: `theme.radius` is `f32` px; `radius_md()` etc. return `Length`.
* `window.use_keyed_state` goes away: components run once, so a local `signal` is the state.
* Focus: `let node = NodeRef::new();` + `ref={node}`; `node.focus()`,
  `node.is_focused()`; `utils::is_focus_visible(node)` for focus rings (use
  `utils::ring`/`focus_ring` as the last child of a `relative` root).
  Put `onKeyDown={on_tab_key}` (or call it from your key handler) on focusable
  roots so Tab switches to keyboard-focus mode.
* Hover text changes: `let hovered = signal(false);` + `onHover={move |h: &bool| hovered.set(*h)}`.
* Overlays: use `popover::Popover` (click/hover/manual modes, `PopoverHandle`)
  or `<Portal>` directly (full-window layers: dialogs, sheets, toasts).
  `portal::FullWindow` is a full-window portal layer.
* Animations: rsx's `transition`/`duration-*`/`animate-*` classes, or
  `utils::use_animation_clock()` / `use_transition(ms)` for values driven from code.
* Icons: `<Icon name={IconName::Check} class="size-3" />` or `{IconName::Check}`.

## rsx notes

The port needs rsx 0.0.3 or newer: 0.0.2 and 0.0.3 fixed everything the port
first worked around (rsx#7–#36: shadows, fonts, grid, transitions, Tab focus,
hit testing, nested portals, element identity, scrolling, images, ...).

* `//` comments inside markup are **text** (JSX rules); use `{/* ... */}`.
* Unknown class names are no-ops on GPUI; debug builds print them once
  (`RSX_QUIET_CLASSES=1` silences that).
* Components with an `Attributes` prop accept any style method and `ref`
  (`<Button w={px(240.)} ref={node} />`), and a caller's attributes written
  after a reactive binding win over it.
* An endless animation that reads `rsx::animation_frame()` keeps GPUI's test
  executor busy (rsx#42); `utils::set_reduced_motion(true)` turns animations
  off in tests.
* Still missing in rsx: a clipboard API and resize cursors (rsx#43), keeping the
  placeholder while focused (rsx#38), forwarding `Option<Callback>` props
  (rsx#39), logical bubbling out of nested portals (rsx#40), settled
  transitions on the HTML target (rsx#41). Pointer capture doesn't exist either,
  so drags (slider, resizable, scroll thumbs) put a transparent portal over the
  window while dragging.

## Checking your work

* Build: `cargo build -p gallery-rsx` (it compiles every `registry-rsx/ui/*.rsx`
  and `crates/gallery-rsx/src/**/*.rsx`; rsx compile errors point at the `.rsx` line).
* Behavior on real GPUI layout and input: `cargo test -p gallery-rsx`
  (`src/tests.rsx` renders every page in light and dark mode; add a test per
  overlay/interaction: `mount`, `center(node_ref)`, `cx.simulate_click`,
  `cx.simulate_keystrokes("escape")`, `settle(cx)`).
* Looks: `GALLERY_PAGE=<page> gallery-rsx --html` renders a page through rsx's
  HTML target, which you can screenshot in a browser and compare with the
  gpui gallery (`cargo run -p gallery`, `GALLERY_PAGE=<page>`).

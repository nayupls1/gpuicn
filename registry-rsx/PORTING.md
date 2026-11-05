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
* Animations: `utils::use_animation_clock()` gives a ticking seconds signal; there
  are no transforms on divs, but `<svg>` supports `transform`.
* Icons: `<Icon name={IconName::Check} class="size-3" />` or `{IconName::Check}`.

## rsx gotchas

* `//` comments inside markup are **text** (JSX rules); use `{/* ... */}`.
* Children can't be forwarded as `Children` directly: pass `trigger={move || children.render()}`.
* `Callback` isn't `PartialEq`, so keep callbacks in a `StoredValue`, not a `Signal`.
* Float literals passed to `f32` props warn (`float_literal_f32_fallback`): write `4.0f32`.
* Unknown attributes call the `Styled`/`Div` method of the same name
  (`corner_radius={px(6.)}`, `w={px(10.)}`, `bind={move |el| el.bg(..)}`).
  A reactive expression (`.get()` inside) becomes a binding, and bindings are
  layered **above** static styles, so a caller's static class can't override a
  reactive default unless the whole root is inside a `move ||` (the pattern above).
* Class names rsx doesn't know (e.g. `shrink-0`, `truncate`, `whitespace-nowrap`,
  `shadow-md`, `z-10`, `font-mono`, `select-none`, `rounded-t-md`) are silently
  ignored on GPUI. Use the known ones (`flex-shrink-0`, `flex-none`, ...) or
  style methods. See the list of shorthands in rsx's `styled.rs` build script.
* Missing rsx features are tracked as issues in nayupls1/rsx; leave a
  `// rsx#N: ...` note where a workaround or a visual difference remains.

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

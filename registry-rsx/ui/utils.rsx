//! Small helpers shared by every component: the rsx counterpart of
//! shadcn's `lib/utils.ts`.

use rsx::prelude::*;
use rsx::{Div, Length, Scope};

use super::theme::{mix, ColorExt, Theme};

thread_local! {
    /// Whether focus last moved by keyboard, i.e. whether `focus-visible`
    /// rings should show. Tab sets it; a pointer press on a component clears it.
    static KEYBOARD_FOCUS: Signal<bool> = {
        let scope = Scope::root();
        let s = scope.run(|| signal(false));
        std::mem::forget(scope);
        s
    };
}

/// Called by `ui::init`. Nothing to install: rsx moves focus on Tab /
/// Shift-Tab itself (between buttons, fields, links and `tabIndex >= 0`
/// elements; `rsx::focus_next()` / `focus_prev()` do it from code) and styles
/// `focus-visible:` classes. The keyboard-focus flag here is only for rings
/// drawn as overlay children (see [`ring`]), which a `focus-visible:` class
/// on the focused element can't reach; components set it from their key
/// handlers (see [`on_tab_key`]).
pub fn init() {}

/// Whether focus is currently being driven by the keyboard (tracked).
pub fn keyboard_focus_mode() -> bool {
    KEYBOARD_FOCUS.with(|k| k.get())
}

/// Enter keyboard-focus mode (focus rings show).
pub fn keyboard_focus() {
    KEYBOARD_FOCUS.with(|k| k.set(true));
}

/// Forget keyboard focus mode (call on pointer presses).
pub fn pointer_pressed() {
    KEYBOARD_FOCUS.with(|k| k.set(false));
}

/// Put on a focusable element's `onKeyDown`: Tab / Shift-Tab switch to
/// keyboard-focus mode so [`is_focus_visible`] rings show. It only observes
/// the key: rsx still moves focus to the next tab stop (don't call
/// `prevent_default`).
pub fn on_tab_key(e: &KeyboardEvent) {
    if e.key == "Tab" {
        keyboard_focus();
    }
}

/// `:focus-visible` for `node`: focused, and focus arrived by keyboard (tracked).
pub fn is_focus_visible(node: NodeRef) -> bool {
    node.is_focused() && keyboard_focus_mode()
}

/// The surface of a form control (`bg-transparent dark:bg-input/30 shadow-xs`).
///
/// GPUI paints box shadows as a filled shape *under* the element, so a
/// shadow only looks right on an opaque background: in light mode controls
/// get the page background and `shadow-xs`; in dark mode they stay
/// translucent and skip the (invisible anyway) shadow.
pub fn control_surface(theme: &Theme, el: Div) -> Div {
    if theme.is_dark() {
        el.bg(theme.input.opacity(0.3))
    } else if theme.shadows {
        el.bg(theme.background).class("shadow-xs")
    } else {
        el.bg(theme.background)
    }
}

/// Border for small toggles (checkbox, radio). Light mode nudges
/// `border-input` toward the foreground so it reads without a shadow.
pub fn control_border(theme: &Theme) -> Color {
    if theme.is_dark() {
        theme.input
    } else {
        mix(theme.input, theme.foreground, 0.12)
    }
}

/// A tailwind `ring-[width]` drawn *outside* an element, as an overlay child.
///
/// Add it as the last child of a `relative` element whose border is
/// `border` px wide and whose corner radius is `radius` px.
pub fn ring(color: Color, width: f32, radius: f32, border: f32) -> Div {
    let inset = -(width + border);
    div()
        .class("absolute")
        .top(px(inset))
        .left(px(inset))
        .right(px(inset))
        .bottom(px(inset))
        .corner_radius(px(radius + width))
        .border_width(px(width))
        .border_color(color)
}

/// The standard `focus-visible:ring-[3px] ring-ring/50` focus ring (see [`ring`]).
pub fn focus_ring(theme: &Theme, radius: f32) -> Div {
    ring(theme.ring.opacity(0.5), 3.0, radius, 1.0)
}

/// The `aria-invalid:ring-destructive/20` ring (see [`ring`]).
pub fn invalid_ring(theme: &Theme, radius: f32) -> Div {
    let alpha = if theme.is_dark() { 0.4 } else { 0.2 };
    ring(theme.destructive.opacity(alpha), 3.0, radius, 1.0)
}

/// Pixels of a length (rems at 16px), for arithmetic on radii and sizes.
pub fn to_px(l: Length) -> f32 {
    l.to_px(16.0, 0.0).unwrap_or(0.0)
}

/// Which side of the trigger floating content appears on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

/// How floating content is aligned along the trigger's edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    Start,
    #[default]
    Center,
    End,
}

/// Horizontal or vertical layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// The order of the parts registered with a parent (accordion items, table
/// rows, breadcrumb entries, ...), for styles that depend on position, like
/// Tailwind's `first:` / `last:`. The parent creates one and hands it to its
/// parts through context; each part calls [`Siblings::register`] once.
///
/// Parts are ordered by creation, so a part inserted later in the middle of
/// a list counts as the last one.
#[derive(Clone, Copy)]
pub struct Siblings {
    ids: Signal<Vec<u64>>,
    next: StoredValue<u64>,
}

impl Default for Siblings {
    fn default() -> Self {
        Self::new()
    }
}

impl Siblings {
    pub fn new() -> Self {
        Siblings { ids: signal(Vec::new()), next: StoredValue::new(0) }
    }

    /// Adds a part at the end and returns its id. The part is removed (on the
    /// next tick) when the scope that registered it is disposed.
    pub fn register(self) -> u64 {
        let id = self.next.get_value();
        self.next.set_value(id + 1);
        self.ids.update(|ids| ids.push(id));
        // Removed a tick later: when the whole parent is being disposed, the
        // parent's readers must not re-run against parts already gone.
        on_cleanup(move || {
            spawn_local(async move {
                if self.ids.is_alive() {
                    self.ids.update(|ids| ids.retain(|i| *i != id));
                }
            })
        });
        id
    }

    /// The part's position (tracked).
    pub fn index(self, id: u64) -> Option<usize> {
        self.ids.with(|ids| ids.iter().position(|i| *i == id))
    }

    /// How many parts are registered (tracked).
    pub fn len(self) -> usize {
        self.ids.with(|ids| ids.len())
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Whether `id` is the first part (tracked).
    pub fn is_first(self, id: u64) -> bool {
        self.ids.with(|ids| ids.first() == Some(&id))
    }

    /// Whether `id` is the last part (tracked).
    pub fn is_last(self, id: u64) -> bool {
        self.ids.with(|ids| ids.last() == Some(&id))
    }
}

thread_local! {
    /// Seconds since the clock started, and how many components use it
    /// (see [`use_animation_clock`]).
    static CLOCK: (Signal<f32>, std::cell::Cell<usize>) = {
        let scope = Scope::root();
        let s = scope.run(|| signal(0.0f32));
        std::mem::forget(scope);
        (s, std::cell::Cell::new(0))
    };
}

thread_local! {
    static REDUCED_MOTION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// `prefers-reduced-motion`: when set, animations created afterwards don't
/// run (transitions jump to their end, spinners and pulses hold still). Also
/// useful in tests, where an endless animation keeps the app from ever idling.
pub fn set_reduced_motion(reduced: bool) {
    REDUCED_MOTION.with(|r| r.set(reduced));
}

/// See [`set_reduced_motion`].
pub fn reduced_motion() -> bool {
    REDUCED_MOTION.with(|r| r.get())
}

/// A clock for animations (spinners, skeleton pulses): returns a signal
/// holding seconds since start, which ticks every ~16ms while at least one
/// component that called this is alive. Prefer rsx's own `animate-spin` /
/// `animate-pulse` classes where they fit.
///
/// It runs on a timer (`sleep`, GPUI's executor timer) rather than
/// [`rsx::animation_frame`]: an endless animation driven by frames keeps
/// GPUI's test executor busy forever (`run_until_parked` never returns),
/// while a timer only fires when a test advances the clock.
pub fn use_animation_clock() -> ReadSignal<f32> {
    let time = CLOCK.with(|(s, _)| *s);
    if reduced_motion() {
        return time.read_only();
    }
    let users = CLOCK.with(|(_, users)| {
        users.set(users.get() + 1);
        users.get()
    });
    if users == 1 {
        let start = std::time::Instant::now();
        spawn_local(async move {
            while CLOCK.with(|(_, u)| u.get()) > 0 {
                time.set(start.elapsed().as_secs_f32());
                sleep(std::time::Duration::from_millis(16)).await;
            }
        });
    }
    on_cleanup(|| CLOCK.with(|(_, u)| u.set(u.get().saturating_sub(1))));
    time.read_only()
}

/// Ease in-out (cubic), for transitions driven by [`use_transition`].
pub fn ease_in_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
}

/// A one-shot transition (gpui's `with_animation`): returns a signal that
/// goes from 0 to 1 over `ms` milliseconds after the call, then stops
/// ticking. Create it where the animated element is created, so it replays
/// each time the element appears; shape it with [`ease_in_out`].
///
/// Driven by [`rsx::animation_frame`]. For opacity, colors and translation,
/// rsx's `transition duration-* ease-*` classes animate a changed value by
/// themselves; this is for everything else (sizes, custom drawing).
pub fn use_transition(ms: u64) -> ReadSignal<f32> {
    let ms = if reduced_motion() { 0 } else { ms };
    let progress = signal(if ms == 0 { 1.0f32 } else { 0.0 });
    if ms > 0 {
        let start = std::time::Instant::now();
        effect(move || {
            if progress.get_untracked() >= 1.0 {
                return;
            }
            // Re-runs every frame until done (then it reads nothing and stops).
            rsx::animation_frame();
            let t = (start.elapsed().as_secs_f32() * 1000.0 / ms as f32).min(1.0);
            progress.set(t);
        });
    }
    progress.read_only()
}

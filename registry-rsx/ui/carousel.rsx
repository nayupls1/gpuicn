//! A carousel with motion and swipe-free navigation buttons.
//!
//! ```ignore
//! <Carousel class="w-[320px]" items={(1..=5).map(|n| <Card>{n}</Card>).collect::<Vec<_>>()} />
//! <Carousel per_view={3} looping items={logos} />
//! <Carousel vertical class="h-[200px]" items={slides} />
//! ```

use std::time::Duration;

use rsx::prelude::*;

use super::button::{Button, ButtonSize};
use super::icon::IconName;
use super::utils::{ease_in_out, Orientation};

/// The gap between slides (`-ml-4` on the track, `pl-4` on each item).
const GAP: f32 = 16.0;
/// How long a move between slides takes.
const SLIDE_MS: f32 = 300.0;

#[component]
pub fn Carousel(
    /// The slides.
    #[prop(optional)]
    items: Vec<Element>,
    /// Items visible at once (`basis-1/3` → 3).
    #[prop(default = 1)]
    per_view: usize,
    #[prop(optional)] orientation: Orientation,
    /// Shorthand for `orientation={Orientation::Vertical}`.
    vertical: bool,
    /// Wrap around at the ends (`opts={{ loop: true }}`).
    looping: bool,
    /// Hide the previous/next buttons.
    hide_controls: bool,
    /// Called with the new index after a move.
    on_change: Option<Callback<usize>>,
    attrs: Attributes,
) -> impl IntoElement {
    let per_view = per_view.max(1);
    let vertical = vertical || orientation == Orientation::Vertical;
    let count = items.len();
    let last = count.saturating_sub(per_view);
    let index = signal(0usize);
    // Where the current move started (in items) and how far along it is (0..1).
    let from = signal(0.0f32);
    let progress = signal(1.0f32);
    let generation = signal(0u64);
    let viewport = NodeRef::new();

    let extent = move || {
        viewport.bounds().map(|b| if vertical { b.size.height } else { b.size.width }).unwrap_or(0.0)
    };
    let item_extent = move || extent() / per_view as f32;
    // The track's offset in items, mid-animation.
    let position = move || {
        let p = ease_in_out(progress.get());
        let (a, b) = (from.get(), index.get() as f32);
        a + (b - a) * p
    };

    let go = move |delta: isize| {
        let current = index.get_untracked();
        let target = current as isize + delta;
        let next = if looping {
            target.rem_euclid(last as isize + 1) as usize
        } else {
            target.clamp(0, last as isize) as usize
        };
        // Start from wherever the track is now, so quick clicks don't jump.
        from.set(position_untracked(from, progress, index));
        index.set(next);
        progress.set(0.0);
        let generation_now = generation.get_untracked() + 1;
        generation.set(generation_now);
        spawn_local(async move {
            let start = std::time::Instant::now();
            loop {
                if !generation.is_alive() || generation.get_untracked() != generation_now {
                    break;
                }
                let t = (start.elapsed().as_secs_f32() * 1000.0 / SLIDE_MS).min(1.0);
                progress.set(t);
                if t >= 1.0 {
                    break;
                }
                sleep(Duration::from_millis(16)).await;
            }
        });
        if let Some(f) = on_change {
            f.call(&next);
        }
    };
    let can_prev = move || looping || index.get() > 0;
    let can_next = move || looping || index.get() < last;

    let slides = items.into_iter().map(|item| {
        <div
            class="flex-none"
            bind={move |el: rsx::Div| {
                // Before the viewport is measured (and on targets that don't
                // measure, like HTML), size items as a fraction of the track.
                let size = if extent() > 0.0 { px(item_extent()) } else { relative(1.0 / per_view as f32) };
                if vertical { el.h(size).pt(px(GAP)) } else { el.w(size).pl(px(GAP)) }
            }}
        >
            {item}
        </div>
    }).collect::<Vec<_>>();

    let track = <div
        class="flex"
        class:flex-col={vertical}
        bind={move |el: rsx::Div| {
            // rsx#16: no transforms; the track moves by its margin, like gpuicn's.
            let offset = px(-GAP - item_extent() * position());
            if vertical { el.mt(offset) } else { el.ml(offset) }
        }}
    >
        {slides}
    </div>;

    let (prev_icon, next_icon) = if vertical { (IconName::ArrowUp, IconName::ArrowDown) } else { (IconName::ArrowLeft, IconName::ArrowRight) };
    let nav = move |icon: IconName, enabled: Box<dyn Fn() -> bool>, delta: isize| {
        <Button
            outline
            size={ButtonSize::IconSm}
            class="rounded-full"
            icon={icon}
            disabled={move || !enabled()}
            onClick={move |_| go(delta)}
        />
    };
    let controls = (!hide_controls).then(|| {
        let prev = nav(prev_icon, Box::new(can_prev), -1);
        let next = nav(next_icon, Box::new(can_next), 1);
        if vertical {
            <>
                <div class="absolute top-[-48px] left-0 right-0 flex justify-center">{prev}</div>
                <div class="absolute bottom-[-48px] left-0 right-0 flex justify-center">{next}</div>
            </>
        } else {
            <>
                <div class="absolute left-[-48px] top-0 bottom-0 flex items-center">{prev}</div>
                <div class="absolute right-[-48px] top-0 bottom-0 flex items-center">{next}</div>
            </>
        }
    });

    <div class="relative flex" {...attrs}>
        <div ref={viewport} class="relative flex flex-col overflow-hidden size-full">{track}</div>
        {controls}
    </div>
}

fn position_untracked(from: Signal<f32>, progress: Signal<f32>, index: Signal<usize>) -> f32 {
    let p = ease_in_out(progress.get_untracked());
    let (a, b) = (from.get_untracked(), index.get_untracked() as f32);
    a + (b - a) * p
}

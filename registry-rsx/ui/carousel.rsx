//! A carousel with motion and swipe-free navigation buttons.
//!
//! ```ignore
//! <Carousel class="w-[320px]" items={(1..=5).map(|n| <Card>{n}</Card>).collect::<Vec<_>>()} />
//! <Carousel per_view={3} looping items={logos} />
//! <Carousel vertical class="h-[200px]" items={slides} />
//! ```

use rsx::prelude::*;

use super::button::{Button, ButtonSize};
use super::icon::IconName;
use super::utils::Orientation;

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
    let viewport = NodeRef::new();

    let extent = move || {
        viewport.bounds().map(|b| if vertical { b.size.height } else { b.size.width }).unwrap_or(0.0)
    };
    let item_extent = move || extent() / per_view as f32;

    let go = move |delta: isize| {
        let target = index.get_untracked() as isize + delta;
        let next = if looping {
            target.rem_euclid(last as isize + 1) as usize
        } else {
            target.clamp(0, last as isize) as usize
        };
        index.set(next);
        if let Some(f) = on_change {
            f.call(&next);
        }
    };
    let can_prev = move || looping || index.get() > 0;
    let can_next = move || looping || index.get() < last;

    // Before the viewport is measured (and on targets that don't measure,
    // like HTML), items are sized as a fraction of the track.
    let item_size = move || if extent() > 0.0 { px(item_extent()) } else { relative(1.0 / per_view as f32) };
    let slides = items.into_iter().map(|item| {
        <div
            class="flex-none"
            class:pt-4={vertical}
            class:pl-4={!vertical}
            w={move || if vertical { relative(1.0) } else { item_size() }}
            h={move || if vertical { item_size() } else { relative(1.0) }}
        >
            {item}
        </div>
    }).collect::<Vec<_>>();

    // The track slides by a transform, eased over 300ms like gpuicn's animation.
    let offset = move || -item_extent() * index.get() as f32;
    // The gap between slides is `pl-4` on each item, offset by `-ml-4` on the track.
    let track = <div
        class={if vertical { "flex flex-col -mt-4 transition-transform duration-300 ease-in-out" } else { "flex -ml-4 transition-transform duration-300 ease-in-out" }}
        translate_x={move || px(if vertical { 0.0 } else { offset() })}
        translate_y={move || px(if vertical { offset() } else { 0.0 })}
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

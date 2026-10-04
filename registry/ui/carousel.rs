//! A carousel with motion and swipe-free navigation buttons.
//!
//! ```ignore
//! Carousel::new("photos")
//!     .w(px(320.))
//!     .items((1..=5).map(|n| Card::new().child(CardContent::new().child(n.to_string()))))
//! Carousel::new("logos").per_view(3).looping()
//! ```

use gpui::{
    Animation, AnimationExt, AnyElement, App, ElementId, IntoElement, Pixels, RenderOnce,
    StyleRefinement, Window, canvas, div, prelude::*, px,
};
use std::rc::Rc;
use std::time::Duration;

use super::button::{Button, ButtonSize};
use super::icon::IconName;
use super::utils::{Orientation, StyledExt};

struct CarouselState {
    index: usize,
    from: usize,
    extent: Pixels,
}

#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    items: Vec<AnyElement>,
    per_view: usize,
    orientation: Orientation,
    looping: bool,
    show_controls: bool,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Carousel);

impl Carousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            per_view: 1,
            orientation: Orientation::Horizontal,
            looping: false,
            show_controls: true,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: impl IntoElement) -> Self {
        self.items.push(item.into_any_element());
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.items.extend(items.into_iter().map(|i| i.into_any_element()));
        self
    }

    /// Items visible at once (`basis-1/3` → 3).
    pub fn per_view(mut self, per_view: usize) -> Self {
        self.per_view = per_view.max(1);
        self
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = Orientation::Vertical;
        self
    }

    /// Wrap around at the ends (`opts={{ loop: true }}`).
    pub fn looping(mut self) -> Self {
        self.looping = true;
        self
    }

    /// Hide the previous/next buttons.
    pub fn hide_controls(mut self) -> Self {
        self.show_controls = false;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Carousel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| CarouselState { index: 0, from: 0, extent: px(0.) });
        let (index, from, extent) = {
            let s = state.read(cx);
            (s.index, s.from, s.extent)
        };
        let count = self.items.len();
        let last = count.saturating_sub(self.per_view);
        let vertical = self.orientation == Orientation::Vertical;
        let item_extent = extent / self.per_view as f32;
        let gap = px(16.);

        let go = |delta: isize| {
            let state = state.clone();
            let on_change = self.on_change.clone();
            let looping = self.looping;
            move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                let next = state.update(cx, |s, cx| {
                    let target = s.index as isize + delta;
                    let next = if looping {
                        target.rem_euclid(last as isize + 1) as usize
                    } else {
                        target.clamp(0, last as isize) as usize
                    };
                    s.from = s.index;
                    s.index = next;
                    cx.notify();
                    next
                });
                if let Some(handler) = &on_change {
                    handler(&next, window, cx);
                }
            }
        };
        let can_prev = self.looping || index > 0;
        let can_next = self.looping || index < last;

        let (from_offset, to_offset) = (item_extent * from as f32, item_extent * index as f32);
        let track = div()
            .flex()
            .when(vertical, |this| this.flex_col())
            .map(|this| if vertical { this.mt(-gap) } else { this.ml(-gap) })
            .children(self.items.into_iter().map(|item| {
                div()
                    .flex_none()
                    .map(|this| {
                        if vertical { this.h(item_extent).pt(gap) } else { this.w(item_extent).pl(gap) }
                    })
                    .child(item)
            }))
            .with_animation(
                ElementId::NamedInteger("carousel-slide".into(), (from * 1000 + index) as u64),
                Animation::new(Duration::from_millis(300)).with_easing(gpui::ease_in_out),
                move |this, delta| {
                    let offset = from_offset + (to_offset - from_offset) * delta;
                    if vertical { this.mt(-gap - offset) } else { this.ml(-gap - offset) }
                },
            );

        let viewport = div()
            .relative()
            .overflow_hidden()
            .size_full()
            .child(track)
            .child({
                let state = state.clone();
                canvas(
                    move |bounds, _, cx| {
                        let extent = if vertical { bounds.size.height } else { bounds.size.width };
                        state.update(cx, |s, cx| {
                            if s.extent != extent {
                                s.extent = extent;
                                cx.notify();
                            }
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full()
            });

        let nav_button = |id: &'static str, icon: IconName, enabled: bool| {
            Button::new(id).outline().size(ButtonSize::IconSm).rounded_full().icon(icon).disabled(!enabled)
        };

        div()
            .id(self.id)
            .relative()
            .refine_style(&self.style)
            .child(viewport)
            .when(self.show_controls, |this| {
                let prev = nav_button("carousel-prev", if vertical { IconName::ArrowUp } else { IconName::ArrowLeft }, can_prev)
                    .on_click(go(-1));
                let next = nav_button("carousel-next", if vertical { IconName::ArrowDown } else { IconName::ArrowRight }, can_next)
                    .on_click(go(1));
                if vertical {
                    this.child(div().absolute().top(px(-48.)).left_0().right_0().flex().justify_center().child(prev))
                        .child(div().absolute().bottom(px(-48.)).left_0().right_0().flex().justify_center().child(next))
                } else {
                    this.child(div().absolute().left(px(-48.)).top_0().bottom_0().flex().items_center().child(prev))
                        .child(div().absolute().right(px(-48.)).top_0().bottom_0().flex().items_center().child(next))
                }
            })
    }
}

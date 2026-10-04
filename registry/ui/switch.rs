//! A control that allows the user to toggle between checked and not checked.
//!
//! ```ignore
//! Switch::new("airplane").label("Airplane Mode")
//! ```

use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Window, div, prelude::*, px,
};
use std::rc::Rc;
use std::time::Duration;

use super::label::Label;
use super::theme::ActiveTheme;
use super::utils::StyledExt;

#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: Option<bool>,
    default_checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    on_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    style: StyleRefinement,
}

crate::impl_styled!(Switch);

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: None,
            default_checked: false,
            disabled: false,
            label: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn default_checked(mut self, checked: bool) -> Self {
        self.default_checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default = self.default_checked;
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| default);
        let checked = self.checked.unwrap_or(*state.read(cx));
        let theme = cx.theme();

        let (track, thumb) = match (checked, theme.is_dark()) {
            (true, false) => (theme.primary, theme.background),
            (true, true) => (theme.primary, theme.primary_foreground),
            (false, false) => (theme.input, theme.background),
            (false, true) => (theme.input.opacity(0.8), theme.foreground),
        };
        // Track is 32×18.4 with a 1px border; the 16px thumb travels 14px.
        let travel = 14.0;
        let (from, to) = if checked { (0.0, travel) } else { (travel, 0.0) };

        let track_el = div()
            .flex()
            .flex_none()
            .items_center()
            .w_8()
            .h(px(18.4))
            .rounded_full()
            .border_1()
            .border_color(gpui::transparent_black())
            .bg(track)
            .when(theme.shadows, |this| this.shadow_xs())
            .refine_style(&self.style)
            .child(
                div()
                    .size_4()
                    .rounded_full()
                    .bg(thumb)
                    .with_animation(
                        ElementId::NamedInteger("thumb".into(), checked as u64),
                        Animation::new(Duration::from_millis(150)).with_easing(gpui::ease_in_out),
                        move |this, delta| this.ml(px(from + (to - from) * delta)),
                    ),
            );

        let disabled = self.disabled;
        let on_change = self.on_change;
        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .when(disabled, |this| this.opacity(0.5))
            .when(!disabled, |this| this.cursor_pointer())
            .child(track_el)
            .children(self.label.map(Label::new))
            .when(!disabled, |this| {
                this.on_click(move |_, window, cx| {
                    let next = !checked;
                    state.update(cx, |s, cx| {
                        *s = next;
                        cx.notify();
                    });
                    if let Some(handler) = &on_change {
                        handler(&next, window, cx);
                    }
                })
            })
    }
}

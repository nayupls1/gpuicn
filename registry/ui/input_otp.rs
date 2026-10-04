//! Accessible one-time password component with copy paste functionality.
//!
//! ```ignore
//! InputOtp::new("otp")
//!     .length(6)
//!     .groups(&[3, 3])
//!     .on_complete(|code, _, _| println!("{code}"))
//! ```

use gpui::{
    App, ElementId, FocusHandle, IntoElement, RenderOnce, SharedString, StyleRefinement, Window,
    div, prelude::*, px,
};
use std::rc::Rc;

use super::icon::{Icon, IconName};
use super::theme::ActiveTheme;
use super::utils::{StyledExt, ring_shadow};

/// Which characters a slot accepts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OtpPattern {
    #[default]
    Digits,
    Alphanumeric,
}

impl OtpPattern {
    fn accepts(self, c: char) -> bool {
        match self {
            OtpPattern::Digits => c.is_ascii_digit(),
            OtpPattern::Alphanumeric => c.is_ascii_alphanumeric(),
        }
    }
}

struct OtpState {
    value: String,
    focus_handle: FocusHandle,
}

type OtpCallback = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct InputOtp {
    id: ElementId,
    length: usize,
    groups: Vec<usize>,
    pattern: OtpPattern,
    value: Option<SharedString>,
    disabled: bool,
    on_change: Option<OtpCallback>,
    on_complete: Option<OtpCallback>,
    style: StyleRefinement,
}

crate::impl_styled!(InputOtp);

impl InputOtp {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            length: 6,
            groups: Vec::new(),
            pattern: OtpPattern::Digits,
            value: None,
            disabled: false,
            on_change: None,
            on_complete: None,
            style: StyleRefinement::default(),
        }
    }

    /// Number of slots (`maxLength`).
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// Split slots into groups with a separator between, e.g. `&[3, 3]`.
    pub fn groups(mut self, groups: &[usize]) -> Self {
        self.groups = groups.to_vec();
        self
    }

    pub fn pattern(mut self, pattern: OtpPattern) -> Self {
        self.pattern = pattern;
        self
    }

    /// Controlled value.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Called once every slot is filled.
    pub fn on_complete(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_complete = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for InputOtp {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| OtpState {
            value: String::new(),
            focus_handle: cx.focus_handle(),
        });
        if let Some(value) = &self.value {
            state.update(cx, |s, _| {
                if s.value != value.as_ref() {
                    s.value = value.to_string();
                }
            });
        }
        let (value, focus_handle) = {
            let s = state.read(cx);
            (s.value.clone(), s.focus_handle.clone())
        };
        let focused = focus_handle.is_focused(window);
        let theme = cx.theme();
        let radius = theme.radius_md();
        let length = self.length;
        let groups = if self.groups.is_empty() { vec![length] } else { self.groups.clone() };
        let chars: Vec<char> = value.chars().collect();
        let active = chars.len().min(length.saturating_sub(1));

        let mut slot_ix = 0;
        let mut group_els = Vec::new();
        for (group_ix, group_len) in groups.iter().enumerate() {
            if group_ix > 0 {
                group_els.push(div().flex().items_center().child(Icon::new(IconName::Minus)).into_any_element());
            }
            let mut slots = Vec::new();
            for i in 0..*group_len {
                let ix = slot_ix;
                slot_ix += 1;
                let is_active = focused && ix == active;
                let (first, last) = (i == 0, i + 1 == *group_len);
                slots.push(
                    div()
                        .relative()
                        .flex()
                        .size_9()
                        .items_center()
                        .justify_center()
                        .border_y_1()
                        .border_r_1()
                        .border_color(theme.input)
                        .when(theme.is_dark(), |this| this.bg(theme.input.opacity(0.3)))
                        .when(first, |this| this.border_l_1().rounded_l(radius))
                        .when(last, |this| this.rounded_r(radius))
                        .when(theme.shadows, |this| this.shadow_xs())
                        .text_sm()
                        .when(is_active, |this| {
                            this.border_color(theme.ring).shadow(vec![ring_shadow(theme.ring.opacity(0.5), px(3.))])
                        })
                        .children(chars.get(ix).map(|c| c.to_string()))
                        .when(is_active && chars.len() < length, |this| {
                            this.child(div().h_4().w_px().bg(theme.foreground))
                        }),
                );
            }
            group_els.push(div().flex().items_center().children(slots).into_any_element());
        }

        let pattern = self.pattern;
        let (on_change, on_complete) = (self.on_change, self.on_complete);
        let key_state = state.clone();
        let disabled = self.disabled;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap_2()
            .when(disabled, |this| this.opacity(0.5))
            .refine_style(&self.style)
            .when(!disabled, |this| {
                this.track_focus(&focus_handle)
                    .cursor_text()
                    .on_click(move |_, window, _| window.focus(&focus_handle))
                    .on_key_down(move |event, window, cx| {
                        let mut value = key_state.read(cx).value.clone();
                        let before = value.clone();
                        let paste = event.keystroke.modifiers.secondary() && event.keystroke.key == "v";
                        if event.keystroke.key == "backspace" {
                            value.pop();
                        } else if paste {
                            if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                                value.extend(text.chars().filter(|c| pattern.accepts(*c)));
                            }
                        } else if let Some(c) = event.keystroke.key_char.as_ref().and_then(|s| s.chars().next())
                            && !event.keystroke.modifiers.secondary()
                            && pattern.accepts(c)
                        {
                            value.push(c);
                        }
                        value = value.chars().take(length).collect();
                        if value == before {
                            return;
                        }
                        cx.stop_propagation();
                        key_state.update(cx, |s, cx| {
                            s.value = value.clone();
                            cx.notify();
                        });
                        let value: SharedString = value.into();
                        if let Some(handler) = &on_change {
                            handler(&value, window, cx);
                        }
                        if value.chars().count() == length && let Some(handler) = &on_complete {
                            handler(&value, window, cx);
                        }
                    })
            })
            .children(group_els)
    }
}

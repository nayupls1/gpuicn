//! Used to display textual user input from keyboard.
//!
//! ```ignore
//! <KbdGroup><Kbd text="⌘" /><Kbd text="K" /></KbdGroup>
//! ```

use rsx::prelude::*;

use super::icon::{Icon, IconName};
use super::theme::theme;

/// The icon drawn for a modifier symbol, if any. Most UI fonts (Geist,
/// Inter, system fonts) lack ⌘/⌥, and fallback glyphs come out mismatched.
pub fn modifier_icon(text: &str) -> Option<IconName> {
    match text {
        "⌘" => Some(IconName::Command),
        "⇧" => Some(IconName::ArrowBigUp),
        "⌥" => Some(IconName::Option),
        "⌃" => Some(IconName::ChevronUp),
        _ => None,
    }
}

/// A key cap. Like gpuicn's and shadcn's (`font-sans`), it uses the UI font,
/// not `Theme::mono_font_family`.
#[component]
pub fn Kbd(
    #[prop(optional)] text: Accessor<String>,
    children: Option<Children>,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    move || {
        let theme = t.get();
        let text = text.get();
        <div
            class="flex flex-none items-center justify-center gap-1 h-5 min-w-5 px-1 text-xs leading-[1rem] font-medium"
            corner_radius={theme.radius_sm()}
            bg={theme.muted}
            text_color={theme.muted_foreground}
            {...&attrs}
        >
            {match modifier_icon(&text) {
                Some(icon) => <Icon name={icon} class="size-3" />,
                None => text.into_element(),
            }}
            {children}
        </div>
    }
}

#[component]
pub fn KbdGroup(children: Option<Children>, attrs: Attributes) -> impl IntoElement {
    <div class="flex items-center gap-1" {...attrs}>{children}</div>
}

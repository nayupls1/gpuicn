//! Displays a form input field or a component that looks like an input field.
//!
//! The editing engine here (selection, clipboard, IME, wrapping) is shared by
//! `textarea`, `input_group`, `command` and `combobox`.
//!
//! Call `input::init(cx)` (done by `ui::init`) to install the key bindings.
//!
//! ```ignore
//! // Self-managed: state lives as long as the element is rendered.
//! Input::new("email")
//!     .placeholder("Email")
//!     .on_change(|text, _, _| println!("{text}"))
//!
//! // Or own the state to read/write it from your view:
//! let email = cx.new(|cx| InputState::new(window, cx).placeholder("Email"));
//! Input::from_state(&email)
//! email.read(cx).text()
//! ```

use gpui::{
    AnyElement, App, Bounds, ClipboardItem, Context, ContentMask, CursorStyle, Element,
    ElementId, ElementInputHandler, Entity, EntityInputHandler, EventEmitter, FocusHandle,
    Focusable, GlobalElementId, Hsla, InspectorElementId, IntoElement, KeyBinding, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, Point,
    RenderOnce, SharedString, Style, StyleRefinement, TextAlign, TextRun, UTF16Selection,
    UnderlineStyle, Window, WrappedLine, actions, div, fill, point, prelude::*, px, relative,
    size,
};
use std::ops::Range;
use std::rc::Rc;
use unicode_segmentation::UnicodeSegmentation;

use super::theme::ActiveTheme;
use super::utils::{StyledExt, ring_shadow};

actions!(
    input,
    [
        Backspace,
        Delete,
        DeleteWordLeft,
        Left,
        Right,
        Up,
        Down,
        WordLeft,
        WordRight,
        SelectLeft,
        SelectRight,
        SelectUp,
        SelectDown,
        SelectWordLeft,
        SelectWordRight,
        SelectAll,
        Home,
        End,
        SelectHome,
        SelectEnd,
        Enter,
        Paste,
        Copy,
        Cut,
        ShowCharacterPalette,
    ]
);

pub const KEY_CONTEXT: &str = "Input";

/// Install the editing key bindings. Called by `ui::init`.
pub fn init(cx: &mut App) {
    let word = if cfg!(target_os = "macos") { "alt" } else { "ctrl" };
    let ctx = Some(KEY_CONTEXT);
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, ctx),
        KeyBinding::new("shift-backspace", Backspace, ctx),
        KeyBinding::new("delete", Delete, ctx),
        KeyBinding::new(&format!("{word}-backspace"), DeleteWordLeft, ctx),
        KeyBinding::new("left", Left, ctx),
        KeyBinding::new("right", Right, ctx),
        KeyBinding::new("up", Up, ctx),
        KeyBinding::new("down", Down, ctx),
        KeyBinding::new(&format!("{word}-left"), WordLeft, ctx),
        KeyBinding::new(&format!("{word}-right"), WordRight, ctx),
        KeyBinding::new("shift-left", SelectLeft, ctx),
        KeyBinding::new("shift-right", SelectRight, ctx),
        KeyBinding::new("shift-up", SelectUp, ctx),
        KeyBinding::new("shift-down", SelectDown, ctx),
        KeyBinding::new(&format!("{word}-shift-left"), SelectWordLeft, ctx),
        KeyBinding::new(&format!("{word}-shift-right"), SelectWordRight, ctx),
        KeyBinding::new("secondary-a", SelectAll, ctx),
        KeyBinding::new("home", Home, ctx),
        KeyBinding::new("end", End, ctx),
        KeyBinding::new("shift-home", SelectHome, ctx),
        KeyBinding::new("shift-end", SelectEnd, ctx),
        KeyBinding::new("enter", Enter, ctx),
        KeyBinding::new("secondary-v", Paste, ctx),
        KeyBinding::new("secondary-c", Copy, ctx),
        KeyBinding::new("secondary-x", Cut, ctx),
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, ctx),
    ]);
    if cfg!(target_os = "macos") {
        cx.bind_keys([
            KeyBinding::new("cmd-left", Home, ctx),
            KeyBinding::new("cmd-right", End, ctx),
            KeyBinding::new("cmd-shift-left", SelectHome, ctx),
            KeyBinding::new("cmd-shift-right", SelectEnd, ctx),
        ]);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputEvent {
    Change(SharedString),
    Submit(SharedString),
}

type TextCallback = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;

/// The editable text, selection and layout of an input.
pub struct InputState {
    focus_handle: FocusHandle,
    text: SharedString,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    multiline: bool,
    masked: bool,
    disabled: bool,
    layout: Option<InputLayout>,
    is_selecting: bool,
    scroll_x: Pixels,
    on_change: Option<TextCallback>,
    on_submit: Option<TextCallback>,
}

impl EventEmitter<InputEvent> for InputState {}

impl Focusable for InputState {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl InputState {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            text: SharedString::default(),
            placeholder: SharedString::default(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            multiline: false,
            masked: false,
            disabled: false,
            layout: None,
            is_selecting: false,
            scroll_x: px(0.),
            on_change: None,
            on_submit: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// Render every character as `•` (`type="password"`).
    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    pub fn default_value(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self.selected_range = self.text.len()..self.text.len();
        self
    }

    pub fn text(&self) -> &SharedString {
        &self.text
    }

    /// Replace the text (moves the cursor to the end) and emit a change.
    pub fn set_text(&mut self, text: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) {
        self.set_text_silently(text);
        self.changed(window, cx);
    }

    fn set_text_silently(&mut self, text: impl Into<SharedString>) {
        self.text = text.into();
        let len = self.text.len();
        self.selected_range = len..len;
        self.selection_reversed = false;
        self.marked_range = None;
    }

    pub fn focus(&self, window: &mut Window) {
        window.focus(&self.focus_handle);
    }

    pub fn is_focused(&self, window: &Window) -> bool {
        self.focus_handle.is_focused(window)
    }

    fn changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.text.clone();
        cx.emit(InputEvent::Change(text.clone()));
        if let Some(handler) = self.on_change.clone() {
            window.defer(cx, move |window, cx| handler(&text, window, cx));
        }
        cx.notify();
    }

    // ── movement ────────────────────────────────────────────────────────

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed { self.selected_range.start } else { self.selected_range.end }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        cx.notify();
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify();
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.text
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.text
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.text.len())
    }

    fn previous_word(&self, offset: usize) -> usize {
        let before = &self.text[..offset];
        let trimmed = before.trim_end();
        trimmed
            .char_indices()
            .rev()
            .find(|(_, c)| !c.is_alphanumeric() && *c != '_')
            .map(|(ix, c)| ix + c.len_utf8())
            .unwrap_or(0)
    }

    fn next_word(&self, offset: usize) -> usize {
        let after = &self.text[offset..];
        let skip = after.len() - after.trim_start().len();
        let rest = &after[skip..];
        let word = rest
            .char_indices()
            .find(|(_, c)| !c.is_alphanumeric() && *c != '_')
            .map(|(ix, _)| ix)
            .unwrap_or(rest.len());
        offset + skip + word.max(rest.chars().next().map_or(0, |c| c.len_utf8()).min(rest.len()))
    }

    fn line_start(&self, offset: usize) -> usize {
        if !self.multiline {
            return 0;
        }
        self.text[..offset].rfind('\n').map(|ix| ix + 1).unwrap_or(0)
    }

    fn line_end(&self, offset: usize) -> usize {
        if !self.multiline {
            return self.text.len();
        }
        self.text[offset..].find('\n').map(|ix| offset + ix).unwrap_or(self.text.len())
    }

    /// Offset one visual row above/below the cursor (multi-line only).
    fn vertical_offset(&self, direction: f32) -> Option<usize> {
        let layout = self.layout.as_ref()?;
        let cursor = self.to_display(self.cursor_offset());
        let position = layout.position_for_offset(cursor);
        let target = point(position.x, position.y + layout.line_height * direction + layout.line_height / 2.);
        if target.y < px(0.) || target.y > layout.total_height() {
            return None;
        }
        Some(self.from_display(layout.offset_for_position(target)))
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        if !self.multiline {
            cx.propagate();
            return;
        }
        let target = self.vertical_offset(-1.).unwrap_or(0);
        self.move_to(target, cx);
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        if !self.multiline {
            cx.propagate();
            return;
        }
        let target = self.vertical_offset(1.).unwrap_or(self.text.len());
        self.move_to(target, cx);
    }

    fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.previous_word(self.cursor_offset()), cx);
    }

    fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.next_word(self.cursor_offset()), cx);
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        let target = if self.multiline { self.vertical_offset(-1.).unwrap_or(0) } else { 0 };
        self.select_to(target, cx);
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        let len = self.text.len();
        let target = if self.multiline { self.vertical_offset(1.).unwrap_or(len) } else { len };
        self.select_to(target, cx);
    }

    fn select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_word(self.cursor_offset()), cx);
    }

    fn select_word_right(&mut self, _: &SelectWordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_word(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.text.len();
        self.selection_reversed = false;
        cx.notify();
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.line_start(self.cursor_offset()), cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.line_end(self.cursor_offset()), cx);
    }

    fn select_home(&mut self, _: &SelectHome, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.line_start(self.cursor_offset()), cx);
    }

    fn select_end(&mut self, _: &SelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.line_end(self.cursor_offset()), cx);
    }

    // ── editing ─────────────────────────────────────────────────────────

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete_word_left(&mut self, _: &DeleteWordLeft, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_word(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn enter(&mut self, _: &Enter, window: &mut Window, cx: &mut Context<Self>) {
        if self.multiline {
            self.replace_text_in_range(None, "\n", window, cx);
        } else {
            let text = self.text.clone();
            cx.emit(InputEvent::Submit(text.clone()));
            match self.on_submit.clone() {
                Some(handler) => window.defer(cx, move |window, cx| handler(&text, window, cx)),
                None => cx.propagate(),
            }
        }
    }

    fn show_character_palette(&mut self, _: &ShowCharacterPalette, window: &mut Window, _: &mut Context<Self>) {
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() && !self.masked {
            cx.write_to_clipboard(ClipboardItem::new_string(self.text[self.selected_range.clone()].to_string()));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() && !self.masked {
            cx.write_to_clipboard(ClipboardItem::new_string(self.text[self.selected_range.clone()].to_string()));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    // ── mouse ───────────────────────────────────────────────────────────

    fn offset_for_mouse(&self, position: Point<Pixels>) -> usize {
        match &self.layout {
            Some(layout) if !self.text.is_empty() => {
                self.from_display(layout.offset_for_position(position - layout.origin))
            }
            _ => 0,
        }
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.is_selecting = true;
        window.focus(&self.focus_handle);
        let offset = self.offset_for_mouse(event.position);
        if event.modifiers.shift {
            self.select_to(offset, cx);
        } else if event.click_count >= 2 {
            let start = self.previous_word(offset.min(self.next_boundary(offset)));
            let end = self.next_word(start);
            self.selected_range = start..end;
            cx.notify();
        } else {
            self.move_to(offset, cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.offset_for_mouse(event.position), cx);
        }
    }

    // ── offsets ─────────────────────────────────────────────────────────

    const MASK: char = '•';

    fn display_text(&self) -> SharedString {
        if self.masked {
            Self::MASK.to_string().repeat(self.text.chars().count()).into()
        } else {
            self.text.clone()
        }
    }

    fn to_display(&self, offset: usize) -> usize {
        if self.masked {
            self.text[..offset].chars().count() * Self::MASK.len_utf8()
        } else {
            offset
        }
    }

    fn from_display(&self, offset: usize) -> usize {
        if self.masked {
            let chars = offset / Self::MASK.len_utf8();
            self.text.char_indices().nth(chars).map(|(ix, _)| ix).unwrap_or(self.text.len())
        } else {
            offset
        }
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.text.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.text.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn sanitize<'a>(&self, text: &'a str) -> std::borrow::Cow<'a, str> {
        if self.multiline {
            text.into()
        } else {
            text.replace(['\n', '\r'], " ").into()
        }
    }
}

impl EntityInputHandler for InputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.text[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range.as_ref().map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let new_text = self.sanitize(new_text);
        self.text = (self.text[0..range.start].to_owned() + &new_text + &self.text[range.end..]).into();
        let cursor = range.start + new_text.len();
        self.selected_range = cursor..cursor;
        self.selection_reversed = false;
        self.marked_range.take();
        self.changed(window, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let new_text = self.sanitize(new_text);
        self.text = (self.text[0..range.start].to_owned() + &new_text + &self.text[range.end..]).into();
        self.marked_range = (!new_text.is_empty()).then(|| range.start..range.start + new_text.len());
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.start)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());
        self.changed(window, cx);
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let layout = self.layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        let start = layout.position_for_offset(self.to_display(range.start));
        let end = layout.position_for_offset(self.to_display(range.end));
        Some(Bounds::from_corners(
            layout.origin + start,
            layout.origin + point(end.x, end.y + layout.line_height),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let layout = self.layout.as_ref()?;
        let offset = self.from_display(layout.offset_for_position(point - layout.origin));
        Some(self.offset_to_utf16(offset))
    }
}

/// Shaped text from the last frame, used for hit testing and IME.
struct InputLayout {
    lines: Vec<WrappedLine>,
    line_starts: Vec<usize>,
    line_tops: Vec<Pixels>,
    line_height: Pixels,
    /// Window position of the first glyph (text area origin minus scroll).
    origin: Point<Pixels>,
}

impl InputLayout {
    fn total_height(&self) -> Pixels {
        match (self.line_tops.last(), self.lines.last()) {
            (Some(top), Some(line)) => *top + line.size(self.line_height).height,
            _ => self.line_height,
        }
    }

    fn line_for_offset(&self, offset: usize) -> usize {
        self.line_starts.iter().rposition(|start| *start <= offset).unwrap_or(0)
    }

    /// Position (relative to `origin`) of the caret before `offset`.
    fn position_for_offset(&self, offset: usize) -> Point<Pixels> {
        if self.lines.is_empty() {
            return point(px(0.), px(0.));
        }
        let ix = self.line_for_offset(offset);
        let local = (offset - self.line_starts[ix]).min(self.lines[ix].len());
        let mut position = self.lines[ix].position_for_index(local, self.line_height).unwrap_or_default();
        position.y += self.line_tops[ix];
        position
    }

    /// Display offset closest to `position` (relative to `origin`).
    fn offset_for_position(&self, position: Point<Pixels>) -> usize {
        if self.lines.is_empty() {
            return 0;
        }
        let ix = self.line_tops.iter().rposition(|top| *top <= position.y).unwrap_or(0);
        let line = &self.lines[ix];
        let height = line.size(self.line_height).height;
        let y = (position.y - self.line_tops[ix]).clamp(px(0.), height - px(1.));
        let local = match line.closest_index_for_position(point(position.x.max(px(0.)), y), self.line_height) {
            Ok(ix) | Err(ix) => ix,
        };
        self.line_starts[ix] + local
    }

    /// Selection rectangles (relative to `origin`) for a display range.
    fn selection_rects(&self, range: Range<usize>, row_width: Pixels) -> Vec<Bounds<Pixels>> {
        let mut rects = Vec::new();
        let lh = self.line_height;
        for (ix, line) in self.lines.iter().enumerate() {
            let (line_start, line_end) = (self.line_starts[ix], self.line_starts[ix] + line.len());
            let (start, end) = (range.start.max(line_start), range.end.min(line_end));
            let continues = range.end > line_end;
            if start > end || (start == end && !(continues && range.start <= line_end)) {
                continue;
            }
            let top = self.line_tops[ix];
            let ps = line.position_for_index(start - line_start, lh).unwrap_or_default();
            let pe = line.position_for_index(end - line_start, lh).unwrap_or_default();
            let mut y = ps.y;
            while y <= pe.y {
                let x0 = if y == ps.y { ps.x } else { px(0.) };
                let x1 = if y == pe.y {
                    if continues { pe.x + px(6.) } else { pe.x }
                } else {
                    row_width
                };
                rects.push(Bounds::from_corners(point(x0, top + y), point(x1, top + y + lh)));
                y += lh;
            }
        }
        rects
    }
}

/// The element that shapes, paints and hit-tests the text.
struct TextElement {
    state: Entity<InputState>,
    placeholder_color: Hsla,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct TextPrepaint {
    lines: Vec<WrappedLine>,
    line_starts: Vec<usize>,
    line_tops: Vec<Pixels>,
    line_height: Pixels,
    origin: Point<Pixels>,
    selection: Vec<Bounds<Pixels>>,
    cursor: Option<Bounds<Pixels>>,
}

fn shape(
    text: SharedString,
    color: Hsla,
    marked: Option<Range<usize>>,
    wrap_width: Option<Pixels>,
    window: &mut Window,
) -> (Vec<WrappedLine>, Pixels) {
    let style = window.text_style();
    let font_size = style.font_size.to_pixels(window.rem_size());
    let run = TextRun {
        len: text.len(),
        font: style.font(),
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let runs = match marked {
        Some(marked) if marked.end <= text.len() => vec![
            TextRun { len: marked.start, ..run.clone() },
            TextRun {
                len: marked.end - marked.start,
                underline: Some(UnderlineStyle { color: Some(color), thickness: px(1.), wavy: false }),
                ..run.clone()
            },
            TextRun { len: text.len() - marked.end, ..run },
        ],
        _ => vec![run],
    };
    let lines = window
        .text_system()
        .shape_text(text, font_size, &runs, wrap_width, None)
        .map(|lines| lines.into_iter().collect())
        .unwrap_or_default();
    (lines, window.line_height())
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = TextPrepaint;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        let input = self.state.read(cx);
        if !input.multiline {
            style.size.height = window.line_height().into();
            return (window.request_layout(style, [], cx), ());
        }

        // Multi-line: height depends on wrapping at the final width.
        let text = if input.text.is_empty() { input.placeholder.clone() } else { input.display_text() };
        let text_style = window.text_style();
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();
        let font = text_style.font();
        let layout_id = window.request_measured_layout(style, move |known, available, window, _cx| {
            let width = known.width.or(match available.width {
                gpui::AvailableSpace::Definite(width) => Some(width),
                _ => None,
            });
            let run = TextRun {
                len: text.len(),
                font: font.clone(),
                color: gpui::black(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let height = window
                .text_system()
                .shape_text(text.clone(), font_size, &[run], width, None)
                .map(|lines| lines.iter().map(|line| line.size(line_height).height).fold(px(0.), |a, b| a + b))
                .unwrap_or(line_height)
                .max(line_height);
            size(width.unwrap_or(px(0.)), height)
        });
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.state.read(cx);
        let is_placeholder = input.text.is_empty();
        let (text, color) = if is_placeholder {
            (input.placeholder.clone(), self.placeholder_color)
        } else {
            (input.display_text(), window.text_style().color)
        };
        let marked = input.marked_range.clone().filter(|_| !is_placeholder && !input.masked);
        let wrap_width = input.multiline.then_some(bounds.size.width);
        let selection = input.to_display(input.selected_range.start)..input.to_display(input.selected_range.end);
        let cursor_offset = input.to_display(input.cursor_offset());
        let multiline = input.multiline;
        let mut scroll_x = input.scroll_x;

        let (lines, line_height) = shape(text, color, marked, wrap_width, window);
        let mut line_starts = Vec::with_capacity(lines.len());
        let mut line_tops = Vec::with_capacity(lines.len());
        let (mut start, mut top) = (0, px(0.));
        for line in &lines {
            line_starts.push(start);
            line_tops.push(top);
            start += line.len() + 1;
            top += line.size(line_height).height;
        }

        let mut layout = InputLayout { lines, line_starts, line_tops, line_height, origin: bounds.origin };
        let cursor_position = if is_placeholder { point(px(0.), px(0.)) } else { layout.position_for_offset(cursor_offset) };

        // Keep the caret in view for single-line inputs.
        if !multiline {
            let width = bounds.size.width;
            if cursor_position.x - scroll_x > width - px(2.) {
                scroll_x = cursor_position.x - width + px(2.);
            } else if cursor_position.x < scroll_x {
                scroll_x = cursor_position.x;
            }
            let content_width = layout.lines.first().map(|l| l.width()).unwrap_or_default();
            scroll_x = scroll_x.min((content_width - width + px(2.)).max(px(0.))).max(px(0.));
            layout.origin.x -= scroll_x;
        }
        self.state.update(cx, |input, _| input.scroll_x = scroll_x);

        let selection_rects = if selection.is_empty() || is_placeholder {
            Vec::new()
        } else {
            layout
                .selection_rects(selection, bounds.size.width)
                .into_iter()
                .map(|rect| rect + layout.origin)
                .collect()
        };
        let cursor = Some(Bounds::new(layout.origin + cursor_position, size(px(1.5), line_height)));

        TextPrepaint {
            lines: layout.lines,
            line_starts: layout.line_starts,
            line_tops: layout.line_tops,
            line_height,
            origin: layout.origin,
            selection: selection_rects,
            cursor,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (focus_handle, selection_empty) = {
            let input = self.state.read(cx);
            (input.focus_handle.clone(), input.selected_range.is_empty())
        };
        window.handle_input(&focus_handle, ElementInputHandler::new(bounds, self.state.clone()), cx);

        let theme = cx.theme();
        let selection_color = theme.primary.opacity(0.25);
        let cursor_color = theme.foreground;
        // Let the caret sit just past the right edge without being clipped.
        let mask = Bounds::from_corners(bounds.origin, bounds.bottom_right() + point(px(2.), px(0.)));
        window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
            for rect in prepaint.selection.drain(..) {
                window.paint_quad(fill(rect, selection_color));
            }
            for (ix, line) in prepaint.lines.iter().enumerate() {
                let origin = prepaint.origin + point(px(0.), prepaint.line_tops[ix]);
                line.paint(origin, prepaint.line_height, TextAlign::Left, None, window, cx).ok();
            }
            if focus_handle.is_focused(window) && selection_empty && let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(fill(cursor, cursor_color));
            }
        });

        let layout = InputLayout {
            lines: std::mem::take(&mut prepaint.lines),
            line_starts: std::mem::take(&mut prepaint.line_starts),
            line_tops: std::mem::take(&mut prepaint.line_tops),
            line_height: prepaint.line_height,
            origin: prepaint.origin,
        };
        self.state.update(cx, |input, _| input.layout = Some(layout));
    }
}

enum InputSource {
    Id(ElementId),
    State(Entity<InputState>),
}

/// A text input. See the module docs for usage.
#[derive(IntoElement)]
pub struct Input {
    source: InputSource,
    placeholder: Option<SharedString>,
    default_value: Option<SharedString>,
    value: Option<SharedString>,
    masked: bool,
    disabled: bool,
    invalid: bool,
    multiline: bool,
    autofocus: bool,
    appearance: bool,
    prefix: Vec<AnyElement>,
    suffix: Vec<AnyElement>,
    on_change: Option<TextCallback>,
    on_submit: Option<TextCallback>,
    style: StyleRefinement,
}

crate::impl_styled!(Input);

impl Input {
    /// An input that manages its own state, keyed by `id`.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self::with_source(InputSource::Id(id.into()))
    }

    /// An input backed by state you own.
    pub fn from_state(state: &Entity<InputState>) -> Self {
        Self::with_source(InputSource::State(state.clone()))
    }

    fn with_source(source: InputSource) -> Self {
        Self {
            source,
            placeholder: None,
            default_value: None,
            value: None,
            masked: false,
            disabled: false,
            invalid: false,
            multiline: false,
            autofocus: false,
            appearance: true,
            prefix: Vec::new(),
            suffix: Vec::new(),
            on_change: None,
            on_submit: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Initial text for self-managed inputs.
    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Controlled text: the input is reset to this whenever it differs.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// `type="password"`
    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// `aria-invalid` — red border and ring.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Multi-line, wrapping text (used by `Textarea`).
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    /// Focus the input the first time it is rendered.
    pub fn autofocus(mut self) -> Self {
        self.autofocus = true;
        self
    }

    /// Without border, background, padding and focus ring (for `InputGroup`).
    pub fn unstyled(mut self) -> Self {
        self.appearance = false;
        self
    }

    /// Element shown before the text, inside the border.
    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix.push(prefix.into_any_element());
        self
    }

    /// Element shown after the text, inside the border.
    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix.push(suffix.into_any_element());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Enter in a single-line input.
    pub fn on_submit(mut self, handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
    }

    /// Point this input at already-resolved state (keeps all other options).
    pub fn with_state(mut self, state: &Entity<InputState>) -> Self {
        self.source = InputSource::State(state.clone());
        self
    }

    /// Resolve (and create, for self-managed inputs) the backing state.
    pub fn state(&self, window: &mut Window, cx: &mut App) -> Entity<InputState> {
        match &self.source {
            InputSource::State(state) => state.clone(),
            InputSource::Id(id) => {
                let default = self.default_value.clone();
                let autofocus = self.autofocus;
                window.use_keyed_state(id.clone(), cx, move |window, cx| {
                    let mut state = InputState::new(window, cx);
                    if let Some(default) = default {
                        state = state.default_value(default);
                    }
                    if autofocus {
                        window.focus(&state.focus_handle);
                    }
                    state
                })
            }
        }
    }
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state(window, cx);
        state.update(cx, |s, _| {
            if let Some(placeholder) = self.placeholder.clone() {
                s.placeholder = placeholder;
            }
            if let Some(value) = self.value.clone()
                && value != s.text
            {
                s.set_text_silently(value);
            }
            s.masked = self.masked;
            s.disabled = self.disabled;
            s.multiline = self.multiline;
            s.on_change = self.on_change.clone();
            s.on_submit = self.on_submit.clone();
        });
        let focus_handle = state.read(cx).focus_handle.clone();
        let focused = focus_handle.is_focused(window);
        let theme = cx.theme();
        let (multiline, disabled, invalid) = (self.multiline, self.disabled, self.invalid);

        macro_rules! listen {
            ($el:expr, $method:ident) => {{
                let state = state.clone();
                $el.on_action(move |action, window, cx| state.update(cx, |s, cx| s.$method(action, window, cx)))
            }};
        }

        let mut el = div()
            .id(ElementId::NamedInteger("input".into(), state.entity_id().as_u64()))
            .key_context(KEY_CONTEXT)
            .flex()
            .gap_2()
            .w_full()
            .min_w_0()
            .text_sm()
            .text_color(theme.foreground)
            .map(|this| if multiline { this.items_start() } else { this.items_center() })
            .when(self.appearance, |this| {
                this.rounded(theme.radius_md())
                    .border_1()
                    .border_color(theme.input)
                    .when(theme.is_dark(), |this| this.bg(theme.input.opacity(0.3)))
                    .px_3()
                    .map(|this| if multiline { this.min_h_16().py_2() } else { this.h_9().py_1() })
                    .when(theme.shadows, |this| this.shadow_xs())
                    .when(focused && !invalid, |this| {
                        this.border_color(theme.ring).shadow(vec![ring_shadow(theme.ring.opacity(0.5), px(3.))])
                    })
                    .when(invalid, |this| {
                        this.border_color(theme.destructive)
                            .when(focused, |this| this.shadow(vec![ring_shadow(theme.destructive.opacity(0.2), px(3.))]))
                    })
            })
            .when(disabled, |this| this.opacity(0.5).cursor_not_allowed())
            .refine_style(&self.style);

        if !disabled {
            el = el.track_focus(&focus_handle).cursor(CursorStyle::IBeam);
            el = listen!(el, backspace);
            el = listen!(el, delete);
            el = listen!(el, delete_word_left);
            el = listen!(el, left);
            el = listen!(el, right);
            el = listen!(el, up);
            el = listen!(el, down);
            el = listen!(el, word_left);
            el = listen!(el, word_right);
            el = listen!(el, select_left);
            el = listen!(el, select_right);
            el = listen!(el, select_up);
            el = listen!(el, select_down);
            el = listen!(el, select_word_left);
            el = listen!(el, select_word_right);
            el = listen!(el, select_all);
            el = listen!(el, home);
            el = listen!(el, end);
            el = listen!(el, select_home);
            el = listen!(el, select_end);
            el = listen!(el, enter);
            el = listen!(el, paste);
            el = listen!(el, copy);
            el = listen!(el, cut);
            el = listen!(el, show_character_palette);
            let (down, up, up_out, moved) = (state.clone(), state.clone(), state.clone(), state.clone());
            el = el
                .on_mouse_down(MouseButton::Left, move |e, w, cx| down.update(cx, |s, cx| s.on_mouse_down(e, w, cx)))
                .on_mouse_up(MouseButton::Left, move |e, w, cx| up.update(cx, |s, cx| s.on_mouse_up(e, w, cx)))
                .on_mouse_up_out(MouseButton::Left, move |e, w, cx| up_out.update(cx, |s, cx| s.on_mouse_up(e, w, cx)))
                .on_mouse_move(move |e, w, cx| moved.update(cx, |s, cx| s.on_mouse_move(e, w, cx)));
        }

        el.children(self.prefix)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .child(TextElement { state: state.clone(), placeholder_color: theme.muted_foreground }),
            )
            .children(self.suffix)
    }
}

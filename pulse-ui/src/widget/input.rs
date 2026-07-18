#[cfg(target_os = "macos")]
mod platform {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    pub fn is_backspace(event: &KeyEvent) -> bool {
        matches!(event.code, KeyCode::Backspace)
            || matches!(
                event,
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers,
                    ..
                } if modifiers.contains(KeyModifiers::CONTROL)
            )
    }

    pub fn is_delete(event: &KeyEvent) -> bool {
        matches!(event.code, KeyCode::Delete)
            || matches!(
                event,
                KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers,
                    ..
                } if modifiers.contains(KeyModifiers::CONTROL)
            )
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    pub fn is_backspace(event: &KeyEvent) -> bool {
        matches!(event.code, KeyCode::Backspace)
            || matches!(
                event,
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers,
                    ..
                } if modifiers.contains(KeyModifiers::CONTROL)
            )
    }

    pub fn is_delete(event: &KeyEvent) -> bool {
        matches!(event.code, KeyCode::Delete)
    }
}

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::widget::Widget;

#[derive(Debug, Clone)]
pub struct InputState {
    pub text: String,
    pub cursor: usize,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
        }
    }
}

pub struct Input<'a>(pub &'a str, pub &'a InputState);

impl<'a> Widget for Input<'a> {
    fn render(&self, scope: &mut crate::render::RenderScope) {
        let mut text = self.1.text.clone();
        if self.1.cursor == text.len() {
            text.insert_str(self.1.cursor, "\x1b[47m \x1b[0m");
        } else {
            text.insert_str(self.1.cursor + 1, "\x1b[0m");
            text.insert_str(self.1.cursor, "\x1b[47m\x1b[30m");
        }
        scope.draw_text(0, format!("{}{text}", self.0));
    }
}

impl InputState {
    pub fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(key) if platform::is_backspace(key) => {
                if self.cursor > 0 {
                    let prev = self.text[..self.cursor]
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap();

                    self.text.remove(prev);
                    self.cursor = prev;
                }
                true
            }

            Event::Key(key) if platform::is_delete(key) => {
                if self.cursor < self.text.len() {
                    let prev = self.text[..self.cursor + 1]
                        .char_indices()
                        .last()
                        .map(|(i, _)| i)
                        .unwrap();

                    self.text.remove(prev);
                    self.cursor = prev;
                }
                true
            }

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                true
            }

            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                if self.cursor < self.text.len() {
                    self.cursor += 1;
                }
                true
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            }) => {
                if !modifiers.contains(KeyModifiers::CONTROL)
                    && !modifiers.contains(KeyModifiers::ALT)
                {
                    self.text.insert(self.cursor, *c);
                    self.cursor += c.len_utf8();
                }
                true
            }

            Event::Paste(content) => {
                self.text.insert_str(self.cursor, content);
                self.cursor += content.len();
                true
            }

            _ => false,
        }
    }
}

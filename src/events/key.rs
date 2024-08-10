use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Key {
    Char(char),
    Down,
    Up,
    Right,
    Left,
    Enter,
    Esc,
    Backspace,
    Ctrl(char),
    None,
}

impl From<KeyEvent> for Key {
    fn from(key: KeyEvent) -> Self {
        if key.modifiers == KeyModifiers::CONTROL {
            return match key.code {
                KeyCode::Char(c) => Key::Ctrl(c),
                _ => Key::None,
            };
        }

        match key.code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Enter => Key::Enter,
            KeyCode::Down => Key::Down,
            KeyCode::Up => Key::Up,
            KeyCode::Right => Key::Right,
            KeyCode::Left => Key::Left,
            KeyCode::Esc => Key::Esc,
            KeyCode::Backspace => Key::Backspace,
            _ => Key::None,
        }
    }
}

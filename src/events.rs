//! Editor events
pub(crate) mod handler;
pub(crate) mod input;
pub(crate) mod key;
pub(crate) mod mouse;
pub mod register;

use key::KeyEvent;
use mouse::MouseEvent;
use ratatui::crossterm::event::Event as CTEvent;

pub(crate) enum EditorEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    None,
}

impl From<CTEvent> for EditorEvent {
    fn from(event: CTEvent) -> Self {
        match event {
            CTEvent::Key(key_event) => Self::Key(KeyEvent::from(key_event)),
            CTEvent::Mouse(mouse_event) => Self::Mouse(MouseEvent::from(mouse_event)),
            _ => Self::None,
        }
    }
}

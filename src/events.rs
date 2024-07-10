//! Handles events
pub mod handler;
pub mod key;
pub(crate) mod mouse;
pub mod register;

use key::Key;
use mouse::MouseEvent;
use ratatui::crossterm::event::Event as CTEvent;

pub(crate) enum EditorEvent {
    Key(Key),
    Mouse(MouseEvent),
    None,
}

impl From<CTEvent> for EditorEvent {
    fn from(event: CTEvent) -> Self {
        match event {
            CTEvent::Key(key_event) => Self::Key(Key::from(key_event)),
            CTEvent::Mouse(mouse_event) => Self::Mouse(MouseEvent::from(mouse_event)),
            _ => Self::None,
        }
    }
}

use ratatui::crossterm::event::Event as CTEvent;

use crate::EditorState;

use super::input::EditorInput;
use super::mouse::EditorMouse;
use super::EditorEvent;

/// Handles key and mouse events.
#[derive(Clone, Debug, Default)]
pub struct EditorHandler {
    input: EditorInput,
}

impl EditorHandler {
    /// Creates a new `EditorHandler`.
    #[must_use]
    pub fn new(input: EditorInput) -> Self {
        Self { input }
    }
}

impl EditorHandler {
    pub fn on_event(&mut self, event: CTEvent, state: &mut EditorState) {
        let event = event.into();

        match event {
            EditorEvent::Key(event) => self.input.on_event(event, state),
            EditorEvent::Mouse(event) => EditorMouse::on_event(event, state),
            EditorEvent::None => {}
        };
    }
}

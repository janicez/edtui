use ratatui::crossterm::event::{MouseEvent as CTMouseEvent, MouseEventKind};
/// Represents a mouse event.
///
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum MouseEvent {
    /// A mouse press event.
    Down(MousePosition),

    /// A mouse release event.
    Up(MousePosition),

    /// A mouse Drag event.
    Drag(MousePosition),

    /// A mouse event that is handled by the editor.
    None,
}

impl From<CTMouseEvent> for MouseEvent {
    fn from(event: CTMouseEvent) -> Self {
        match event.kind {
            MouseEventKind::Down(_) => Self::Down(MousePosition::new(event.row, event.column)),
            MouseEventKind::Up(_) => Self::Up(MousePosition::new(event.row, event.column)),
            MouseEventKind::Drag(_) => Self::Drag(MousePosition::new(event.row, event.column)),
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct MousePosition {
    /// The row that the event occurred on.
    pub(crate) row: usize,
    /// The column that the event occurred on.
    pub(crate) col: usize,
}

impl MousePosition {
    /// Creates a new `MousePosition` instance.
    fn new(row: u16, col: u16) -> Self {
        Self {
            row: row.into(),
            col: col.into(),
        }
    }
}

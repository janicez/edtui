use std::collections::HashMap;

use crate::actions::search::StartSearch;
use crate::actions::{
    Action, Append, AppendCharToSearch, AppendNewline, Composed, CopySelection, DeleteChar,
    DeleteLine, DeleteSelection, Execute, FindNext, FindPrevious, InsertChar, InsertNewline,
    LineBreak, MoveBackward, MoveDown, MoveForward, MoveToEnd, MoveToFirst, MoveToStart, MoveUp,
    MoveWordBackward, MoveWordForward, Paste, Redo, RemoveChar, RemoveCharFromSearch,
    SelectBetween, SelectLine, StopSearch, SwitchMode, TriggerSearch, Undo,
};
use crate::{EditorMode, EditorState};

use super::key::KeyEvent;
use super::register::RegisterKey;

/// Registers a sequence of `KeyEvents` and maps them to an action.
#[derive(Clone, Debug)]
pub struct EditorInput {
    pub(crate) lookup: Vec<KeyEvent>,
    pub(crate) register: HashMap<RegisterKey, Action>,
}

impl Default for EditorInput {
    #[allow(clippy::too_many_lines)]
    fn default() -> Self {
        let mut register = HashMap::new();

        // Go into normal mode
        register.insert(
            RegisterKey::i(vec![KeyEvent::Esc]),
            SwitchMode(EditorMode::Normal).into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Esc]),
            SwitchMode(EditorMode::Normal).into(),
        );

        // Go into insert mode
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('i')]),
            SwitchMode(EditorMode::Insert).into(),
        );

        // Go into visual mode
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('v')]),
            SwitchMode(EditorMode::Visual).into(),
        );

        // Goes into search mode and starts of a new search.
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('/')]),
            StartSearch.into(),
        );
        // Trigger initial search
        register.insert(RegisterKey::s(vec![KeyEvent::Enter]), TriggerSearch.into());
        // Find next
        register.insert(RegisterKey::n(vec![KeyEvent::Char('n')]), FindNext.into());
        // Find previous
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('N')]),
            FindPrevious.into(),
        );
        // Clear search
        register.insert(RegisterKey::s(vec![KeyEvent::Esc]), StopSearch.into());
        // Delete last character from search
        register.insert(
            RegisterKey::s(vec![KeyEvent::Backspace]),
            RemoveCharFromSearch.into(),
        );

        // Go into insert mode and move one char forward
        register.insert(RegisterKey::n(vec![KeyEvent::Char('a')]), Append.into());

        // Move cursor right
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('l')]),
            MoveForward(1).into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('l')]),
            MoveForward(1).into(),
        );
        register.insert(RegisterKey::n(vec![KeyEvent::Right]), MoveForward(1).into());
        register.insert(RegisterKey::v(vec![KeyEvent::Right]), MoveForward(1).into());
        register.insert(RegisterKey::i(vec![KeyEvent::Right]), MoveForward(1).into());

        // Move cursor left
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('h')]),
            MoveBackward(1).into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('h')]),
            MoveBackward(1).into(),
        );
        register.insert(RegisterKey::n(vec![KeyEvent::Left]), MoveBackward(1).into());
        register.insert(RegisterKey::v(vec![KeyEvent::Left]), MoveBackward(1).into());
        register.insert(RegisterKey::i(vec![KeyEvent::Left]), MoveBackward(1).into());

        // Move cursor up
        register.insert(RegisterKey::n(vec![KeyEvent::Char('k')]), MoveUp(1).into());
        register.insert(RegisterKey::v(vec![KeyEvent::Char('k')]), MoveUp(1).into());
        register.insert(RegisterKey::n(vec![KeyEvent::Up]), MoveUp(1).into());
        register.insert(RegisterKey::v(vec![KeyEvent::Up]), MoveUp(1).into());
        register.insert(RegisterKey::i(vec![KeyEvent::Up]), MoveUp(1).into());
        //
        // Move cursor down
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('j')]),
            MoveDown(1).into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('j')]),
            MoveDown(1).into(),
        );
        register.insert(RegisterKey::n(vec![KeyEvent::Down]), MoveDown(1).into());
        register.insert(RegisterKey::v(vec![KeyEvent::Down]), MoveDown(1).into());
        register.insert(RegisterKey::i(vec![KeyEvent::Down]), MoveDown(1).into());

        // Move one word forward/backward
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('w')]),
            MoveWordForward(1).into(),
        );
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('b')]),
            MoveWordBackward(1).into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('w')]),
            MoveWordForward(1).into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('b')]),
            MoveWordBackward(1).into(),
        );

        // Move cursor to start/first/last position
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('0')]),
            MoveToStart().into(),
        );
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('_')]),
            MoveToFirst().into(),
        );
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('$')]),
            MoveToEnd().into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('0')]),
            MoveToStart().into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('_')]),
            MoveToFirst().into(),
        );
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('$')]),
            MoveToEnd().into(),
        );

        // Move cursor to start/first/last position and enter insert mode
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('I')]),
            Composed::new(MoveToFirst())
                .chain(SwitchMode(EditorMode::Insert))
                .into(),
        );
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('A')]),
            Composed::new(MoveToEnd()).chain(Append).into(),
        );

        // Append/insert new line and switch into insert mode
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('o')]),
            Composed::new(AppendNewline(1))
                .chain(SwitchMode(EditorMode::Insert))
                .into(),
        );
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('O')]),
            Composed::new(InsertNewline(1))
                .chain(SwitchMode(EditorMode::Insert))
                .into(),
        );

        // Insert a line break
        register.insert(RegisterKey::i(vec![KeyEvent::Enter]), LineBreak(1).into());

        // Remove the current character
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('x')]),
            RemoveChar(1).into(),
        );

        // Delete the previous character
        register.insert(
            RegisterKey::i(vec![KeyEvent::Backspace]),
            DeleteChar(1).into(),
        );

        // Delete the current line
        register.insert(
            RegisterKey::n(vec![KeyEvent::Char('d'), KeyEvent::Char('d')]),
            DeleteLine(1).into(),
        );

        // Delete the current selection
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('d')]),
            DeleteSelection.into(),
        );

        // Select inner word between delimiters
        register.insert(
            RegisterKey::n(vec![
                KeyEvent::Char('c'),
                KeyEvent::Char('i'),
                KeyEvent::Char('w'),
            ]),
            SelectBetween('"').into(),
        );

        // Select  the line
        register.insert(RegisterKey::n(vec![KeyEvent::Char('V')]), SelectLine.into());

        // Undo
        register.insert(RegisterKey::n(vec![KeyEvent::Char('u')]), Undo.into());

        // Redo
        register.insert(RegisterKey::n(vec![KeyEvent::Ctrl('r')]), Redo.into());

        // Copy
        register.insert(
            RegisterKey::v(vec![KeyEvent::Char('y')]),
            CopySelection.into(),
        );

        // Paste
        register.insert(RegisterKey::n(vec![KeyEvent::Char('p')]), Paste.into());
        register.insert(RegisterKey::v(vec![KeyEvent::Char('p')]), Paste.into());

        Self::new(register)
    }
}

impl EditorInput {
    /// Creates a new `EditorInput`.
    #[must_use]
    pub fn new(register: HashMap<RegisterKey, Action>) -> Self {
        Self {
            lookup: Vec::new(),
            register,
        }
    }

    pub fn on_event<E>(&mut self, event: E, state: &mut EditorState)
    where
        E: Into<KeyEvent>,
    {
        let mode = state.mode;
        let event = event.into();

        match event {
            // Always insert characters in insert mode
            KeyEvent::Char(c) if mode == EditorMode::Insert => InsertChar(c).execute(state),
            // Always add characters to search in search mode
            KeyEvent::Char(c) if mode == EditorMode::Search => AppendCharToSearch(c).execute(state),
            // Else lookup an action from the register
            _ => {
                if let Some(mut action) = self.get(event, mode) {
                    action.execute(state);
                }
            }
        }
    }
}

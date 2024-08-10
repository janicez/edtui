use edtui::{EditorTheme, EditorView};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::AppContext;

pub struct Root<'a> {
    context: &'a mut AppContext,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a mut AppContext) -> Self {
        Self { context }
    }

    pub fn handle_events(self, key: KeyEvent) {
        let event_handler = &mut self.context.event_handler;
        let state = &mut self.context.state;

        match key.code {
            KeyCode::Enter => {}
            _ => event_handler.on_key_event(key, state),
        }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, _] = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);
        let theme = Theme::new();

        let state = &mut self.context.state;
        let editor = EditorView::new(state).theme(theme.editor);
        editor.render(top, buf)
    }
}

pub struct Theme<'a> {
    pub editor: EditorTheme<'a>,
}

impl<'a> Theme<'a> {
    pub fn new() -> Self {
        Self {
            editor: EditorTheme::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick),
                )
                .base(EditorTheme::default().base.bold())
                .hide_status_line(),
        }
    }
}

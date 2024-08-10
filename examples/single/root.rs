use edtui::{EditorTheme, EditorView};
use ratatui::crossterm::event::{Event, KeyCode};
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

    pub fn handle_events(self, event: Event) {
        let handler = &mut self.context.editor_handler;
        let state = &mut self.context.editor_state;

        match event {
            Event::Key(key) if key.code == KeyCode::Enter => {}
            _ => handler.on_event(event, state),
        }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, _] = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);
        let theme = Theme::new();

        let state = &mut self.context.editor_state;
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

use edtui::{EditorHandler, EditorState};
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};
use root::Root;
use std::error::Error;
use term::Term;
mod root;
mod term;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    App::run()
}

pub struct App {
    term: Term,
    context: AppContext,
    should_quit: bool,
}

pub struct AppContext {
    editor_state: EditorState,
    editor_handler: EditorHandler,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            editor_state: EditorState::default(),
            editor_handler: EditorHandler::default(),
        }
    }
}

impl App {
    pub fn new() -> Result<App> {
        Ok(App {
            term: Term::new()?,
            context: AppContext::new(),
            should_quit: false,
        })
    }

    fn draw(&mut self) -> Result<()> {
        let root = Root::new(&mut self.context);
        let _ = self.term.draw(|f| f.render_widget(root, f.size()));
        Ok(())
    }

    /// Handles incoming events.
    fn handle_events(&mut self) -> Result<()> {
        let root = Root::new(&mut self.context);

        let event = ratatui::crossterm::event::read()?;

        // If the ctrl 'c' key is pressed quit the application.
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                self.should_quit = true;
                return Ok(());
            }
        }

        // Delegate the event handling to the event handler instance.
        root.handle_events(event);

        // Return a successful result.
        Ok(())
    }

    pub fn run() -> Result<()> {
        let mut app = Self::new()?;
        while !app.should_quit {
            app.draw()?;
            app.handle_events()?;
        }
        Term::stop()?;
        Ok(())
    }
}

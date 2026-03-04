mod app;
mod ui;

use crossterm::{event, terminal};
use crossterm::event::{Event, KeyCode};
use ratatui::backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::{Terminal, prelude::Backend};

use crate::app::*;
use crate::ui::*;
use std::io;

fn main() -> io::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let settings = load_config();
    let mut app = App::new(settings);

    let res = run_app(&mut terminal, &mut app);

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, app)).unwrap();

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.mode {
                AppMode::Selection => match key.code {
                    KeyCode::Char('q') => {
                        app.mode = AppMode::Exit;
                    }
                    _ => {}
                }
                _ => {}
            }
        }
    }
}
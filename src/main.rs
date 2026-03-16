mod app;
mod ui;

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::{Terminal, prelude::Backend};
use ratatui_textarea::{Input, Key, TextArea};

use crate::app::*;
use crate::ui::*;
use core::panic;
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let settings = load_config();
    let mut app = App::new(settings);

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match result {
        Err(err) => panic!("{:?}", err),
        Ok(_) => Ok(()),
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, app)).unwrap();

        match app.mode {
            AppMode::Selection => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Release {
                        // Skip events that are not KeyEventKind::Press
                        continue;
                    } else {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.mode = AppMode::Exit;
                            }
                            KeyCode::Down => {
                                app.select_next();
                            }
                            KeyCode::Up => {
                                app.select_previous();
                            }
                            KeyCode::Enter => {
                                if app.list_state.selected().unwrap() == app.available_prompts.len()
                                {
                                    for prompt in &app.available_prompts {
                                        if prompt.status == PromptStatus::Selected {
                                            app.selected_prompts.push(prompt.prompt.clone());
                                        }
                                    }
                                    if !app.selected_prompts.is_empty() {
                                        app.current_prompt = Some(app.selected_prompts[0].clone());
                                        let mut text_area = TextArea::default();
                                        text_area.set_block(
                                            Block::default()
                                                .borders(Borders::ALL)
                                                .title("Type to enter entry"),
                                        );
                                        app.user_input = Some(text_area);
                                        app.mode = AppMode::Entry;
                                    }
                                } else {
                                    app.toggle_selected();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            AppMode::Entry => match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => app.mode = AppMode::Exit,
                Input {
                    key: Key::Enter, ..
                } => {
                    let prompt = app.current_prompt.clone().unwrap();
                    app.submit_prompt(&prompt);
                    if app.entries.len() == app.selected_prompts.len() {
                        app.mode = AppMode::WrapUp;
                    } else {
                        let index = app
                            .selected_prompts
                            .iter()
                            .position(|p| p == app.current_prompt.as_ref().unwrap());
                        app.current_prompt =
                            Some(app.selected_prompts[index.unwrap() + 1_usize].clone());
                    }
                }
                input => {
                    app.user_input.as_mut().unwrap().input(input);
                }
            },
            AppMode::WrapUp => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Release {
                        // Skip events that are not KeyEventKind::Press
                        continue;
                    } else {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.mode = AppMode::Exit;
                            }
                            _ => {}
                        }
                    }
                }
            }

            AppMode::Exit => {
                return Ok(true);
            }
        }
    }
}

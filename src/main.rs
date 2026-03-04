use ratatui::{DefaultTerminal, Frame, crossterm};
use std::io;

enum AppMode {
    Selection,
    Entry,
    Exit,
}

pub struct Config {
    prompts: Vec::<JournalPrompt>
}

pub struct JournalPrompt {
    name: String,
    prompt: String,
    user_input: Option<String>,
}

pub struct App {
    mode: AppMode,
    available_prompts: Vec::<JournalPrompt>,
    selected_prompts: Vec::<JournalPrompt>,
    current_prompt: Option<JournalPrompt>,
}

impl App {
    pub fn new(conf: Config) -> App {
        App {
            mode: AppMode::Selection,
            available_prompts: conf.prompts,
            selected_prompts: Vec::<JournalPrompt>::new(),
            current_prompt: None,
        }
    }
}

fn main() -> io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

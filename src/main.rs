use config::Config;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(PartialEq)]
enum AppMode {
    Selection,
    Entry,
    WrapUp,
    Exit,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    prompts: Vec<JournalPrompt>,
}

pub fn load_config() -> Settings {
    let config_path = dirs::home_dir()
        .expect("Couldn't find home directory")
        .join(".redox.json");

    if !(Path::new(config_path.to_str().unwrap()).exists()) {
        File::create_new(&config_path).unwrap();
    }
    Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct JournalPrompt {
    name: String,
    prompt: String,
    user_input: Option<String>,
}

pub struct App {
    mode: AppMode,
    available_prompts: Vec<JournalPrompt>,
    selected_prompts: Vec<JournalPrompt>,
    current_prompt: Option<JournalPrompt>,
}

impl App {
    pub fn new(settings: Settings) -> App {
        App {
            mode: AppMode::Selection,
            available_prompts: settings.prompts,
            selected_prompts: Vec::<JournalPrompt>::new(),
            current_prompt: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.mode != AppMode::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.mode = AppMode::Exit;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut s: String = "".to_string();
        for prompt in &self.available_prompts {
            s.push_str(&prompt.name);
            s.push_str("\n");
            s.push_str(&prompt.prompt);
            s.push_str("\n\n");
        }

        let title = Line::from(" Redox - A journaling app ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let text = Text::from(s).blue();

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let settings = load_config();
    let mut app = App::new(settings);
    ratatui::run(|terminal| app.run(terminal))
}

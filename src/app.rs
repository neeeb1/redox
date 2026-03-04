use config::Config;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph, Widget},
};
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::path::Path;

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
    pub name: String,
    pub prompt: String,
    pub user_input: Option<String>,
}

#[derive(PartialEq)]
pub enum AppMode {
    Selection,
    JournalEntry,
    WrapUp,
    Exit,
}

pub struct App {
    pub mode: AppMode,
    pub available_prompts: Vec<JournalPrompt>,
    pub selected_prompts: Vec<JournalPrompt>,
    pub current_prompt: Option<JournalPrompt>,
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
}

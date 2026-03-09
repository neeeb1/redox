use config::Config;
use ratatui::widgets::ListState;
use serde::Deserialize;
use std::fs::File;
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

#[derive(PartialEq, Deserialize, Debug, Clone)]
pub enum PromptStatus {
    Selected,
    Unselected,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JournalPrompt {
    pub name: String,
    pub prompt: String,
    pub user_input: Option<String>,
}

#[derive(Clone)]
pub struct SelectionItem {
    pub prompt: JournalPrompt,
    pub status: PromptStatus,
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
    pub available_prompts: Vec<SelectionItem>,
    pub selected_prompts: Vec<JournalPrompt>,
    pub current_prompt: Option<JournalPrompt>,
    pub list_state: ListState,
}

impl App {
    pub fn new(settings: Settings) -> App {
        let mut selection_prompts: Vec<SelectionItem> = Vec::new();
        for prompt in settings.prompts {
            let item = SelectionItem {
                prompt: prompt,
                status: PromptStatus::Unselected,
            };
            selection_prompts.push(item);
        }

        App {
            mode: AppMode::Selection,
            available_prompts: selection_prompts,
            selected_prompts: Vec::<JournalPrompt>::new(),
            current_prompt: None,
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn toggle_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            let status = &mut self.available_prompts[i].status;
            *status = match status {
                PromptStatus::Selected => PromptStatus::Unselected,
                PromptStatus::Unselected => PromptStatus::Selected,
            };
        }
    }
}

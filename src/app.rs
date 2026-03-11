use config::Config;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    prompts: Vec<JournalPrompt>,
}

pub fn load_config() -> Settings {
    let config_path = dirs::home_dir()
        .expect("Couldn't find home directory")
        .join(".redox.json");

    if !(Path::new(config_path.to_str().unwrap()).exists()) {
        File::create_new(&config_path).unwrap();

        let mut default_settings = Settings {
            prompts: Vec::<JournalPrompt>::new(),
        };
        default_settings.prompts.push(JournalPrompt { name: ("Today's Thoughts".to_string()), prompt: ("What are you thinking about today? Any recurring thoughts that you can't get out of your head?".to_string()) });
        default_settings.prompts.push(JournalPrompt {
            name: ("Graditude is Rad-itude".to_string()),
            prompt: ("What are you thankful for today? Try to name 2 or 3 things.".to_string()),
        });
        default_settings.prompts.push(JournalPrompt {
            name: ("My Media Diet".to_string()),
            prompt: ("What are you consuming lately? Games, movies, music, books... anything!"
                .to_string()),
        });
        default_settings.prompts.push(JournalPrompt { name: ("Today's Top Tasks".to_string()), prompt: ("Name 3 tasks that are essential for today.\nIf you did these, you've done the bare minimum for a successful day!".to_string()) });

        let config_json = serde_json::to_string(&default_settings).unwrap();
        std::fs::write(&config_path, config_json).unwrap();
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalPrompt {
    pub name: String,
    pub prompt: String,
}

#[derive(Clone)]
pub struct SelectionItem {
    pub prompt: JournalPrompt,
    pub status: PromptStatus,
}

#[derive(PartialEq)]
pub enum AppMode {
    Selection,
    Entry,
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

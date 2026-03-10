use crate::app::*;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, VerticalAlignment},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, Paragraph},
};
use ratatui_textarea::TextArea;

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Split screen into 3 sections
    // Top has length of 3, middle has a minimum length of 1, and bottom has length of 3
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Configure the Top widget (title block)
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        " Redox - A journaling TUI ",
        Style::default().fg(Color::Blue),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    match app.mode {
        AppMode::Selection => {
            // Configure the middle widget (list of available prompts)
            let mut list_items = Vec::<ListItem>::new();
            for item in &app.available_prompts {
                let selected: String;
                if item.status == PromptStatus::Unselected {
                    selected = "[ ] ".to_string();
                } else {
                    selected = "[X] ".to_string()
                }

                list_items.push(ListItem::new(Line::from(Span::styled(
                    format!("{} {}", selected, item.prompt.name),
                    Style::default().fg(Color::Blue),
                ))));
            }

            list_items.push(ListItem::new("Continue"));

            let list_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let list = List::new(list_items)
                .block(list_block)
                .highlight_symbol(">> ")
                .highlight_style(SELECTED_STYLE)
                .highlight_spacing(HighlightSpacing::Always);

            frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
        }
        AppMode::Entry => {
            let current_prompt = app.current_prompt.clone().unwrap();

            let entry_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
                ])
                .split(chunks[1]);

            let prompt_title = Span::styled(current_prompt.name, Style::default().fg(Color::Blue));

            let prompt_text =
                Span::styled(current_prompt.prompt, Style::default().fg(Color::LightBlue));

            let mut user_input = TextArea::default();

            frame.render_widget(prompt_title, entry_chunks[0]);
            frame.render_widget(prompt_text, entry_chunks[1]);
            frame.render_widget(&user_input, entry_chunks[2]);
        }
        _ => {}
    }

    // Configure the bottom widget (navigation/help hints)
    let current_nav_text = vec![
        match app.mode {
            AppMode::Selection => Span::styled("Pick a prompt", Style::default().fg(Color::Green)),
            _ => Span::styled("whoops, not ready yet", Style::default().fg(Color::Green)),
        }
        .to_owned(),
    ];

    let mode_footer =
        Paragraph::new(Line::from(current_nav_text)).block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.mode {
            AppMode::Selection => Span::styled(
                "(q) to quit | (up / down) to navigate | (enter) to select",
                Style::default().fg(Color::Yellow),
            ),
            AppMode::Entry => Span::styled(
                "Type to enter your entry | (ctrl+enter) to continue | (esc) to exit",
                Style::default().fg(Color::Yellow),
            ),
            _ => Span::styled(
                "(q) to quit | (up / down) to navigate | (enter) to select",
                Style::default().fg(Color::Yellow),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}

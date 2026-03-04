use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        " Redox - A journaling TUI ",
        Style::default().fg(Color::Blue),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();
    for prompt in &app.available_prompts {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", prompt.name),
            Style::default().fg(Color::Blue),
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);
}

/* impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut list_items = Vec::<ListItem>::new();
        for prompt in &self.available_prompts {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{}", prompt.name),
                Style::default().fg(Color::Blue),
            ))));
        }

        let title = Line::from(" Redox - A journaling app ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        List::new(list_items).block(block).render(area, buf);
    }
} */

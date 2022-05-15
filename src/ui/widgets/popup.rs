use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

pub struct PopupWdg<'a> {
    title: &'a str,
    contents: &'a Vec<String>,
    pub area: Rect,
    pub is_show: bool,
}

impl<'a> PopupWdg<'a> {
    pub fn new(contents: &'a Vec<String>) -> Self {
        Self {
            title: "Select database",
            contents: contents,
            area: Rect::default(),
            is_show: false,
        }
    }

    pub fn widget(&mut self, size: Rect) -> List<'a> {
        let block = Block::default()
            .title(self.title.to_string())
            .borders(Borders::ALL);
        self.area = self.centered_rect(60, 20, size);

        let table_names: Vec<_> = self
            .contents
            .iter()
            .map(|table_name| {
                ListItem::new(Spans::from(vec![Span::styled(
                    table_name,
                    Style::default(),
                )]))
            })
            .collect();

        let widget = List::new(table_names).block(block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        return widget;
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1];

        return layout;
    }
}

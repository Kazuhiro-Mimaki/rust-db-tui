use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
};

pub enum TableMode {
    Records,
    Columns,
}

pub struct TabWdg<'a> {
    pub titles: Vec<&'a str>,
    pub mode: TableMode,
}

impl<'a> TabWdg<'a> {
    pub fn new() -> Self {
        Self {
            titles: vec!["Records [0]", "Columns [1]"],
            mode: TableMode::Records,
        }
    }

    pub fn widget(&self) -> Tabs<'a> {
        let tab_titles = self
            .titles
            .iter()
            .map(|t| Spans::from(Span::styled(*t, Style::default())))
            .collect();

        let widget = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL))
            .select(self.mode())
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            );
        return widget;
    }

    pub fn mode(&self) -> usize {
        match self.mode {
            TableMode::Records => 0,
            TableMode::Columns => 1,
        }
    }
}

use tui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

pub struct SqlInputWdg<'a> {
    title: &'a str,
    pub input: String,
}

impl<'a> SqlInputWdg<'a> {
    pub fn new() -> Self {
        Self {
            title: "SQL [e: start editing] [esc: stop editing]",
            input: String::new(),
        }
    }

    pub fn widget(&self) -> Paragraph<'a> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.to_string())
            .style(Style::default());
        let widget = Paragraph::new(self.input.clone())
            .style(Style::default())
            .block(block);
        return widget;
    }
}

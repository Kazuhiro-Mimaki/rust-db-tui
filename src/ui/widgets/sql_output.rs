use sqlx::mysql::MySqlQueryResult;
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub struct SqlOutputWdg<'a> {
    title: &'a str,
    output: Vec<Spans<'a>>,
}

impl<'a> SqlOutputWdg<'a> {
    pub fn new() -> Self {
        Self {
            title: "Output",
            output: vec![Spans::from("")],
        }
    }

    pub fn widget(&self) -> Paragraph<'a> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.to_string())
            .style(Style::default());
        let widget = Paragraph::new(self.output.clone())
            .block(block)
            .wrap(Wrap { trim: true });

        return widget;
    }

    pub fn set_error_msg(&mut self, error: String) {
        self.output = vec![
            Spans::from(Span::from("Fail to execute")),
            Spans::from(Span::from(error.to_string())),
        ];
    }

    pub fn set_success_msg(&mut self, query_result: MySqlQueryResult) {
        self.output = vec![
            Spans::from(Span::from("Success to execute")),
            Spans::from(Span::from(format!(
                "{} {}",
                "rows_affected:",
                query_result.rows_affected().to_string()
            ))),
            Spans::from(Span::from(format!(
                "{} {}",
                "last_insert_id:",
                query_result.last_insert_id().to_string()
            ))),
        ];
    }
}

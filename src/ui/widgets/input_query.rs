use std::vec;

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{App, InputMode};

pub fn render_sql_input_wdg<B: Backend>(f: &mut Frame<'_, B>, area: Rect, app: &mut App) {
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("SQL [e: start editing] [esc: stop editing]")
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        });

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::White),
        })
        .block(input_block);

    f.render_widget(input, area);

    match app.input_mode {
        InputMode::Normal => {}

        InputMode::Editing => {
            f.set_cursor(
                // Put cursor past the end of the input text
                area.x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )
        }
    }
}

pub fn render_sql_output_wdg<B: Backend>(f: &mut Frame<'_, B>, area: Rect, app: &mut App) {
    let mut text = vec![Spans::from("")];

    if app.error != String::new() {
        text = vec![
            Spans::from(Span::from("Fail to execute")),
            Spans::from(Span::from(app.error.to_string())),
        ];
    }

    if app.output.rows_affected() != 0 && app.output.last_insert_id() != 0 {
        text = vec![
            Spans::from(Span::from("Success to execute")),
            Spans::from(Span::from(format!(
                "{} {}",
                "rows_affected:",
                app.output.rows_affected().to_string()
            ))),
            Spans::from(Span::from(format!(
                "{} {}",
                "last_insert_id:",
                app.output.last_insert_id().to_string()
            ))),
        ];
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Output")
        .style(Style::default());
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

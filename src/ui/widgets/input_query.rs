use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
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

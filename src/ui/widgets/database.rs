use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_database_wdg<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
    let db_block = Block::default().title("DB").borders(Borders::ALL);

    let db_wdg = Paragraph::new("sample database")
        .style(Style::default().fg(Color::White))
        .block(db_block);

    f.render_widget(db_wdg, area);
}

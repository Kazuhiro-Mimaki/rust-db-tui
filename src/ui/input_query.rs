use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

pub fn render_input_query_wdg<B: Backend>(f: &mut Frame<'_, B>, area: Rect) {
    let input_query_block = Block::default()
        .title("Input queries")
        .borders(Borders::ALL);

    f.render_widget(input_query_block, area);
}

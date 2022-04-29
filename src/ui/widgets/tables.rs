use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub fn render_table_list_wdg<B: Backend>(
    f: &mut Frame<'_, B>,
    area: Rect,
    tables: &Vec<String>,
    table_list_state: &mut ListState,
) {
    let tables_block = Block::default().title("Tables").borders(Borders::ALL);

    let table_names: Vec<_> = tables
        .iter()
        .map(|table_name| {
            ListItem::new(Spans::from(vec![Span::styled(
                table_name,
                Style::default(),
            )]))
        })
        .collect();

    let table_list_wdg = List::new(table_names).block(tables_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    f.render_stateful_widget(table_list_wdg, area, table_list_state);
}

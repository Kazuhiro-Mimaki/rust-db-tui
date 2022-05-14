use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render_popup_wdg<B: Backend>(f: &mut Frame<'_, B>, size: Rect, tables: &Vec<String>) {
    let popup_block = Block::default().title("Popup").borders(Borders::ALL);
    let area = centered_rect(60, 20, size);

    let table_names: Vec<_> = tables
        .iter()
        .map(|table_name| {
            ListItem::new(Spans::from(vec![Span::styled(
                table_name,
                Style::default(),
            )]))
        })
        .collect();

    let popup_wdg = List::new(table_names).block(popup_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(popup_wdg, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

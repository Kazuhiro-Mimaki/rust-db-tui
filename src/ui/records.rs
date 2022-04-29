use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::table::TableStruct;

pub fn render_table_records_wdg<B: Backend>(
    f: &mut Frame<'_, B>,
    area: Rect,
    table: &mut TableStruct,
) {
    let table_records_block = Block::default().title("Records").borders(Borders::ALL);

    let header_layout = Row::new(
        table.headers[table.visible_start_column_index..]
            .iter()
            .map(|h| {
                Cell::from(h.to_string()).style(Style::default().add_modifier(Modifier::BOLD))
            }),
    )
    .height(1)
    .bottom_margin(1);

    let record_layout = table.records.iter().enumerate().map(|(row_index, item)| {
        let cells = item[table.visible_start_column_index..]
            .iter()
            .enumerate()
            .map(|(column_idx, c)| {
                Cell::from(c.to_string()).style(
                    if column_idx == table.selected_column_index - table.visible_start_column_index
                        && Some(row_index) == table.row_list_state.selected()
                    {
                        Style::default().bg(Color::Blue)
                    } else {
                        Style::default()
                    },
                )
            });
        Row::new(cells).bottom_margin(1)
    });

    let record_list_wdg = Table::new(record_layout)
        .header(header_layout)
        .block(table_records_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ]);

    f.render_stateful_widget(record_list_wdg, area, &mut table.row_list_state);
}

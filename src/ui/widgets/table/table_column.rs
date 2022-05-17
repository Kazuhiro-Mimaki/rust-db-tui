use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

use crate::model::table::TableColumnModel;

use super::table::{SelectableRange, VisibleRange};

pub struct TableColumnWdg<'a> {
    pub title: &'a str,
    pub current_table: String,
    table_column_model: TableColumnModel,
    selectable_range: SelectableRange,
    visible_range: VisibleRange,
    pub selected_column_index: usize,
    pub select_row_list_state: TableState,
}

impl<'a> TableColumnWdg<'a> {
    pub fn new(current_table: String, table_column_model: TableColumnModel) -> Self {
        let selectable_range = SelectableRange {
            width: table_column_model.headers.len().saturating_sub(1),
            height: table_column_model.columns.len().saturating_sub(1),
        };
        let visible_range = VisibleRange {
            begin_column_index: 0,
            end_column_index: 9,
        };
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        Self {
            title: "Columns",
            current_table: current_table,
            table_column_model: table_column_model,
            selectable_range: selectable_range,
            visible_range: visible_range,
            selected_column_index: 0,
            select_row_list_state: default_state,
        }
    }

    pub fn widget(&self) -> Table<'a> {
        let block = Block::default()
            .title(self.title.to_string())
            .borders(Borders::ALL);

        let header_layout = Row::new(
            self.table_column_model.headers[self.visible_range.begin_column_index..]
                .iter()
                .map(|h| {
                    Cell::from(h.to_string()).style(Style::default().add_modifier(Modifier::BOLD))
                }),
        )
        .height(1)
        .bottom_margin(1);

        let record_layout =
            self.table_column_model
                .columns
                .iter()
                .enumerate()
                .map(|(row_index, item)| {
                    let cells = item[self.visible_range.begin_column_index..]
                        .iter()
                        .enumerate()
                        .map(|(column_idx, c)| {
                            Cell::from(c.to_string()).style(
                                if column_idx
                                    == self.selected_column_index
                                        - self.visible_range.begin_column_index
                                    && Some(row_index) == self.select_row_list_state.selected()
                                {
                                    Style::default().bg(Color::Blue)
                                } else {
                                    Style::default()
                                },
                            )
                        });
                    Row::new(cells).bottom_margin(1)
                });

        let widget = Table::new(record_layout)
            .header(header_layout)
            .block(block)
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
        return widget;
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = self.select_row_list_state.selected() {
            if selected != 0 {
                self.select_row_list_state.select(Some(selected - 1));
            };
        }
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = self.select_row_list_state.selected() {
            if self.selectable_range.height <= selected {
                return;
            }
            self.select_row_list_state.select(Some(selected + 1));
        }
    }

    pub fn move_right(&mut self) {
        if self.table_column_model.columns.is_empty() {
            return;
        }
        if self.selected_column_index >= self.selectable_range.width {
            return;
        }
        self.selected_column_index += 1;
    }

    pub fn move_left(&mut self) {
        if self.table_column_model.columns.is_empty() {
            return;
        }
        if self.selected_column_index == 0 {
            return;
        }
        self.selected_column_index -= 1;
    }
}

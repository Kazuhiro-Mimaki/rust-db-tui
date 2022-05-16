use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

pub struct TableRecordWdg<'a> {
    pub title: &'a str,
    pub current_table: String,
    pub headers: Vec<String>,
    pub records: Vec<Vec<String>>,
    pub selectable_width_range: usize,
    pub selectable_height_range: usize,
    pub selected_column_index: usize,
    pub select_row_list_state: TableState,
    pub visible_start_column_index: usize,
    pub visible_end_column_index: usize,
}

impl<'a> TableRecordWdg<'a> {
    pub fn new(current_table: String, headers: Vec<String>, records: Vec<Vec<String>>) -> Self {
        let selectable_width_range = headers.len().saturating_sub(1);
        let selectable_height_range = records.len().saturating_sub(1);
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        Self {
            title: "Records",
            current_table: current_table,
            headers: headers,
            records: records,
            selectable_width_range: selectable_width_range,
            selectable_height_range: selectable_height_range,
            selected_column_index: 0,
            select_row_list_state: default_state,
            visible_start_column_index: 0,
            visible_end_column_index: 9,
        }
    }

    pub fn widget(&self) -> Table<'a> {
        let block = Block::default()
            .title(self.title.to_string())
            .borders(Borders::ALL);

        let header_layout = Row::new(self.headers[self.visible_start_column_index..].iter().map(
            |h| Cell::from(h.to_string()).style(Style::default().add_modifier(Modifier::BOLD)),
        ))
        .height(1)
        .bottom_margin(1);

        let record_layout = self.records.iter().enumerate().map(|(row_index, item)| {
            let cells = item[self.visible_start_column_index..]
                .iter()
                .enumerate()
                .map(|(column_idx, c)| {
                    Cell::from(c.to_string()).style(
                        if column_idx
                            == self.selected_column_index - self.visible_start_column_index
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
            if self.selectable_height_range <= selected {
                return;
            }
            self.select_row_list_state.select(Some(selected + 1));
        }
    }

    pub fn move_right(&mut self) {
        if self.records.is_empty() {
            return;
        }
        if self.selected_column_index >= self.selectable_width_range {
            return;
        }
        self.selected_column_index += 1;
    }

    pub fn move_left(&mut self) {
        if self.records.is_empty() {
            return;
        }
        if self.selected_column_index == 0 {
            return;
        }
        self.selected_column_index -= 1;
    }

    pub fn scroll_right(&mut self) {
        self.visible_end_column_index = self.selected_column_index;
        self.visible_start_column_index = self.visible_end_column_index - 9;
    }

    pub fn scroll_left(&mut self) {
        self.visible_start_column_index = self.selected_column_index;
        self.visible_end_column_index = self.visible_start_column_index + 9;
    }

    pub fn update_visible_range(&mut self) {
        if self.selected_column_index > self.visible_end_column_index {
            self.scroll_right();
        } else if self.selected_column_index < self.visible_start_column_index {
            self.scroll_left();
        }
    }

    pub fn reset_default_records(
        &mut self,
        selected_table: String,
        headers: Vec<String>,
        records: Vec<Vec<String>>,
    ) {
        let selectable_width_range = headers.len().saturating_sub(1);
        let selectable_height_range = records.len().saturating_sub(1);
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        self.current_table = selected_table;
        self.headers = headers;
        self.records = records;
        self.selectable_width_range = selectable_width_range;
        self.selectable_height_range = selectable_height_range;
        self.selected_column_index = 0;
        self.select_row_list_state = default_state;
        self.visible_start_column_index = 0;
        self.visible_end_column_index = 9;
    }

    pub fn is_current_table(&self, selected_table: String) -> bool {
        return selected_table == self.current_table;
    }
}

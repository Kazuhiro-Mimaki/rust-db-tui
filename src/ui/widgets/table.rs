use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

pub struct TableWdg<'a> {
    pub title: &'a str,
    pub current_table: String,
    table_items: TableItems,
    selectable_range: SelectableRange,
    visible_range: VisibleRange,
    pub selected_column_index: usize,
    pub select_row_list_state: TableState,
}

struct TableItems {
    pub headers: Vec<String>,
    pub fields: Vec<Vec<String>>,
}

struct SelectableRange {
    pub width: usize,
    pub height: usize,
}

struct VisibleRange {
    pub begin_column_index: usize,
    pub end_column_index: usize,
}

impl<'a> TableWdg<'a> {
    fn new(current_table: String, table_items: TableItems) -> Self {
        let selectable_range = SelectableRange {
            width: table_items.headers.len().saturating_sub(1),
            height: table_items.fields.len().saturating_sub(1),
        };
        let visible_range = VisibleRange {
            begin_column_index: 0,
            end_column_index: 9,
        };
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        Self {
            title: "Records",
            current_table: current_table,
            table_items: table_items,
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

        let headers = Row::new(
            self.table_items.headers[self.visible_range.begin_column_index..]
                .iter()
                .map(|h| {
                    Cell::from(h.to_string()).style(Style::default().add_modifier(Modifier::BOLD))
                }),
        )
        .height(1)
        .bottom_margin(1);

        let fields = self
            .table_items
            .fields
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

        let widget = Table::new(fields)
            .header(headers)
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
        if self.table_items.fields.is_empty() {
            return;
        }
        if self.selected_column_index >= self.selectable_range.width {
            return;
        }
        self.selected_column_index += 1;
    }

    pub fn move_left(&mut self) {
        if self.table_items.fields.is_empty() {
            return;
        }
        if self.selected_column_index == 0 {
            return;
        }
        self.selected_column_index -= 1;
    }

    pub fn scroll_right(&mut self) {
        self.visible_range.end_column_index = self.selected_column_index;
        self.visible_range.begin_column_index = self.visible_range.end_column_index - 9;
    }

    pub fn scroll_left(&mut self) {
        self.visible_range.begin_column_index = self.selected_column_index;
        self.visible_range.end_column_index = self.visible_range.begin_column_index + 9;
    }

    pub fn update_visible_range(&mut self) {
        if self.selected_column_index > self.visible_range.end_column_index {
            self.scroll_right();
        } else if self.selected_column_index < self.visible_range.begin_column_index {
            self.scroll_left();
        }
    }

    pub fn reset_table(
        &mut self,
        selected_table: String,
        headers: Vec<String>,
        fields: Vec<Vec<String>>,
    ) {
        let selectable_width_range = headers.len().saturating_sub(1);
        let selectable_height_range = fields.len().saturating_sub(1);
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        self.current_table = selected_table;
        self.table_items.headers = headers;
        self.table_items.fields = fields;
        self.selectable_range.width = selectable_width_range;
        self.selectable_range.height = selectable_height_range;
        self.selected_column_index = 0;
        self.select_row_list_state = default_state;
        self.visible_range.begin_column_index = 0;
        self.visible_range.end_column_index = 9;
    }

    pub fn is_current_table(&self, selected_table: String) -> bool {
        return selected_table == self.current_table;
    }
}

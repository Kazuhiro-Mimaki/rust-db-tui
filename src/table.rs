use tui::widgets::TableState;

pub struct TableStruct {
    pub name: String,
    pub headers: Vec<String>,
    pub records: Vec<Vec<String>>,
    pub selectable_column_range: usize,
    pub selectable_row_range: usize,
    pub selected_column_index: usize,
    pub row_list_state: TableState,
    pub visible_start_column_index: usize,
    pub visible_end_column_index: usize,
}

impl TableStruct {
    pub fn new(name: String, headers: Vec<String>, records: Vec<Vec<String>>) -> Self {
        let selectable_column_range = headers.len().saturating_sub(1);
        let selectable_row_range = records.len().saturating_sub(1);
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        Self {
            name: name,
            headers: headers,
            records: records,
            selectable_column_range: selectable_column_range,
            selectable_row_range: selectable_row_range,
            selected_column_index: 0,
            row_list_state: default_state,
            visible_start_column_index: 0,
            visible_end_column_index: 9,
        }
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = self.row_list_state.selected() {
            if selected != 0 {
                self.row_list_state.select(Some(selected - 1));
            };
        }
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = self.row_list_state.selected() {
            if self.selectable_row_range <= selected {
                return;
            }
            self.row_list_state.select(Some(selected + 1));
        }
    }

    pub fn move_right(&mut self) {
        if self.records.is_empty() {
            return;
        }
        if self.selected_column_index >= self.selectable_column_range {
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

    pub async fn reset_default_records(
        &mut self,
        table_name: String,
        headers: Vec<String>,
        records: Vec<Vec<String>>,
    ) {
        let selectable_column_range = headers.len().saturating_sub(1);
        let selectable_row_range = records.len().saturating_sub(1);
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        self.name = table_name;
        self.headers = headers;
        self.records = records;
        self.selectable_column_range = selectable_column_range;
        self.selectable_row_range = selectable_row_range;
        self.selected_column_index = 0;
        self.row_list_state = default_state;
        self.visible_start_column_index = 0;
        self.visible_end_column_index = 9;
    }
}

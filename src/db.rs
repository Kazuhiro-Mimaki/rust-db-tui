use sqlx::{mysql::MySqlRow, Column, MySql, MySqlPool, Pool, Row};
use tui::widgets::TableState;

use crate::utils;

pub struct MySqlClient {
    pub pool: Pool<MySql>,
}

impl MySqlClient {
    pub async fn new(db_url: &str) -> Self {
        Self {
            pool: MySqlPool::connect(db_url).await.unwrap(),
        }
    }

    pub async fn get_table_list(&self, db_name: &str) -> Vec<MySqlRow> {
        let get_tables_query = format!("{} {}", "SHOW TABLE STATUS FROM", db_name);
        let table_list = sqlx::query(&get_tables_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();
        return table_list;
    }

    pub async fn get_record_list(&self, table_name: &str) -> Vec<MySqlRow> {
        let get_records_query = format!("{} {}", "SELECT * FROM", table_name);
        let record_list = sqlx::query(&get_records_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();
        return record_list;
    }
}

pub struct TableStruct {
    pub name: String,
    pub headers: Vec<String>,
    pub records: Vec<Vec<String>>,
    pub selectable_column_range: usize,
    pub selected_column_index: usize,
    pub row_list_state: TableState,
    pub visible_start_column_index: usize,
    pub visible_end_column_index: usize,
}

impl TableStruct {
    pub fn new(name: String, headers: Vec<String>, records: Vec<Vec<String>>) -> Self {
        let selectable_column_range = headers.clone().len() - 1;
        let mut default_state = TableState::default();
        default_state.select(Some(0));

        Self {
            name: name,
            headers: headers,
            records: records,
            selectable_column_range: selectable_column_range,
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
            self.row_list_state.select(Some(selected + 1));
        }
    }

    pub fn move_right(&mut self) {
        if self.records.is_empty() {
            return;
        }
        if self.selected_column_index >= self.headers.len().saturating_sub(1) {
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
}

pub fn parse_sql_records(table_rows: Vec<MySqlRow>) -> (Vec<String>, Vec<Vec<String>>) {
    let mut headers = vec![];
    let mut records = vec![];

    for table_row in table_rows.iter() {
        headers = table_row
            .columns()
            .iter()
            .map(|column| column.name().to_string())
            .collect();

        let mut record = vec![];
        for column in table_row.columns() {
            let column_name = column.name();
            record.push(utils::convert_column_value_to_string(
                &table_row,
                column_name,
            ));
        }
        records.push(record);
    }

    return (headers, records);
}

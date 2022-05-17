use crate::{model::table::TableModel, ui::widgets::tab::TableMode};

use super::{table_column::TableColumnWdg, table_record::TableRecordWdg};

pub struct TableWdg<'a> {
    table_mode: TableMode,
    pub record_widget: TableRecordWdg<'a>,
    pub column_widget: TableColumnWdg<'a>,
}

pub struct SelectableRange {
    pub width: usize,
    pub height: usize,
}

pub struct VisibleRange {
    pub begin_column_index: usize,
    pub end_column_index: usize,
}

pub trait TableWdgTrait {
    fn move_right();
    fn move_left();
}

impl<'a> TableWdg<'a> {
    pub fn new(record_widget: TableRecordWdg<'a>, column_widget: TableColumnWdg<'a>) -> Self {
        Self {
            table_mode: TableMode::Records,
            record_widget: record_widget,
            column_widget: column_widget,
        }
    }

    pub fn reset_table_widget(&mut self, selected_table: String, table_model: TableModel) {
        self.record_widget = TableRecordWdg::new(selected_table.clone(), table_model.record);
        self.column_widget = TableColumnWdg::new(selected_table.clone(), table_model.column);
    }

    pub fn move_up(&mut self) {
        match self.table_mode {
            TableMode::Records => {
                self.record_widget.move_up();
            }
            TableMode::Columns => {
                self.column_widget.move_up();
            }
        };
    }

    pub fn move_down(&mut self) {
        match self.table_mode {
            TableMode::Records => {
                self.record_widget.move_down();
            }
            TableMode::Columns => {
                self.column_widget.move_down();
            }
        };
    }

    pub fn move_right(&mut self) {
        match self.table_mode {
            TableMode::Records => {
                self.record_widget.move_right();
            }
            TableMode::Columns => {
                self.column_widget.move_right();
            }
        };
    }

    pub fn move_left(&mut self) {
        match self.table_mode {
            TableMode::Records => {
                self.record_widget.move_left();
            }
            TableMode::Columns => {
                self.column_widget.move_left();
            }
        };
    }
}

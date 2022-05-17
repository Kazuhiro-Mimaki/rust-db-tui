use crate::model::table::TableModel;

use super::{
    database::DatabaseWdg, sql_input::SqlInputWdg, sql_output::SqlOutputWdg, tab::TabWdg,
    table_list::TableListWdg, table::{table::TableWdg, table_record::TableRecordWdg, table_column::TableColumnWdg},
};

pub struct WidgetCtx<'a> {
    pub database: DatabaseWdg<'a>,
    pub table_list: TableListWdg<'a>,
    pub table: TableWdg<'a>,
    pub sql_input: SqlInputWdg<'a>,
    pub sql_output: SqlOutputWdg<'a>,
    pub tab: TabWdg<'a>,
}

impl<'a> WidgetCtx<'a> {
    pub fn new(databases: Vec<String>, tables: Vec<String>, table_model: TableModel) -> Self {
        let default_table_name = tables[0].clone();
        let table_record_widget =
            TableRecordWdg::new(default_table_name.clone(), table_model.record);
        let table_column_widget =
            TableColumnWdg::new(default_table_name.clone(), table_model.column);

        Self {
            database: DatabaseWdg::new(databases.clone()),
            table_list: TableListWdg::new(tables.clone()),
            table: TableWdg::new(table_record_widget, table_column_widget),
            sql_input: SqlInputWdg::new(),
            sql_output: SqlOutputWdg::new(),
            tab: TabWdg::new(),
        }
    }
}

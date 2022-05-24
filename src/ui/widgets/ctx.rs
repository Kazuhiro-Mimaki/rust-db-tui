use crate::model::database::DatabaseModel;

use super::{
    database::DatabaseWdg,
    sql_input::SqlInputWdg,
    sql_output::SqlOutputWdg,
    tab::TabWdg,
    table::{table::TableWdg, table_column::TableColumnWdg, table_record::TableRecordWdg},
    table_list::TableListWdg,
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
    pub fn new(db_model: DatabaseModel) -> Self {
        let default_table_name = db_model.current_table.name.clone();
        let table_record_widget =
            TableRecordWdg::new(default_table_name.clone(), db_model.current_table.record);
        let table_column_widget =
            TableColumnWdg::new(default_table_name.clone(), db_model.current_table.column);

        Self {
            database: DatabaseWdg::new(db_model.databases.clone()),
            table_list: TableListWdg::new(db_model.tables.clone()),
            table: TableWdg::new(table_record_widget, table_column_widget),
            sql_input: SqlInputWdg::new(),
            sql_output: SqlOutputWdg::new(),
            tab: TabWdg::new(),
        }
    }
}

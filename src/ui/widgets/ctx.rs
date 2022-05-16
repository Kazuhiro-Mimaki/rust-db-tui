use super::{
    database::DatabaseWdg, sql_input::SqlInputWdg, sql_output::SqlOutputWdg, tab::TabWdg,
    table_column::TableColumnWdg, table_list::TableListWdg, table_record::TableRecordWdg,
};

pub struct WidgetCtx<'a> {
    pub database: DatabaseWdg<'a>,
    pub table_list: TableListWdg<'a>,
    pub table_record: TableRecordWdg<'a>,
    pub table_column: TableColumnWdg<'a>,
    pub sql_input: SqlInputWdg<'a>,
    pub sql_output: SqlOutputWdg<'a>,
    pub tab: TabWdg<'a>,
}

impl<'a> WidgetCtx<'a> {
    pub fn new(
        databases: Vec<String>,
        tables: Vec<String>,
        record_headers: Vec<String>,
        record_items: Vec<Vec<String>>,
        column_headers: Vec<String>,
        column_items: Vec<Vec<String>>,
    ) -> Self {
        let default_table_name = tables[0].clone();

        Self {
            database: DatabaseWdg::new(databases.clone()),
            table_list: TableListWdg::new(tables.clone()),
            table_record: TableRecordWdg::new(
                default_table_name.to_string(),
                record_headers,
                record_items,
            ),
            table_column: TableColumnWdg::new(
                default_table_name.to_string(),
                column_headers,
                column_items,
            ),
            sql_input: SqlInputWdg::new(),
            sql_output: SqlOutputWdg::new(),
            tab: TabWdg::new(),
        }
    }
}

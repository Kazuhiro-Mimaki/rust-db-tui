use crate::db::sql_client::MySqlClient;

pub struct TableModel {
    pub name: String,
    pub record: TableRecordModel,
    pub column: TableColumnModel,
}

pub struct TableRecordModel {
    pub headers: Vec<String>,
    pub records: Vec<Vec<String>>,
}

pub struct TableColumnModel {
    pub headers: Vec<String>,
    pub columns: Vec<Vec<String>>,
}

impl TableModel {
    pub async fn new(mysql_client: &MySqlClient, table: String) -> Self {
        let (record_headers, record_fields) = mysql_client.get_table_records(table.clone()).await;
        let table_record = TableRecordModel {
            headers: record_headers,
            records: record_fields,
        };

        let (column_headers, column_fields) = mysql_client.get_table_columns(table.clone()).await;
        let table_column = TableColumnModel {
            headers: column_headers,
            columns: column_fields,
        };

        Self {
            name: table,
            record: table_record,
            column: table_column,
        }
    }
}

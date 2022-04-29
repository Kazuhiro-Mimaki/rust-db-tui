use sqlx::mysql::MySqlRow;
use sqlx::{Column, Row};

use crate::utils;

pub fn parse_sql_tables(rows: Vec<MySqlRow>) -> Vec<String> {
    let tables: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("Name"))
        .collect();
    return tables;
}

pub fn parse_sql_table_rows(table_rows: Vec<MySqlRow>) -> (Vec<String>, Vec<Vec<String>>) {
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

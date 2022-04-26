use sqlx::{mysql::MySqlRow, Row};

pub fn convert_column_value_to_string(row: &MySqlRow, column_name: &str) -> String {
    if let Ok(value) = row.try_get(column_name) {
        let value: String = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: &str = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: i8 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: i16 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: i32 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: i64 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: u8 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: u16 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: u32 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: u64 = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: bool = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: chrono::DateTime<chrono::Utc> = value;
        value.to_string()
    } else if let Ok(value) = row.try_get(column_name) {
        let value: serde_json::Value = value;
        value.to_string()
    } else {
        String::from("NULL")
    }
}

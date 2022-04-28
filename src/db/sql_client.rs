use sqlx::{MySql, MySqlPool, Pool};

use super::parser::{parse_sql_records, parse_sql_tables};

pub struct MySqlClient {
    pub pool: Pool<MySql>,
}

impl MySqlClient {
    pub async fn new(db_url: &str) -> Self {
        Self {
            pool: MySqlPool::connect(db_url).await.unwrap(),
        }
    }

    pub async fn get_table_list(&self, db_name: &str) -> Vec<String> {
        let get_tables_query = format!("{} {}", "SHOW TABLE STATUS FROM", db_name);
        let table_rows = sqlx::query(&get_tables_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return parse_sql_tables(table_rows);
    }

    pub async fn get_record_list(&self, table_name: &str) -> (Vec<String>, Vec<Vec<String>>) {
        let get_records_query = format!("{} {}", "SELECT * FROM", table_name);
        let record_rows = sqlx::query(&get_records_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return parse_sql_records(record_rows);
    }
}

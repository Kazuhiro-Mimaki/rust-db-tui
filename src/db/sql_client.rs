use std::env;

use sqlx::{mysql::MySqlQueryResult, MySql, MySqlPool, Pool};

use super::parser::{parse_sql_db, parse_sql_table_rows, parse_sql_tables};

pub struct MySqlClient {
    pub base_db_url: String,
    pub pool: Pool<MySql>,
}

impl MySqlClient {
    pub async fn new() -> Self {
        let base_db_url = env::var("DATABASE_URL").unwrap().to_string();

        Self {
            base_db_url: base_db_url.clone(),
            pool: MySqlPool::connect(&base_db_url).await.unwrap(),
        }
    }

    pub async fn reconnect(&mut self, new_database: String) {
        let new_db_url = format!("{}/{}", self.base_db_url, new_database);
        self.pool = MySqlPool::connect(&new_db_url).await.unwrap();
    }

    pub async fn get_database_list(&self) -> Vec<String> {
        let get_db_query = format!("{}", "SHOW DATABASES");
        let db_rows = sqlx::query(&get_db_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return parse_sql_db(db_rows);
    }

    pub async fn get_table_list(&self, db_name: String) -> Vec<String> {
        let get_tables_query = format!("{} {}", "SHOW TABLE STATUS FROM", db_name);
        let table_rows = sqlx::query(&get_tables_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return parse_sql_tables(table_rows);
    }

    pub async fn get_table_records(&self, table_name: String) -> (Vec<String>, Vec<Vec<String>>) {
        let get_records_query = format!("{} {}", "SELECT * FROM", table_name);
        let record_rows = sqlx::query(&get_records_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return parse_sql_table_rows(record_rows);
    }

    pub async fn get_table_columns(&self, table_name: String) -> (Vec<String>, Vec<Vec<String>>) {
        let get_records_query = format!("{} {}", "SHOW COLUMNS FROM", table_name);
        let column_rows = sqlx::query(&get_records_query.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return parse_sql_table_rows(column_rows);
    }

    pub async fn execute_input_query(
        &self,
        input: String,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let query = format!("{}", input);
        match sqlx::query(&query.as_str()).execute(&self.pool).await {
            Ok(result) => return Ok(result),
            Err(e) => return Err(e),
        };
    }
}

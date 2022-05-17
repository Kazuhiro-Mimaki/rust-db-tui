use sqlx::{mysql::MySqlQueryResult, MySql, MySqlPool, Pool};

use super::parser::{parse_sql_db, parse_sql_table_rows, parse_sql_tables};

pub struct MySqlClient {
    pub pool: Pool<MySql>,
}

impl MySqlClient {
    pub async fn new(db_url: &str) -> Self {
        Self {
            pool: MySqlPool::connect(db_url).await.unwrap(),
        }
    }

    pub async fn change_db(&mut self, new_db_url: &str) {
        self.pool = MySqlPool::connect(new_db_url).await.unwrap();
    }

    pub async fn get_db_list(&self) -> Vec<String> {
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

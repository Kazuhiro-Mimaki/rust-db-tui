use crate::db::sql_client::MySqlClient;

use super::table::TableModel;

pub struct DatabaseModel {
    pub current_database: String,
    pub databases: Vec<String>,
    pub current_table: TableModel,
    pub tables: Vec<String>,
}

impl DatabaseModel {
    pub async fn new(mysql_client: &mut MySqlClient) -> Self {
        let databases = mysql_client.get_database_list().await;
        mysql_client.reconnect(databases[0].clone()).await;
        let tables = mysql_client.get_table_list(databases[0].clone()).await;
        let current_table = TableModel::new(&mysql_client, tables[0].clone()).await;

        Self {
            current_database: databases[0].clone(),
            current_table: current_table,
            databases: databases,
            tables: tables,
        }
    }

    pub async fn set_default_database(&self, mysql_client: &mut MySqlClient) {
        mysql_client.reconnect(self.databases[0].clone()).await;
    }

    pub async fn change_database(&self, mysql_client: &mut MySqlClient, new_database: String) {
        mysql_client.reconnect(new_database).await;
    }
}

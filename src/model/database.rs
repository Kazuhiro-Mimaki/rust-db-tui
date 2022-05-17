use crate::db::sql_client::MySqlClient;

pub struct DatabaseModel {
    pub current_database: String,
    pub databases: Vec<String>,
}

impl DatabaseModel {
    pub async fn new(mysql_client: &MySqlClient) -> Self {
        let databases = mysql_client.get_database_list().await;

        Self {
            current_database: databases[0].clone(),
            databases: databases,
        }
    }

    pub async fn set_default_database(&self, mysql_client: &mut MySqlClient) {
        mysql_client.reconnect(self.databases[0].clone()).await;
    }

    pub async fn change_database(&self, mysql_client: &mut MySqlClient, new_database: String) {
        mysql_client.reconnect(new_database).await;
    }
}

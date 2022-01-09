use crate::entity::{prelude::*, todos};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Debug, Clone)]
pub struct TodosRepository {
    pub db_conn: DatabaseConnection,
}

impl TodosRepository {
    pub async fn get_todos(&self) -> Vec<todos::Model> {
        Todos::find()
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all todos")
    }
}

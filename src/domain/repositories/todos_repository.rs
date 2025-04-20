use crate::domain::entities::todo::{DraftTodo, Todo};
use async_trait::async_trait;

#[async_trait]
pub trait TodosRepository {
    async fn create(&self, todo: DraftTodo) -> anyhow::Result<Todo>;
}

use crate::domain::entities::todo::{DraftTodo, Todo};

pub trait TodosRepository {
    async fn create(&self, todo: DraftTodo) -> anyhow::Result<Todo>;
}

use async_trait::async_trait;
use sqlx::query_as;

use crate::{
    config::datastore::DataStore,
    domain::{
        entities::todo::{DraftTodo, Todo},
        repositories::todos_repository::TodosRepository,
    },
};

pub struct SqlxTodosRepository<'a> {
    datastore: &'a DataStore,
}

impl<'a> SqlxTodosRepository<'a> {
    pub fn new(datastore: &'a DataStore) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl TodosRepository for SqlxTodosRepository<'_> {
    async fn create(&self, draft_todo: DraftTodo) -> anyhow::Result<Todo> {
        let (id,): (i32,) = query_as(
            r#"INSERT INTO "todos"
            (title, content, completed)
            VALUES ($1, $2, $3)
            RETURNING id"#,
        )
        .bind(&draft_todo.title)
        .bind(&draft_todo.content)
        .bind(draft_todo.completed)
        .fetch_one(self.datastore.get_db())
        .await?;

        Ok(draft_todo.into_todo(id))
    }
}

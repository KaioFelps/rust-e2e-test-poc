use async_trait::async_trait;
use sqlx::{query_as, Postgres, QueryBuilder};

use crate::{
    common::paginated_fetch::PaginatedFetch,
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

    async fn find_many_paginated(
        &self,
        page: u32,
        per_page: u8,
        query: Option<String>,
        completed: Option<bool>,
    ) -> anyhow::Result<PaginatedFetch<Todo>> {
        let mut select_query = QueryBuilder::<Postgres>::new(
            r#"SELECT id, title, content, created_at, completed
            FROM "todos""#,
        );

        let mut count_query = QueryBuilder::<Postgres>::new(r#"SELECT COUNT(id) from "todos""#);

        let mut has_inserted_early_where_clause = false;
        if let Some(content) = query {
            select_query
                .push(" WHERE (content ilike CONCAT('%', ")
                .push_bind(content.clone())
                .push(", '%') OR title ilike CONCAT('%', ")
                .push_bind(content.clone())
                .push(", '%'))");

            count_query
                .push(" WHERE (content ilike CONCAT('%', ")
                .push_bind(content.clone())
                .push(", '%') OR title ilike CONCAT('%', ")
                .push_bind(content.clone())
                .push(", '%'))");

            has_inserted_early_where_clause = true;
        }

        if let Some(completed) = completed {
            let where_statement = if has_inserted_early_where_clause {
                " AND "
            } else {
                " WHERE "
            };

            select_query
                .push(where_statement)
                .push("completed = ")
                .push_bind(completed);

            count_query
                .push(where_statement)
                .push("completed = ")
                .push_bind(completed);
        }

        select_query
            .push(" ORDER BY created_at DESC")
            .push(" LIMIT ")
            .push_bind(per_page as i16)
            .push(" OFFSET ")
            .push_bind(((page - 1) * per_page as u32) as i32);

        let (todos, (count,)): (Vec<Todo>, (i64,)) = tokio::try_join!(
            select_query
                .build_query_as()
                .fetch_all(self.datastore.get_db()),
            count_query
                .build_query_as()
                .fetch_one(self.datastore.get_db())
        )?;

        Ok(PaginatedFetch(todos, count as u64))
    }
}

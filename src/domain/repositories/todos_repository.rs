use crate::{
    common::paginated_fetch::PaginatedFetch,
    domain::entities::todo::{DraftTodo, Todo},
};
use async_trait::async_trait;

#[async_trait]
pub trait TodosRepository {
    async fn create(&self, todo: DraftTodo) -> anyhow::Result<Todo>;

    async fn find_many_paginated(
        &self,
        page: u32,
        per_page: u8,
        query: Option<String>,
        completed: Option<bool>,
    ) -> anyhow::Result<PaginatedFetch<Todo>>;
}

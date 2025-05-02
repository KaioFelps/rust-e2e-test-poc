use crate::{
    common::paginated_fetch::PaginatedFetch,
    config::options::Options,
    domain::{entities::todo::Todo, repositories::todos_repository::TodosRepository},
};

pub struct FetchPaginatedTodosParams {
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub query: Option<String>,
    pub completed: Option<bool>,
}

pub struct FetchPaginatedTodos<TR: TodosRepository> {
    todos_repository: TR,
}

impl<TR: TodosRepository> FetchPaginatedTodos<TR> {
    pub fn new(todos_repository: TR) -> Self {
        Self { todos_repository }
    }

    pub async fn exec(
        &self,
        params: FetchPaginatedTodosParams,
    ) -> anyhow::Result<PaginatedFetch<Todo>> {
        self.todos_repository
            .find_many_paginated(
                params.page.unwrap_or(1),
                params
                    .per_page
                    .unwrap_or_else(|| Options::get().default_per_page),
                params.query,
                params.completed,
            )
            .await
    }
}

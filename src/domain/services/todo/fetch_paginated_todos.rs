use crate::{
    common::paginated_fetch::PaginatedFetch,
    config::options::Options,
    domain::{entities::todo::Todo, repositories::todos_repository::TodosRepository},
};

pub enum TodosQuery {
    Completed(bool),
    Content(String),
}

pub struct FetchPaginatedTodosParams {
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub query: Option<TodosQuery>,
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
                    .unwrap_or_else(|| Options::get().get_default_per_page()),
                params.query,
            )
            .await
    }
}

use serde::Deserialize;
use validator::Validate;

use crate::domain::services::todo::fetch_paginated_todos::TodosQuery;

#[derive(Validate, Deserialize)]
pub struct CreateTodoDto {
    #[validate(required(message = "`title` is a mandatory field."))]
    pub title: Option<String>,

    #[validate(required(message = "`content` is a mandatory field."))]
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub enum PaginatedTodosDtoQueryBy {
    #[serde(rename = "content")]
    Content,

    #[serde(rename = "completed")]
    Completed,
}

#[derive(Validate, Deserialize, Default)]
pub struct PaginatedTodosDto {
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub query: Option<String>,
    pub query_by: Option<PaginatedTodosDtoQueryBy>,
}

impl PaginatedTodosDto {
    pub fn get_query(&self) -> Option<TodosQuery> {
        Some(match self.query_by.as_ref()? {
            PaginatedTodosDtoQueryBy::Completed => {
                TodosQuery::Completed(self.query.as_ref()?.parse::<bool>().ok()?)
            }
            PaginatedTodosDtoQueryBy::Content => {
                TodosQuery::Content(self.query.as_ref()?.to_owned())
            }
        })
    }
}

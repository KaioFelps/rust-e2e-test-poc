use std::borrow::Cow;

use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::domain::services::todo::fetch_paginated_todos::TodosQuery;

use super::ENUM_INVALID_VARIANT_ERR;

#[derive(Validate, Deserialize)]
pub struct CreateTodoDto {
    #[validate(required(message = "`title` is a mandatory field."))]
    pub title: Option<String>,

    #[validate(required(message = "`content` is a mandatory field."))]
    pub content: Option<String>,
}

#[derive(Validate, Deserialize, Default)]
pub struct PaginatedTodosDto {
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub query: Option<String>,
    #[validate(custom(function = is_valid_variant_of_paginated_todos_query_by))]
    pub query_by: Option<String>,
}

impl PaginatedTodosDto {
    pub fn get_query(&self) -> Option<TodosQuery> {
        match self.query_by.as_ref()?.as_str() {
            "completed" => Some(TodosQuery::Completed(
                self.query.as_ref()?.parse::<bool>().ok()?,
            )),
            "content" => Some(TodosQuery::Content(self.query.as_ref()?.to_owned())),
            _ => None,
        }
    }
}

fn is_valid_variant_of_paginated_todos_query_by(query_by: &str) -> Result<(), ValidationError> {
    match query_by.to_string().to_lowercase().as_str() {
        "completed" => Ok(()),
        "content" => Ok(()),
        _ => Err(
            ValidationError::new(ENUM_INVALID_VARIANT_ERR).with_message(Cow::Owned(format!(
                "Received invalid variant `{query_by}`, expected either `content` or `completed`.",
            ))),
        ),
    }
}

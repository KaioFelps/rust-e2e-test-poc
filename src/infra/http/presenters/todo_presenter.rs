use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::entities::todo::Todo;

#[derive(Serialize, Deserialize)]
pub struct TodoPresenter {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub completed: bool,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl From<Todo> for TodoPresenter {
    fn from(value: Todo) -> Self {
        Self {
            completed: value.completed(),
            content: value.content().to_owned(),
            created_at: value.created_at(),
            id: value.id(),
            title: value.title().to_owned(),
        }
    }
}

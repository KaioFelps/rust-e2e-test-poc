use serde::Serialize;

use crate::domain::entities::todo::Todo;

#[derive(Serialize)]
pub struct TodoPresenter {
    id: i32,
    title: String,
    content: String,
    completed: bool,
    #[serde(rename = "createdAt")]
    created_at: String,
}

impl From<Todo> for TodoPresenter {
    fn from(value: Todo) -> Self {
        Self {
            completed: value.completed(),
            content: value.content().to_owned(),
            created_at: value.created_at().to_string(),
            id: value.id(),
            title: value.title().to_owned(),
        }
    }
}

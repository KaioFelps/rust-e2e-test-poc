use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct CreateTodoDto {
    #[validate(required(message = "`title` is a mandatory field"))]
    pub title: Option<String>,

    #[validate(required(message = "`content` is a mandatory field"))]
    pub content: Option<String>,
}

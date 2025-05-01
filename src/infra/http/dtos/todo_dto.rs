use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct CreateTodoDto {
    #[validate(required(message = "`title` is a mandatory field."))]
    pub title: Option<String>,

    #[validate(required(message = "`content` is a mandatory field."))]
    pub content: Option<String>,
}

#[derive(Validate, Deserialize, Default, Debug)]
pub struct PaginatedTodosDto {
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub query: Option<String>,
    pub completed: Option<bool>,
}

//     match query_by.to_string().to_lowercase().as_str() {
//         "completed" => Ok(()),
//         "content" => Ok(()),
//         _ => Err(
//             ValidationError::new(ENUM_INVALID_VARIANT_ERR).with_message(Cow::Owned(format!(
//                 "Received invalid variant `{query_by}`, expected either `content` or `completed`.",
//             ))),
//         ),
//     }
// }

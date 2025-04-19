use crate::{
    config::datastore::DataStore, domain::services::todo::create_todo_service::CreateTodoService,
    infra::sqlx::repositories::todos_repository::SqlxTodosRepository,
};

pub fn get_create_todo_service(datastore: &DataStore) -> CreateTodoService<SqlxTodosRepository> {
    let todos_repository = SqlxTodosRepository::new(datastore);
    CreateTodoService::new(todos_repository)
}

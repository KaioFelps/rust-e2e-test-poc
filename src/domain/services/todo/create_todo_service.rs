use crate::domain::entities::todo::{DraftTodo, Todo};
use crate::domain::repositories::todos_repository::TodosRepository;

pub struct CreateTodoParams {
    pub draft_todo: DraftTodo,
}

pub struct CreateTodoService<TR: TodosRepository> {
    todos_repository: TR,
}

impl<TR: TodosRepository> CreateTodoService<TR> {
    pub fn new(todos_repository: TR) -> Self {
        Self { todos_repository }
    }
}

impl<TR: TodosRepository> CreateTodoService<TR> {
    pub async fn exec(&self, params: CreateTodoParams) -> anyhow::Result<Todo> {
        self.todos_repository.create(params.draft_todo).await
    }
}

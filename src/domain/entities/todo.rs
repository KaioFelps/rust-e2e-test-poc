use chrono::NaiveDateTime;

pub struct Todo {
    id: i32,
    title: String,
    content: String,
    completed: bool,
    created_at: NaiveDateTime,
}

pub struct DraftTodo {
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

impl Todo {
    pub fn draft(title: String, content: String) -> DraftTodo {
        DraftTodo {
            title,
            content,
            completed: false,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn new_from_existing(
        id: i32,
        title: String,
        content: String,
        completed: bool,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            title,
            content,
            completed,
            created_at,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

impl DraftTodo {
    pub fn into_todo(self, id: i32) -> Todo {
        Todo {
            id,
            title: self.title,
            content: self.content,
            completed: self.completed,
            created_at: self.created_at,
        }
    }
}

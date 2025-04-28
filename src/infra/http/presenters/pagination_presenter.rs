use serde::Serialize;

#[derive(Serialize)]
pub struct PaginationPresenter {
    #[serde(rename = "totalItems")]
    total_items: u64,

    #[serde(rename = "perPage")]
    per_page: u8,

    #[serde(rename = "currentPage")]
    current_page: u32,

    #[serde(rename = "lastPage")]
    last_page: u32,
}

impl PaginationPresenter {
    pub fn new(total_items: u64, current_page: u32, per_page: u8) -> Self {
        Self {
            current_page,
            last_page: (total_items as f64 / per_page as f64).ceil() as u32,
            per_page,
            total_items,
        }
    }
}

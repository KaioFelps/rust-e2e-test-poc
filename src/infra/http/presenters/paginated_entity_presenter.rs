use serde::Serialize;

use super::pagination_presenter::PaginationPresenter;

#[derive(Serialize)]
pub struct PaginatedEntityPresenter<T: Serialize> {
    pub data: Vec<T>,
    pub pagination: PaginationPresenter,
}

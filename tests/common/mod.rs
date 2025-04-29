pub mod fixtures;
pub mod setup;

pub fn get_unique_db_schema() -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static ID: AtomicUsize = AtomicUsize::new(0);
    format!("test_schema_{}", ID.fetch_add(1, Ordering::SeqCst))
}

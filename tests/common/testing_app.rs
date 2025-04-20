use actix_web::web::Data;
use e2e_test_poc::config::{self};
use inertia_rust::Inertia;
use rstest::fixture;

use super::{datastore_guard::DataStoreGuard, get_unique_db_schema};

pub type TestingAppData<'a> = (Data<Inertia>, DataStoreGuard<'a>);

#[fixture]
pub async fn testing_app_data<'b>(
    #[default(get_unique_db_schema().leak())] schema: &'static str,
) -> TestingAppData<'b> {
    let vite = config::vite::get_vite()
        .await
        .expect("Failed to initialize vite in test fixture.");

    let inertia = config::inertia::get_inertia(vite)
        .await
        .expect("Failed to initialize inertia in test fixture.");

    (Data::new(inertia), DataStoreGuard::new(schema).await)
}

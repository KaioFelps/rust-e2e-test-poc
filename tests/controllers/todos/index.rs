use crate::common::{
    fixtures::datastore::{datastore, DataStoreGuard},
    fixtures::inertia::inertia,
    setup::__setup,
};
use actix_web::{http::StatusCode, test::TestRequest, web::Data};
use e2e_test_poc::config::{datastore::DataStore, server::get_server};
use inertia_rust::{
    test::{InertiaTestRequest, IntoAssertableInertia},
    Inertia,
};
use pretty_assertions::assert_eq;
use rstest::rstest;

async fn seed_todos(datastore: &DataStore) {
    let has_inserted = sqlx::query(
        r#"INSERT INTO "todos"
        (id, title, content, completed)
        VALUES
        (1, 'Foo', 'Content of foo', false),
        (2, 'Bar', 'Content of bar', true),
        (3, 'Baz', 'Content of baz', true)"#,
    )
    .execute(datastore.get_db())
    .await;

    assert!(has_inserted.is_ok());
}

#[rstest]
#[tokio::test]
#[awt]
async fn returns_is_a_valid_inertia_response(
    #[future] __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] mut datastore: DataStoreGuard<'_>,
) {
    let app =
        actix_web::test::init_service(get_server().app_data(inertia).app_data(datastore.take()))
            .await;

    let response = TestRequest::get().uri("/").send_request(&app).await;

    assert_eq!(StatusCode::OK, response.status());
    assert!(response.headers().contains_key("x-inertia"));
}

#[rstest]
#[awt]
#[tokio::test]
async fn can_fetch_latest_todos(
    #[future] __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] mut datastore: DataStoreGuard<'_>,
) {
    seed_todos(datastore.as_ref()).await;

    let app =
        actix_web::test::init_service(get_server().app_data(inertia).app_data(datastore.take()))
            .await;

    let page = TestRequest::get()
        .inertia()
        .uri("/")
        .send_request(&app)
        .await;

    let page: inertia_rust::test::AssertableInertia = page.into_assertable_inertia();

    assert_eq!(
        3,
        page.props
            .get("todos")
            .unwrap()
            .as_object()
            .unwrap()
            .get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .len()
    );
}

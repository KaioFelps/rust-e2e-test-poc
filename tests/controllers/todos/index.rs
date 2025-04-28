use crate::common::{
    setup::__setup,
    testing_app::{testing_app_data, TestingAppData},
};
use actix_web::{http::StatusCode, test::TestRequest};
use e2e_test_poc::config::server::get_server;
use inertia_rust::test::{InertiaTestRequest, IntoAssertableInertia};
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[tokio::test]
#[awt]
async fn returns_is_a_valid_inertia_response(
    #[future] __setup: (),
    #[future] testing_app_data: TestingAppData<'_>,
) {
    let (inertia, mut datastore_guard) = testing_app_data;

    let app = actix_web::test::init_service(
        get_server()
            .app_data(inertia)
            .app_data(datastore_guard.take()),
    )
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
    #[future] testing_app_data: TestingAppData<'_>,
) {
    let (inertia, mut datastore) = testing_app_data;

    let has_inserted = sqlx::query(
        r#"INSERT INTO "todos"
        (id, title, content, completed)
        VALUES
        (1, 'Foo', 'Content of foo', false),
        (2, 'Bar', 'Content of bar', true),
        (3, 'Baz', 'Content of baz', true)"#,
    )
    .execute(datastore.datastore.as_ref().unwrap().get_db())
    .await;

    assert!(has_inserted.is_ok());

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

use actix_web::{http::StatusCode, test::TestRequest};
use e2e_test_poc::config::server::get_server;
use pretty_assertions::assert_eq;
use rstest::rstest;
use serde_json::json;

use crate::common::{
    setup::__setup,
    testing_app::{testing_app_data, TestingAppData},
};

#[rstest]
#[tokio::test]
#[awt]
async fn index(#[future] __setup: (), #[future] testing_app_data: TestingAppData<'_>) {
    let (inertia, datastore_guard) = testing_app_data;

    let app = actix_web::test::init_service(
        get_server()
            .app_data(inertia)
            .app_data(datastore_guard.datastore.clone()),
    )
    .await;

    let response = TestRequest::get().uri("/").send_request(&app).await;

    assert_eq!(StatusCode::OK, response.status());
    assert!(response.headers().contains_key("x-inertia"));
}

#[rstest]
#[awt]
#[tokio::test]
async fn create(#[future] __setup: (), #[future] testing_app_data: TestingAppData<'_>) {
    let (inertia, datastore_guard) = testing_app_data;

    let app = actix_web::test::init_service(
        get_server()
            .app_data(inertia)
            .app_data(datastore_guard.datastore.clone()),
    )
    .await;

    let response = TestRequest::post()
        .uri("/create")
        .set_json(json!({
            "title": "task",
            "content": "the task content"
        }))
        .insert_header((actix_web::http::header::REFERER, "/foo"))
        .send_request(&app)
        .await;

    let (count,): (i64,) = sqlx::query_as(r#"SELECT COUNT(id) FROM todos"#)
        .fetch_one(datastore_guard.datastore.get_db())
        .await
        .unwrap();

    assert_eq!(
        "/",
        response
            .headers()
            .iter()
            .find(|(key, _)| key.as_str() == "location")
            .unwrap()
            .1,
        "Expected it to always redirect to '/', regardless it's succeeded."
    );
    assert_eq!(StatusCode::SEE_OTHER, response.status());
    assert_eq!(1, count);
}

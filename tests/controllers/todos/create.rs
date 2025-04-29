use actix_web::{http::StatusCode, test::TestRequest, web::Data};
use e2e_test_poc::config::server::get_server;
use inertia_rust::Inertia;
use pretty_assertions::assert_eq;
use rstest::rstest;
use serde_json::json;

use crate::common::{
    fixtures::{
        datastore::{datastore, DataStoreGuard},
        inertia::inertia,
    },
    setup::__setup,
};

#[rstest]
#[awt]
#[tokio::test]
async fn create(
    #[future] __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] mut datastore: DataStoreGuard<'_>,
) {
    let app = actix_web::test::init_service(
        get_server()
            .app_data(inertia)
            .app_data(datastore.clone_datastore()),
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
        .fetch_one(datastore.take().get_db())
        .await
        .unwrap();

    assert_eq!(StatusCode::SEE_OTHER, response.status());
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
    assert_eq!(1, count);
}

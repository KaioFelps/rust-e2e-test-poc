use crate::common::{
    fixtures::datastore::{datastore, DataStoreGuard},
    fixtures::inertia::inertia,
    setup::__setup,
};
use actix_web::{http::StatusCode, test::TestRequest, web::Data};
use chrono::{Duration, Utc};
use e2e_test_poc::{
    config::{datastore::DataStore, server::get_server},
    infra::http::presenters::todo_presenter::TodoPresenter,
};
use inertia_rust::{
    test::{InertiaTestRequest, IntoAssertableInertia},
    Inertia,
};
use pretty_assertions::assert_eq;
use rstest::rstest;

async fn seed_todos(datastore: &DataStore) {
    let created_at = Utc::now();
    let created_at_2 = created_at + Duration::minutes(10);
    let created_at_3 = created_at_2 + Duration::hours(2);

    let has_inserted = sqlx::query(
        r#"INSERT INTO "todos" (id, title, content, completed, created_at)
        VALUES  (1, 'Foo', 'Content of foo', false, $1::TIMESTAMP),
                (2, 'Bar', 'Content of bar', true, $2::TIMESTAMP),
                (3, 'Baz', 'Content of baz', true, $3::TIMESTAMP)"#,
    )
    .bind(created_at)
    .bind(created_at_2)
    .bind(created_at_3)
    .execute(datastore.get_db())
    .await;

    if let Err(ref err) = has_inserted {
        log::error!("{err}");
    }

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

    let page = page.into_assertable_inertia();

    assert_eq!(3, page.props["todos"]["data"].as_array().unwrap().len());
    assert_eq!(3, page.props["todos"]["pagination"]["totalItems"]);
    assert_eq!(
        "Baz",
        page.props["todos"]["data"][0]["title"].as_str().unwrap()
    );
}

#[rstest]
#[awt]
#[tokio::test]
async fn invariant_of_query_by_argument_from_query_causes_validation_error(
    #[future] __setup: (),
    #[future] mut datastore: DataStoreGuard<'_>,
    #[future] inertia: Data<Inertia>,
) {
    let app =
        actix_web::test::init_service(get_server().app_data(inertia).app_data(datastore.take()))
            .await;

    let response = TestRequest::get()
        .uri("/?query_by=not_content_nor_completed&query=baz")
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[rstest]
#[awt]
#[tokio::test]
async fn can_fetch_todos_filtered_by_content(
    #[future] __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] mut datastore: DataStoreGuard<'_>,
) {
    seed_todos(datastore.as_ref()).await;

    let app =
        actix_web::test::init_service(get_server().app_data(inertia).app_data(datastore.take()))
            .await;

    let response = TestRequest::get()
        .inertia()
        .uri("/?query_by=content&query=baz")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());

    let page = response.into_assertable_inertia();

    assert_eq!(1, page.props["todos"]["data"].as_array().unwrap().len());
    assert_eq!(1, page.props["todos"]["pagination"]["totalItems"]);

    let todo = &page.props["todos"]["data"][0];
    assert_eq!(3, todo["id"]);
    assert_eq!("Baz", todo["title"].as_str().unwrap());
    assert_eq!("Content of baz", todo["content"].as_str().unwrap());
    assert_eq!(true, todo["completed"]);
}

#[rstest]
#[awt]
#[tokio::test]
async fn can_fetch_todos_filtered_by_completed_status(
    #[future] __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] mut datastore: DataStoreGuard<'_>,
) {
    seed_todos(datastore.as_ref()).await;

    let app =
        actix_web::test::init_service(get_server().app_data(inertia).app_data(datastore.take()))
            .await;

    let response = TestRequest::get()
        .inertia()
        .uri("/?query_by=completed&query=true")
        .send_request(&app)
        .await;

    assert!(response.status().is_success());

    let page = response.into_assertable_inertia();

    assert_eq!(2, page.props["todos"]["data"].as_array().unwrap().len());
    assert_eq!(2, page.props["todos"]["pagination"]["totalItems"]);

    let first_todo =
        serde_json::from_value::<TodoPresenter>(page.props["todos"]["data"][0].clone()).unwrap();

    let second_todo =
        serde_json::from_value::<TodoPresenter>(page.props["todos"]["data"][1].clone()).unwrap();

    assert!(first_todo.created_at >= second_todo.created_at);

    assert_eq!(3, first_todo.id);
    assert_eq!("Baz", first_todo.title);
    assert_eq!("Content of baz", first_todo.content);
    assert_eq!(true, first_todo.completed);

    assert_eq!(2, second_todo.id);
    assert_eq!("Bar", second_todo.title);
    assert_eq!("Content of bar", second_todo.content);
    assert_eq!(true, second_todo.completed);
}

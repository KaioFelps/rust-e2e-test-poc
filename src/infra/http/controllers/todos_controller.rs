use actix_web::{
    web::{self, Data, Json, Query, Redirect, ServiceConfig},
    HttpRequest, Responder,
};
use inertia_rust::{
    hashmap, validators::InertiaValidateOrRedirect, Inertia, InertiaFacade, InertiaProp,
};
use validator::Validate;

use crate::{
    common::error::AppResult,
    config::{datastore::DataStore, options::Options},
    domain::{
        entities::todo::Todo,
        services::todo::{
            create_todo_service::CreateTodoParams, fetch_paginated_todos::FetchPaginatedTodosParams,
        },
    },
    infra::{
        factories::services::todo::{get_create_todo_service, get_fetch_paginated_todos_service},
        http::{
            dtos::todo_dto::{CreateTodoDto, PaginatedTodosDto},
            presenters::{
                paginated_entity_presenter::PaginatedEntityPresenter,
                pagination_presenter::PaginationPresenter, todo_presenter::TodoPresenter,
            },
        },
        sessions::helpers::flash_silently,
    },
};

use super::controller::Controller;

pub struct TodosController;

impl Controller for TodosController {
    fn register(cfg: &mut ServiceConfig) {
        cfg.route("/", web::get().to(Self::index))
            .route("/create", web::post().to(Self::create));
    }
}

impl TodosController {
    async fn create(
        req: HttpRequest,
        body: Json<CreateTodoDto>,
        datastore: Data<DataStore>,
    ) -> impl Responder {
        let body = match body.validate_or_back(&req) {
            Err(redirect) => return redirect,
            Ok(body) => body,
        };

        let draft_todo = Todo::draft(body.title.unwrap(), body.content.unwrap());
        let todo_title = draft_todo.title.clone();

        match get_create_todo_service(&datastore)
            .exec(CreateTodoParams { draft_todo })
            .await
        {
            Err(err) => {
                log::error!("{err}");

                Inertia::back_with_errors(
                    &req,
                    hashmap![
                        "error" => "We had some problems and could not store your todo task at the moment.".into(),
                    ],
                )
            }
            Ok(_) => {
                flash_silently(
                    &req,
                    "success",
                    format!("Added a new task \"{todo_title}!\""),
                );
                Redirect::to("/").see_other()
            }
        }
    }

    async fn index(
        req: HttpRequest,
        Query(search_params): Query<PaginatedTodosDto>,
        datastore: Data<DataStore>,
    ) -> AppResult<impl Responder> {
        search_params.validate()?;

        let params = FetchPaginatedTodosParams {
            page: search_params.page,
            per_page: search_params.per_page,
            query: search_params.query,
            completed: search_params.completed,
        };

        let todos = get_fetch_paginated_todos_service(&datastore)
            .exec(params)
            .await?;

        let todos = PaginatedEntityPresenter {
            data: todos.0.into_iter().map(TodoPresenter::from).collect(),
            pagination: PaginationPresenter::new(
                todos.1,
                search_params.page.unwrap_or(1),
                search_params
                    .per_page
                    .unwrap_or_else(|| Options::get().default_per_page),
            ),
        };

        Inertia::render_with_props(
            &req,
            "index".into(),
            hashmap![
                "todos" => InertiaProp::data(todos)
            ],
        )
        .await
        .map_err(Into::into)
    }
}

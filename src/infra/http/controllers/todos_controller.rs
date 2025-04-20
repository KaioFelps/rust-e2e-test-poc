use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpRequest, Responder,
};
use inertia_rust::{hashmap, validators::InertiaValidateOrRedirect, Inertia, InertiaFacade};

use crate::{
    config::datastore::DataStore,
    domain::{entities::todo::Todo, services::todo::create_todo_service::CreateTodoParams},
    infra::{
        factories::services::todo::get_create_todo_service, http::dtos::todo_dto::CreateTodoDto,
    },
};

use super::controller::{AppRedirect, Controller};

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
    ) -> AppRedirect {
        let body = match body.validate_or_back(&req) {
            Err(redirect) => return redirect,
            Ok(body) => body,
        };

        let draft_todo = Todo::draft(body.title.unwrap(), body.content.unwrap());

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
            Ok(_) => Inertia::back(&req),
        }
    }

    async fn index(req: HttpRequest) -> impl Responder {
        Inertia::render(&req, "index".into()).await
    }
}

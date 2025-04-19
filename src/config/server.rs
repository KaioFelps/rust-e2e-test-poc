use actix_web::{
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    App,
};
use inertia_rust::actix::InertiaMiddleware;

use crate::infra::http::controllers::{controller::Controller, todos_controller::TodosController};

pub fn get_server() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(InertiaMiddleware::new())
        .configure(TodosController::register)
}

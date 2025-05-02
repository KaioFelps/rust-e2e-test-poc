use actix_session::SessionMiddleware;
use actix_web::{
    cookie::{Key, SameSite},
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    App,
};
use inertia_rust::actix::InertiaMiddleware;

use crate::infra::{
    http::controllers::{controller::Controller, todos_controller::TodosController},
    sessions::{
        file_session::FileSessionStore,
        middlewares::{
            garbage_collector::GarbageCollectorMiddleware,
            reflash_temporary_session::ReflashTemporarySessionMiddleware,
        },
    },
};

use super::options::{Options, RustEnv};

pub fn get_server() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let options = Options::get();
    let key = Key::from(options.app_key);
    let storage = FileSessionStore::default();

    App::new()
        .wrap(InertiaMiddleware::new())
        .wrap(ReflashTemporarySessionMiddleware)
        .wrap(GarbageCollectorMiddleware)
        .wrap(
            SessionMiddleware::builder(storage, key)
                .cookie_domain(None)
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                .cookie_name(options.sessions_cookie_name.into())
                .cookie_secure(options.environment == RustEnv::Production)
                .build(),
        )
        .configure(TodosController::register)
}

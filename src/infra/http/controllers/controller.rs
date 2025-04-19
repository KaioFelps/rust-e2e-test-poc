use actix_web::{
    web::{Redirect, ServiceConfig},
    HttpResponse,
};

pub trait Controller {
    fn register(cfg: &mut ServiceConfig);
}

pub type AppResponse<Response = HttpResponse, Err = anyhow::Error> = Result<Response, Err>;
pub type AppRedirect = Redirect;

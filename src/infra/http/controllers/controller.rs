use actix_web::web::ServiceConfig;

pub trait Controller {
    fn register(cfg: &mut ServiceConfig);
}

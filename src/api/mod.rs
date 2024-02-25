mod ping;

use actix_web::{Responder, web};
use actix_web::dev::{ServiceFactory};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .service(ping::ping)
    );
}
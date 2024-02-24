mod ping;

use actix_web::{App, Responder, web};
use actix_web::dev::{ServiceFactory, ServiceRequest};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .service(ping::ping)
    );
}
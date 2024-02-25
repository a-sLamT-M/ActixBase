mod ping;

use actix_web::{web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .service(ping::ping)
    );
}
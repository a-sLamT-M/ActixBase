mod api;
mod config;
mod logger;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let environment = args.get(1).unwrap_or(&"dev".to_string()).to_string();

    let config = config::Config::new(environment).unwrap_or_else(|err| {
        eprintln!("Failed to read config: {}", err);
        std::process::exit(1);
    });

    logger::build().unwrap_or_else(|err| {
        eprintln!("Failed to initialize logger: {}", err);
        std::process::exit(1);
    });

    HttpServer::new(|| {
        App::new()
            .configure(api::routes)
    }).bind((config.ip, config.port))?.run().await
}

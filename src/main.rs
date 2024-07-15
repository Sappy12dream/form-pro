use actix_web::{App, HttpServer};
use log;

mod config;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    utils::logger::init_logger();
    let config = config::server::load_config();
    let pool = config::db::establish_connection();
    let server_address = format!("{}:{}", config.host, config.port);

    log::info!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(routes::hello::hello)
            .service(routes::name::full_name)
    })
    .bind(&server_address)
    .unwrap_or_else(|err| {
        log::error!("Failed to bind to address {}: {}", server_address, err);
        std::process::exit(1);
    })
    .run()
    .await
}

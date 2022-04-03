use actix_web::{middleware, App, HttpServer};
use tracing::{info};

mod config;
mod db;

use config::load_config::{Config, load_from_file};
use db::dynamodb::{init_db_pool};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // load from config
    let cfg: Config = load_from_file();

    // init db connection with config
    init_db_pool(&cfg);

    info!("Starting server at http://localhost:{}/", &cfg.port);
    println!("{:?}", cfg);

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
    })
    .bind(format!("0.0.0.0:{}", cfg.port))?
    .run()
    .await
}
use actix_web::{middleware, App, HttpServer};
use aws_sdk_dynamodb::{Client, Error};
use tracing::{info};
use aws_sdk_dynamodb::Endpoint;

mod config;
mod db;
mod api;
mod users;

use config::load_config::{Config, load_from_file};
use db::dynamodb::{create_table_if_not_exists};
use api::handler_post_user::{post_user};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // load from config
    let cfg: Config = load_from_file();

    // Init DB
    match init_db(&cfg) {
        Ok(_) => println!("DB successfully started"),
        Err(e) => println!("Got error initiating DynamoDB: {}", e)
    }

    // Services init
    
    // Start server
    info!("Starting server at http://localhost:{}/", &cfg.port);
    println!("{:?}", cfg);

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .service(post_user)
    })
    .bind(format!("0.0.0.0:{}", cfg.port))?
    .run()
    .await
}

#[tokio::main]
async fn init_db(cfg: &Config) -> Result<(), Error> {
    // init db connection with config
    let config = aws_config::load_from_env().await;

    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_resolver(
            // 8000 is the default dynamodb port
            Endpoint::immutable(actix_web::http::Uri::from_static("http://localhost:8000")),
        )
        .build();

    let client = Client::from_conf(dynamodb_local_config);
    create_table_if_not_exists(&cfg, &client).await
}
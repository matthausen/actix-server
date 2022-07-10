use actix_web::{middleware, web, App, HttpServer};
use aws_sdk_dynamodb::{Client};
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

    // Services init
    //let userStorage = Storage::new(&cfg);

    // init DynamoDB client
    let aws_config = aws_config::load_from_env().await;
    let dynamodb_local_aws_config = aws_sdk_dynamodb::config::Builder::from(&aws_config)
        .endpoint_resolver(
            Endpoint::immutable(actix_web::http::Uri::from_static("http://localhost:8000")),
        )
        .build();
    let client = Client::from_conf(dynamodb_local_aws_config);
    
    // Create the table if it doesn't exist
    create_table_if_not_exists(&cfg, &client).await;
    
    
    // Start server
    info!("Starting server at http://localhost:{}/", &cfg.port);
    println!("{:?}", cfg);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(client.clone()))
        .app_data(web::Data::new(cfg.clone()))
        .wrap(middleware::Logger::default())
        .service(post_user)
    })
    .bind(format!("0.0.0.0:{}", "8080"))?
    .run()
    .await
}
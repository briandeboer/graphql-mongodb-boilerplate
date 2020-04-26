use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::io;
use std::sync::Arc;
use uuid::Uuid;

mod db;
mod models;
mod routes;
mod schema;

use crate::db::Clients;
use crate::routes::app_routes;
use crate::schema::create_schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=warn");
    env_logger::init();
    dotenv().ok();

    let port = dotenv::var("PORT").unwrap_or("8080".to_owned());

    let db_clients = Arc::new(Clients {
        mongo: db::mongo::connect(),
    });

    let gql = std::sync::Arc::new(create_schema());
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(gql.clone())
            .data(db_clients.clone())
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o"))
            .configure(app_routes)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

mod graphql;
mod health;

use actix_web::{web, HttpResponse};
use graphql::{graphiql, graphql};
use health::{get_health, pong, readiness};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/")
                .route("ping", web::get().to(pong))
                .route("~/ready", web::get().to(readiness))
                .route("health", web::get().to(get_health))
                .route("graphql", web::post().to(graphql))
                .route("graphiql", web::get().to(graphiql)),
        )
        .route("", web::get().to(|| HttpResponse::NotFound()));
}

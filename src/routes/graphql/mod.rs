use crate::db::Clients;
use crate::schema::Schema;

use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use std::sync::Arc;

pub async fn graphiql() -> HttpResponse {
    let port = dotenv::var("PORT").unwrap_or("8080".to_owned());
    let html = graphiql_source(&format!("http://localhost:{}/graphql", port));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    clients: web::Data<Arc<Clients>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let res = data.execute(&st, &clients);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}

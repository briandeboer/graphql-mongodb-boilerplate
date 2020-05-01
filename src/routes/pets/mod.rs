use crate::db::Clients;
use crate::models::Pet;

use actix_web::{web, Responder};
use cached::TimedCache;
use mongodb_base_service::BaseService;
use mongodb_cursor_pagination::FindResult;
use std::sync::Arc;

pub async fn all_pets(clients: web::Data<Arc<Clients>>) -> impl Responder {
    cached_key! {
        ALL_PETS: TimedCache<String, Vec<Pet>> =
            TimedCache::with_lifespan_and_capacity(10, 10000);
        Key = { format!("hey") };
        fn build(
            clients: &Clients
        ) -> Vec<Pet> = {
            let service = clients.mongo.get_mongo_service("pets").unwrap();
            let result: FindResult<Pet> = service
                .find(None, None, None, None, None, None)
                .expect("Received data");
            result.items
        }
    }
    let pets = build(clients.get_ref());
    web::Json(pets)
}

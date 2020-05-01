#[macro_use]
extern crate cached;

mod db;
mod models;
mod schema;

use bson::doc;
use dotenv::dotenv;
use mongodb_base_service::{BaseService, ID};
use std::sync::Arc;

use crate::db::Clients;
use crate::schema::{create_schema, Schema};

fn main() {
    std::env::set_var("RUST_LOG", "info,actix_web=warn");
    env_logger::init();
    dotenv().ok();

    let db_clients = Arc::new(Clients {
        mongo: db::mongo::connect(),
    });

    // drop the existing data
    let owners_service = db_clients.mongo.get_mongo_service("owners").unwrap();
    let _ = owners_service.data_source().drop(None);

    let pets_service = db_clients.mongo.get_mongo_service("pets").unwrap();
    let _ = pets_service.data_source().drop(None);

    // seed data
    // create owners first
    let owners = vec![
        {
            doc! { "username": "jsmith", "first_name": "John", "last_name": "Smith", "gender": "Male" }
        },
        {
            doc! { "username": "janejohnson", "first_name": "Jane", "last_name": "Johnson", "gender": "Female" }
        },
        {
            doc! { "username": "bgoldman", "first_name": "Bob", "last_name": "Goldman", "gender": "Male" }
        },
        {
            doc! { "username": "emartinez", "first_name": "Eileen", "last_name": "Martinez", "gender": "Female" }
        },
        {
            doc! { "username": "helenp78", "first_name": "Helen", "last_name": "Phillips", "gender": "Female" }
        },
    ];

    // add the owners
    let ids: Vec<ID> = owners_service.insert_many(owners, None).unwrap();
    println!("{:?}", ids);
    let pets = vec![
        {
            doc! { "name": "Fido", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[0].to_bson() }
        },
        {
            doc! { "name": "Cleo", "pet_type": "Cat", "age": 12, "gender": "Female", "owner": &ids[1].to_bson() }
        },
        {
            doc! { "name": "Oreo", "pet_type": "Cat", "age": 2, "gender": "Female", "owner": &ids[2].to_bson() }
        },
        {
            doc! { "name": "Milo", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[3].to_bson() }
        },
        {
            doc! { "name": "Squirt", "pet_type": "Fish", "age": 2, "gender": "Female", "owner": &ids[4].to_bson() }
        },
        {
            doc! { "name": "Lurch", "pet_type": "Hamster", "age": 1, "gender": "Male", "owner": &ids[0].to_bson() }
        },
        {
            doc! { "name": "Fonz", "pet_type": "Turtle", "age": 10, "gender": "Male", "owner": &ids[1].to_bson() }
        },
        {
            doc! { "name": "Lucy", "pet_type": "Turtle", "age": 10, "gender": "Female", "owner": &ids[1].to_bson() }
        },
    ];

    let _pets_results: Vec<ID> = pets_service.insert_many(pets, None).unwrap();
    println!("Data inserted");

    // putting this here to prevent dead code check issues
    let _schema: Arc<Schema> = std::sync::Arc::new(create_schema());
}

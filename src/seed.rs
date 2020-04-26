mod db;
mod models;
mod schema;

use bson::doc;
use dotenv::dotenv;
use mongodb_base_service::BaseService;
use std::sync::Arc;

use crate::db::Clients;
use crate::models::{Owner, Pet};
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
    let results: Vec<Owner> = owners_service.insert_many(owners, None).unwrap();
    let ids: Vec<String> = results.iter().map(|x| x.node.id.to_string()).collect();
    let pets = vec![
        {
            doc! { "name": "Fido", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[0] }
        },
        {
            doc! { "name": "Cleo", "pet_type": "Cat", "age": 12, "gender": "Female", "owner": &ids[1] }
        },
        {
            doc! { "name": "Oreo", "pet_type": "Cat", "age": 2, "gender": "Female", "owner": &ids[2] }
        },
        {
            doc! { "name": "Milo", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[3] }
        },
        {
            doc! { "name": "Squirt", "pet_type": "Fish", "age": 2, "gender": "Female", "owner": &ids[4] }
        },
        {
            doc! { "name": "Lurch", "pet_type": "Hamster", "age": 1, "gender": "Male", "owner": &ids[0] }
        },
        {
            doc! { "name": "Fonz", "pet_type": "Turtle", "age": 10, "gender": "Male", "owner": &ids[1] }
        },
        {
            doc! { "name": "Lucy", "pet_type": "Turtle", "age": 10, "gender": "Female", "owner": &ids[1] }
        },
    ];

    let _pets_results: Vec<Pet> = pets_service.insert_many(pets, None).unwrap();
    println!("Data inserted");

    // putting this here to prevent dead code check issues
    let _schema: Arc<Schema> = std::sync::Arc::new(create_schema());
}

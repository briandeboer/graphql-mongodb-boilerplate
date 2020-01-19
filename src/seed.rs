/// adds some data into the database
mod graphql_schema;
mod db;
mod schema;
mod services;

use bson::{bson, doc};
use crate::schema::owners::Owner;
use crate::db::DataSources;
use crate::graphql_schema::{create_schema, Schema};
use std::sync::Arc;

fn main() {
    let client = db::establish_connection();

    let data_sources = DataSources {
        owners: client.collection("owners"),
        pets: client.collection("pets"),
    };

    // drop the tables
    let _ = data_sources.owners.drop(None);
    let _ = data_sources.pets.drop(None);

    // seed data
    // create owners first
    let owners = vec![
        { doc! { "username": "jsmith", "first_name": "John", "last_name": "Smith", "gender": "Male" } },
        { doc! { "username": "janejohnson", "first_name": "Jane", "last_name": "Johnson", "gender": "Female" } },
        { doc! { "username": "bgoldman", "first_name": "Bob", "last_name": "Goldman", "gender": "Male" } },
        { doc! { "username": "emartinez", "first_name": "Eileen", "last_name": "Martinez", "gender": "Female" } },
        { doc! { "username": "helenp78", "first_name": "Helen", "last_name": "Phillips", "gender": "Female" } },
    ];

    // add the owners
    data_sources.owners.insert_many(owners, None)
        .expect("Unable to insert data");

    // get the ids so we can add pets
    let cursor = data_sources.owners.find(None, None).unwrap();
    let mut ids: Vec<String> = vec![];
    for result in cursor {
        match result {
            Ok(doc) => {
                let owner: Owner = bson::from_bson(bson::Bson::Document(doc.clone())).unwrap();
                ids.push(owner.id.to_hex());
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    let pets = vec![
        { doc! { "name": "Fido", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[0] }},
        { doc! { "name": "Cleo", "pet_type": "Cat", "age": 12, "gender": "Female", "owner": &ids[1] }},
        { doc! { "name": "Oreo", "pet_type": "Cat", "age": 2, "gender": "Female", "owner": &ids[2] }},
        { doc! { "name": "Milo", "pet_type": "Dog", "age": 10, "gender": "Male", "owner": &ids[3] }},
        { doc! { "name": "Squirt", "pet_type": "Fish", "age": 2, "gender": "Female", "owner": &ids[4] }},
        { doc! { "name": "Lurch", "pet_type": "Hamster", "age": 1, "gender": "Male", "owner": &ids[0] }},
        { doc! { "name": "Fonz", "pet_type": "Turtle", "age": 10, "gender": "Male", "owner": &ids[1] }},
        { doc! { "name": "Lucy", "pet_type": "Turtle", "age": 10, "gender": "Female", "owner": &ids[1] }},
    ];

    data_sources.pets.insert_many(pets, None)
        .expect("Unable to insert data");

    println!("Data inserted");

    // putting this here to prevent dead code check issues
    let _schema: Arc<Schema> = std::sync::Arc::new(create_schema());
}
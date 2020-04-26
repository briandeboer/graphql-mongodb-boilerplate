use bson::doc;
use mongodb::Client;
use mongodb_base_service::DataSources;
use std::env;

pub fn connect() -> DataSources {
    // set up database connection pool
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
    let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set");
    let mut data_sources = DataSources::new();

    let client = Client::with_uri_str(&mongo_url)
        .expect("Failed to initialize client.")
        .database(&mongo_db_name);

    data_sources.create_mongo_service("owners", &client.collection("owners"), None);
    data_sources.create_mongo_service(
        "pets",
        &client.collection("pets"),
        // default sort is newest to oldest
        Some(doc! { "node.date_created": -1 }),
    );

    return data_sources;
}

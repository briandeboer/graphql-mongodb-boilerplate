use mongodb::{Client, Collection, Database};

#[derive(Clone)]
pub struct DataSources {
    pub owners: Collection,
    pub pets: Collection,
}

pub fn establish_connection() -> Database {
    Client::with_uri_str("mongodb://localhost:27017/")
        .expect("Failed to initialize client.")
        .database("mypets")
}

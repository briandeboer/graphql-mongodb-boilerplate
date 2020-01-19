use bson::{bson, doc, oid::ObjectId};
use mongodb_cursor_pagination::{Edge, PageInfo};
use serde::{Deserialize, Serialize};

use crate::graphql_schema::Context;
use crate::schema::common::Gender;
use crate::schema::pets::Pet;

#[derive(Clone, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: ObjectId,
    username: String,
    first_name: String,
    last_name: String,
    gender: Gender,
}

// notice that we do an impl version here because juniper doesn't know how to do a bson id
#[juniper::object(Context = Context, description = "A person who owns pets")]
impl Owner {
    fn id(&self) -> String {
        self.id.to_hex()
    }

    fn username(&self) -> String {
        self.username.to_owned()
    }

    fn first_name(&self) -> String {
        self.first_name.to_owned()
    }

    fn last_name(&self) -> String {
        self.last_name.to_owned()
    }

    fn gender(&self) -> Gender {
        self.gender
    }

    fn pets(&self, context: &Context) -> Option<Vec<Pet>> {
        let pets_collection = &context.data_sources.pets;
        // convert the juniper ID (string) into an object id
        let query_doc = doc! { "owner": self.id.to_hex() };
        let cursor = pets_collection.find(query_doc, None).unwrap();
        let mut pets: Vec<Pet> = vec![];
        for result in cursor {
            match result {
                Ok(doc) => {
                    let pet = bson::from_bson(bson::Bson::Document(doc.clone())).unwrap();
                    pets.push(pet);
                }
                Err(error) => {
                    println!("Error to find doc: {}", error);
                }
            }
        }
        Some(pets)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OwnerConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub owners: Vec<Owner>,
    pub total_count: i64,
}

#[juniper::object(Context = Context)]
impl OwnerConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn owners(&self) -> &Vec<Owner> {
        &self.owners
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewOwner {
    pub username: String,
    first_name: String,
    last_name: String,
    gender: Gender,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateOwner {
    /// Optional username to change the value to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Optional first_name to change the value to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Optional last_name to change the value to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// optional gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
}

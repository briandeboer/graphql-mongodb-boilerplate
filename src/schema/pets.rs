use bson::{bson, doc, oid::ObjectId};
use mongodb_cursor_pagination::{get_object_id, Edge, PageInfo};
use serde::{Deserialize, Serialize};

use crate::graphql_schema::Context;
use crate::schema::common::Gender;
use crate::schema::owners::Owner;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PetTypes {
    Cat,
    Dog,
    Fish,
    Hamster,
    Turtle,
}

#[derive(Serialize, Deserialize)]
pub struct Pet {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    _id: ObjectId,
    name: String,
    pet_type: PetTypes,
    age: Option<i32>,
    gender: Gender,
    owner: Option<juniper::ID>,
}

#[juniper::object(Context = Context, description = "A lovable pet")]
impl Pet {
    fn id(&self) -> juniper::ID {
        self._id.to_hex().into()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn pet_type(&self) -> PetTypes {
        self.pet_type
    }

    fn age(&self) -> Option<i32> {
        match self.age {
            Some(age) => Some(age as i32),
            None => None,
        }
    }

    fn gender(&self) -> Gender {
        self.gender
    }

    fn owner(&self, context: &Context) -> Option<Owner> {
        match &self.owner {
            None => None,
            Some(owner_id) => {
                let owners_coll = &context.data_sources.owners;
                // convert the juniper ID (string) into an object id
                let id = get_object_id(&owner_id.to_string()).unwrap();
                let owner_result = owners_coll
                    .find_one(doc! { "_id": id }, None)
                    .expect("Unable to connect to owners collection");
                match owner_result {
                    Some(owner_doc) => {
                        let owner: Owner =
                            bson::from_bson(bson::Bson::Document(owner_doc)).unwrap();
                        Some(owner)
                    }
                    None => None,
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PetConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub pets: Vec<Pet>,
    pub total_count: i64,
}

#[juniper::object(Context = Context, description = "A list of pets")]
impl PetConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn pets(&self) -> &Vec<Pet> {
        &self.pets
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewPet {
    pub name: String,
    pet_type: PetTypes,
    age: Option<i32>,
    gender: Gender,
    owner: Option<juniper::ID>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdatePet {
    /// Optional name to change the value to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional pet_type to change the value to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pet_type: Option<PetTypes>,

    /// optional age
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,

    /// optional gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,

    /// optional owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<juniper::ID>,
}

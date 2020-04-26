use chrono::{DateTime, Utc};
use log::warn;
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::common::Gender;
use crate::models::owners::Owner;

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PetTypes {
    Cat,
    Dog,
    Fish,
    Hamster,
    Turtle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Pet {
    pub node: NodeDetails,
    name: String,
    pet_type: PetTypes,
    age: Option<i32>,
    gender: Gender,
    owner: Option<ID>,
}

impl Node for Pet {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Clients, description = "A lovable pet")]
impl Pet {
    pub fn id(&self) -> juniper::ID {
        self.node.id().into()
    }

    fn date_created(&self) -> DateTime<Utc> {
        self.node.date_created()
    }

    fn date_modified(&self) -> DateTime<Utc> {
        self.node.date_modified()
    }

    fn created_by(&self) -> juniper::ID {
        self.node.created_by_id().into()
    }

    fn updated_by(&self) -> juniper::ID {
        self.node.updated_by_id().into()
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

    fn owner(&self, ctx: &Clients) -> Option<Owner> {
        match &self.owner {
            None => None,
            Some(owner_id) => {
                let service = &ctx.mongo.get_mongo_service("owners").unwrap();
                let result: Result<Option<Owner>, ServiceError> =
                    service.find_one_by_id(owner_id.clone());
                match result {
                    Ok(owner) => owner,
                    Err(e) => {
                        warn!("unable to retrieve owner by id {:?}", owner_id);
                        None
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PetConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Pet>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl PetConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Pet> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Pet>> for PetConnection {
    fn from(fr: FindResult<Pet>) -> PetConnection {
        PetConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
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

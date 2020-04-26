use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{BaseService, Node, NodeDetails, ServiceError};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::db::Clients;
use crate::models::common::Gender;
use crate::models::pets::Pet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub node: NodeDetails,
    username: String,
    first_name: String,
    last_name: String,
    gender: Gender,
}

impl Node for Owner {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

// notice that we do an impl version here because juniper doesn't know how to do a bson id
#[juniper::object(Context = Clients, description = "A person who owns pets")]
impl Owner {
    pub fn id(&self) -> juniper::ID {
        self.node.id().into()
    }

    fn date_created(&self) -> DateTime<Utc> {
        self.node.date_created()
    }

    fn date_modified(&self) -> DateTime<Utc> {
        self.node.date_modified()
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

    fn pets(&self, ctx: &Clients) -> Vec<Pet> {
        let service = &ctx.mongo.get_mongo_service("pets").unwrap();
        let filter = doc! { "owner": self.node.id.to_string() };
        let result: Result<FindResult<Pet>, ServiceError> =
            service.find(Some(filter), None, None, None, None, None);
        match result {
            Ok(all_items) => all_items.items,
            Err(e) => Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnerConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Owner>,
    pub total_count: i64,
}

#[juniper::object(Context = Clients)]
impl OwnerConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Owner> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Owner>> for OwnerConnection {
    fn from(fr: FindResult<Owner>) -> OwnerConnection {
        OwnerConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
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

use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

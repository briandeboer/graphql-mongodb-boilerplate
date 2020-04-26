use bson::doc;
use juniper::{FieldError, RootNode};
use mongodb_base_service::{BaseService, DeleteResponseGQL, ServiceError};
use mongodb_cursor_pagination::FindResult;

use crate::db::Clients;
use crate::models::*;

pub struct Query;

#[juniper::object(Context = Clients)]
impl Query {
    /// returns all pets, will only take one of "before", "after" or "skip"
    fn all_pets(
        ctx: &Clients,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<PetConnection, FieldError> {
        let service = &ctx.mongo.get_mongo_service("pets").unwrap();
        let result: Result<FindResult<Pet>, ServiceError> =
            service.find(None, None, limit, after, before, skip);
        match result {
            Ok(all_items) => {
                let connection: PetConnection = all_items.into();
                Ok(connection)
            }
            Err(e) => Err(FieldError::from(e)),
        }
    }

    fn pet_by_id(ctx: &Clients, id: juniper::ID) -> Result<Pet, FieldError> {
        let service = &ctx.mongo.get_mongo_service("pets").unwrap();
        let result: Result<Option<Pet>, ServiceError> = service.find_one_by_id(id.into());
        match result {
            Ok(item) => match item {
                Some(item) => Ok(item),
                None => Err("Unable to find item".into()),
            },
            Err(e) => Err(FieldError::from(e)),
        }
    }

    fn pets_by_type(
        ctx: &Clients,
        pet_type: Option<PetTypes>,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<PetConnection, FieldError> {
        let service = &ctx.mongo.get_mongo_service("pets").unwrap();
        let filter = match pet_type {
            Some(pt) => Some(doc! { "pet_type": format!("{:?}", pt) }),
            None => None,
        };
        let result: Result<FindResult<Pet>, ServiceError> =
            service.find(filter, None, limit, after, before, skip);
        match result {
            Ok(all_items) => {
                let connection: PetConnection = all_items.into();
                Ok(connection)
            }
            Err(e) => Err(FieldError::from(e)),
        }
    }

    fn all_owners(
        ctx: &Clients,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<OwnerConnection, FieldError> {
        let service = &ctx.mongo.get_mongo_service("owners").unwrap();
        let result: Result<FindResult<Owner>, ServiceError> =
            service.find(None, None, limit, after, before, skip);
        match result {
            Ok(all_items) => {
                let connection: OwnerConnection = all_items.into();
                Ok(connection)
            }
            Err(e) => Err(FieldError::from(e)),
        }
    }

    fn owner_by_id(ctx: &Clients, id: juniper::ID) -> Result<Owner, FieldError> {
        let service = &ctx.mongo.get_mongo_service("owner").unwrap();
        let result: Result<Option<Owner>, ServiceError> = service.find_one_by_id(id.into());
        match result {
            Ok(item) => match item {
                Some(item) => Ok(item),
                None => Err("Unable to find item".into()),
            },
            Err(e) => Err(FieldError::from(e)),
        }
    }
}

pub struct Mutation;

#[juniper::object(Context = Clients)]
impl Mutation {
    fn create_pet(
        ctx: &Clients,
        new_pet: NewPet,
        user_id: Option<juniper::ID>,
    ) -> Result<Pet, FieldError> {
        let service = &ctx.mongo.get_mongo_service("pets").unwrap();
        // don't insert if there's one with the same name and type
        let pet: Pet = service
            .insert_one(new_pet, user_id.map(|id| id.into()))
            .unwrap();
        Ok(pet)
    }

    fn update_pet(
        ctx: &Clients,
        id: juniper::ID,
        update_pet: UpdatePet,
        user_id: Option<juniper::ID>,
    ) -> Result<Pet, FieldError> {
        let service = &ctx.mongo.get_mongo_service("pets").unwrap();
        service
            .update_one(id.into(), update_pet, user_id.map(|id| id.into()))
            .map_err(|e| e.into())
    }

    fn delete_pet(ctx: &Clients, id: juniper::ID) -> Result<DeleteResponseGQL, FieldError> {
        let service = &ctx.mongo.get_mongo_service("pets").unwrap();
        match service.delete_one_by_id(id.into()) {
            Ok(result) => Ok(result.into()),
            Err(e) => Err(e.into()),
        }
    }

    fn create_owner(
        ctx: &Clients,
        new_owner: NewOwner,
        user_id: Option<juniper::ID>,
    ) -> Result<Owner, FieldError> {
        let service = &ctx.mongo.get_mongo_service("owners").unwrap();
        // don't insert if there's one with the same name and type
        let owner: Owner = service
            .insert_one(new_owner, user_id.map(|id| id.into()))
            .unwrap();
        Ok(owner)
    }

    fn update_owner(
        ctx: &Clients,
        id: juniper::ID,
        update_owner: UpdateOwner,
        user_id: Option<juniper::ID>,
    ) -> Result<Owner, FieldError> {
        let service = &ctx.mongo.get_mongo_service("owners").unwrap();
        service
            .update_one(id.into(), update_owner, user_id.map(|id| id.into()))
            .map_err(|e| e.into())
    }

    fn delete_owner(ctx: &Clients, id: juniper::ID) -> Result<DeleteResponseGQL, FieldError> {
        let service = &ctx.mongo.get_mongo_service("owners").unwrap();
        match service.delete_one_by_id(id.into()) {
            Ok(result) => Ok(result.into()),
            Err(e) => Err(e.into()),
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

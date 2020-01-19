use juniper::{FieldError, RootNode};

use crate::db::DataSources;
use crate::schema::owners::{NewOwner, Owner, OwnerConnection, UpdateOwner};
use crate::schema::pets::{NewPet, Pet, PetConnection, PetTypes, UpdatePet};
use crate::services;

#[derive(Clone)]
pub struct Context {
    pub data_sources: DataSources,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    /// returns all pets, will only take one of "before", "after" or "skip"
    fn all_pets(
        ctx: &Context,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<PetConnection, FieldError> {
        let pets: PetConnection = services::pets::get_pets(ctx, limit, after, before, skip)?;
        Ok(pets)
    }

    fn pet_by_id(ctx: &Context, id: juniper::ID) -> Result<Pet, FieldError> {
        let pet: Pet = services::pets::get_pet_by_id(ctx, id)?;
        Ok(pet)
    }

    fn pets_by_type(
        ctx: &Context,
        pet_type: PetTypes,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<PetConnection, FieldError> {
        let pets: PetConnection =
            services::pets::get_pets_by_type(ctx, pet_type, limit, after, before, skip)?;
        Ok(pets)
    }

    fn all_owners(
        ctx: &Context,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<OwnerConnection, FieldError> {
        let owners: OwnerConnection =
            services::owners::get_owners(ctx, limit, after, before, skip)?;
        Ok(owners)
    }

    fn owner_by_id(ctx: &Context, id: juniper::ID) -> Result<Owner, FieldError> {
        let owner: Owner = services::owners::get_owner_by_id(ctx, id)?;
        Ok(owner)
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_pet(ctx: &Context, new_pet: NewPet) -> Result<Pet, FieldError> {
        let pet: Pet = services::pets::create_pet(ctx, new_pet)?;
        Ok(pet)
    }

    fn update_pet(ctx: &Context, id: String, update_pet: UpdatePet) -> Result<Pet, FieldError> {
        let pet: Pet = services::pets::update_pet(ctx, id, update_pet)?;
        Ok(pet)
    }

    fn create_owner(ctx: &Context, new_owner: NewOwner) -> Result<Owner, FieldError> {
        let owner: Owner = services::owners::create_owner(ctx, new_owner)?;
        Ok(owner)
    }

    fn update_owner(
        ctx: &Context,
        id: String,
        update_owner: UpdateOwner,
    ) -> Result<Owner, FieldError> {
        let owner: Owner = services::owners::update_owner(ctx, id, update_owner)?;
        Ok(owner)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

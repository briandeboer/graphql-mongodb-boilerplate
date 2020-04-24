use crate::graphql_schema::Context;
use crate::schema::pets::{NewPet, Pet, PetConnection, PetTypes, UpdatePet};
use bson::doc;
use juniper::FieldError;
use log::warn;
use mongodb::options::FindOptions;
use mongodb_cursor_pagination::{get_object_id, CursorDirections, FindResult, PaginatedCursor};

const DEFAULT_LIMIT: i64 = 25;

pub fn get_pets(
    ctx: &Context,
    limit: Option<i32>,
    after: Option<String>,
    before: Option<String>,
    skip: Option<i32>,
) -> Result<PetConnection, FieldError> {
    // build the options object
    let find_options = FindOptions::builder()
        .limit(if let Some(l) = limit {
            l as i64
        } else {
            DEFAULT_LIMIT
        })
        .skip(if let Some(s) = skip { s as i64 } else { 0 })
        // TODO: make this not something arbitrary for testing purposes
        .sort(Some(doc! { "pet_type": 1, "age": 1 }))
        .build();
    let is_previous_query = before.is_some() && after.is_none();
    let query_cursor = if is_previous_query {
        PaginatedCursor::new(Some(find_options), before, Some(CursorDirections::Previous))
    } else {
        PaginatedCursor::new(Some(find_options), after, None)
    };
    let find_results: FindResult<Pet> = query_cursor.find(&ctx.data_sources.pets, None)?;

    Ok(PetConnection {
        page_info: find_results.page_info.into(),
        edges: find_results.edges.iter().map(|x| x.into()).collect(),
        pets: find_results.items,
        total_count: find_results.total_count,
    })
}

pub fn get_pets_by_type(
    ctx: &Context,
    pet_type: PetTypes,
    limit: Option<i32>,
    after: Option<String>,
    before: Option<String>,
    skip: Option<i32>,
) -> Result<PetConnection, FieldError> {
    // build the options object
    let find_options = FindOptions::builder()
        .limit(if let Some(l) = limit {
            l as i64
        } else {
            DEFAULT_LIMIT
        })
        .skip(if let Some(s) = skip { s as i64 } else { 0 })
        // TODO: make this not something arbitrary for testing purposes
        .sort(Some(doc! { "age": -1 }))
        .build();
    let is_previous_query = before.is_some() && after.is_none();
    let query_cursor = if is_previous_query {
        PaginatedCursor::new(Some(find_options), before, Some(CursorDirections::Previous))
    } else {
        PaginatedCursor::new(Some(find_options), after, None)
    };
    // FIX: There's got to be a cleaner way to do this using serde or bson
    let filter = doc! { "pet_type": format!("{:?}", pet_type) };
    let find_results: FindResult<Pet> = query_cursor.find(&ctx.data_sources.pets, Some(&filter))?;

    Ok(PetConnection {
        page_info: find_results.page_info.into(),
        edges: find_results.edges.iter().map(|x| x.into()).collect(),
        pets: find_results.items,
        total_count: find_results.total_count,
    })
}

pub fn get_pet_by_id(ctx: &Context, id: juniper::ID) -> Result<Pet, FieldError> {
    let coll = &ctx.data_sources.pets;
    let object_id = get_object_id(&id.to_string())?;

    let query = Some(doc! { "_id" => object_id });
    let pet_doc = coll.find_one(query, None)?.expect("Document not found");

    let pet = bson::from_bson(bson::Bson::Document(pet_doc))?;
    Ok(pet)
}

pub fn get_pet_by_name(ctx: &Context, name: String) -> Result<Pet, FieldError> {
    let coll = &ctx.data_sources.pets;
    let pet_doc = coll.find_one(Some(doc! { "name" => name }), None)?;

    match pet_doc {
        Some(pet) => Ok(bson::from_bson(bson::Bson::Document(pet))?),
        None => Err("Item not found".into()),
    }
}

pub fn create_pet(ctx: &Context, new_pet: NewPet) -> Result<Pet, FieldError> {
    let coll = &ctx.data_sources.pets;
    let serialized_member = bson::to_bson(&new_pet)?;

    // Check first that it doesn't already exist by this name...sort of arbitrary but worth showing
    match get_pet_by_name(ctx, new_pet.name) {
        // notice that name is public for this reason
        Err(_) => {
            // name doesn't already exist
            if let bson::Bson::Document(document) = serialized_member {
                let result = coll.insert_one(document, None)?; // Insert into a MongoDB collection
                let id = result.inserted_id;
                let pet_doc = coll
                    .find_one(Some(doc! { "_id" => id }), None)?
                    .expect("Document not found");

                let pet = bson::from_bson(bson::Bson::Document(pet_doc))?;
                Ok(pet)
            } else {
                warn!("Error converting the BSON object into a MongoDB document");
                Err("Error converting the BSON object into a MongoDB document".into())
            }
        }
        Ok(_) => Err("Pet name is already taken".into()),
    }
}

pub fn update_pet(ctx: &Context, id: String, update_pet: UpdatePet) -> Result<Pet, FieldError> {
    let object_id = get_object_id(&id)?;
    let search = doc! {"_id": object_id};
    let serialized_member = bson::to_bson(&update_pet)?;

    let coll = &ctx.data_sources.pets;
    match coll.update_one(search.clone(), doc! {"$set": serialized_member}, None) {
        Ok(_res) => match coll.find_one(Some(search), None) {
            Ok(res) => match res {
                Some(doc) => {
                    let pet = bson::from_bson(bson::Bson::Document(doc))?;
                    Ok(pet)
                }
                None => Err(FieldError::from("Unable to find pet".to_owned())),
            },
            Err(t) => {
                warn!("Search failed");
                Err(FieldError::from(t))
            }
        },
        Err(e) => Err(FieldError::from(e)),
    }
}

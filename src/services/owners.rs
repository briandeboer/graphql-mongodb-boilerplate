use crate::graphql_schema::Context;
use crate::schema::owners::{NewOwner, Owner, OwnerConnection, UpdateOwner};
use bson::{bson, doc};
use juniper::FieldError;
use log::warn;
use mongodb::options::FindOptions;
use mongodb_cursor_pagination::{get_object_id, CursorDirections, FindResult, PaginatedCursor};

const DEFAULT_LIMIT: i64 = 25;

pub fn get_owners(
    ctx: &Context,
    limit: Option<i32>,
    after: Option<String>,
    before: Option<String>,
    skip: Option<i32>,
) -> Result<OwnerConnection, FieldError> {
    // build the options object
    let find_options = FindOptions::builder()
        .limit(if let Some(l) = limit {
            l as i64
        } else {
            DEFAULT_LIMIT
        })
        .skip(if let Some(s) = skip { s as i64 } else { 0 })
        // TODO: make this not something arbitrary for testing purposes
        .sort(Some(doc! { "username": 1 }))
        .build();
    let is_previous_query = before.is_some() && after.is_none();
    let query_cursor = if is_previous_query {
        PaginatedCursor::new(Some(find_options), before, Some(CursorDirections::Previous))
    } else {
        PaginatedCursor::new(Some(find_options), after, None)
    };
    let find_results: FindResult<Owner> = query_cursor.find(&ctx.data_sources.owners, None)?;

    Ok(OwnerConnection {
        page_info: find_results.page_info.into(),
        edges: find_results.edges.iter().map(|x| x.into()).collect(),
        owners: find_results.items,
        total_count: find_results.total_count,
    })
}

pub fn get_owner_by_id(ctx: &Context, id: juniper::ID) -> Result<Owner, FieldError> {
    let coll = &ctx.data_sources.owners;
    let object_id = get_object_id(&id.to_string())?;

    let query = Some(doc! { "_id" => object_id });
    let owner_doc = coll.find_one(query, None)?.expect("Document not found");

    let owner = bson::from_bson(bson::Bson::Document(owner_doc))?;
    Ok(owner)
}

pub fn get_owner_by_username(ctx: &Context, username: String) -> Result<Owner, FieldError> {
    let coll = &ctx.data_sources.owners;
    let owner_doc = coll.find_one(Some(doc! { "username" => username }), None)?;

    match owner_doc {
        Some(owner) => Ok(bson::from_bson(bson::Bson::Document(owner))?),
        None => Err("Owner not found".into()),
    }
}

pub fn create_owner(ctx: &Context, new_owner: NewOwner) -> Result<Owner, FieldError> {
    let coll = &ctx.data_sources.owners;
    let serialized_member = bson::to_bson(&new_owner)?;

    // Check first that it doesn't already exist by this name...sort of arbitrary but worth showing
    match get_owner_by_username(ctx, new_owner.username) {
        // notice that username is public for this reason
        Err(_) => {
            // name doesn't already exist
            if let bson::Bson::Document(document) = serialized_member {
                let result = coll.insert_one(document, None)?; // Insert into a MongoDB collection
                let id = result.inserted_id;
                let owner_doc = coll
                    .find_one(Some(doc! { "_id" => id }), None)?
                    .expect("Document not found");

                let owner = bson::from_bson(bson::Bson::Document(owner_doc))?;
                Ok(owner)
            } else {
                warn!("Error converting the BSON object into a MongoDB document");
                Err("Error converting the BSON object into a MongoDB document".into())
            }
        }
        Ok(_) => Err("Owner username is already taken".into()),
    }
}

pub fn update_owner(
    ctx: &Context,
    id: String,
    update_owner: UpdateOwner,
) -> Result<Owner, FieldError> {
    let object_id = get_object_id(&id)?;
    let search = doc! {"_id": object_id};
    let serialized_member = bson::to_bson(&update_owner)?;

    let coll = &ctx.data_sources.owners;
    match coll.update_one(search.clone(), doc! {"$set": serialized_member}, None) {
        Ok(_res) => match coll.find_one(Some(search), None) {
            Ok(res) => match res {
                Some(doc) => {
                    let owner = bson::from_bson(bson::Bson::Document(doc))?;
                    Ok(owner)
                }
                None => Err(FieldError::from("Unable to find owner".to_owned())),
            },
            Err(t) => {
                warn!("Search failed");
                Err(FieldError::from(t))
            }
        },
        Err(e) => Err(FieldError::from(e)),
    }
}

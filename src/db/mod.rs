pub mod mongo;

use mongodb_base_service::DataSources;

#[derive(Clone)]
pub struct Clients {
    pub mongo: DataSources,
}
impl juniper::Context for Clients {}

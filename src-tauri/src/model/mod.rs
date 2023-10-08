mod store;
pub mod types;

use crate::Result;
use store::SurrealStore;

pub use store::{Castable, Creatable, Patchable};

// wrap the store to control granular exposure of internal methods
pub struct Store(SurrealStore);

impl Store {
    /// Create a new Store instance and its corresponding SurrealStore
    pub async fn new() -> Result<Self> {
        Ok(Store(SurrealStore::new().await?))
    }

    pub fn get(&self) -> &SurrealStore {
        &self.0
    }
}

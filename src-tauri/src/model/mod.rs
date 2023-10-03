mod store;
mod try_from;
pub mod types;

use crate::{Error::XPropertyNotFound, Result};
use store::SurrealStore;
use surrealdb::sql::{Object, Value};

pub use store::{Creatable, Patchable};
pub use try_from::W;

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

// helpers
pub fn take_object(mut val: Object, key: &str) -> Result<Object> {
    let v: Option<Result<Object>> = val.remove(key).map(|v| W(v).try_into());
    let val = match v {
        None => Ok(None),
        Some(Ok(val)) => Ok(Some(val)),
        Some(Err(ex)) => Err(ex),
    }?;
    val.ok_or_else(|| XPropertyNotFound(key.to_string()))
}

pub fn take_string(mut val: Object, key: &str) -> Result<String> {
    let v: Option<Result<String>> = val.remove(key).map(|v| W(v).try_into());
    let val = match v {
        None => Ok(None),
        Some(Ok(val)) => Ok(Some(val)),
        Some(Err(ex)) => Err(ex),
    }?;
    val.ok_or_else(|| XPropertyNotFound(key.to_string()))
}

pub fn take_bool(mut val: Object, key: &str) -> Result<bool> {
    val.remove(key)
        .map(|v| v.is_true())
        .ok_or_else(|| XPropertyNotFound(key.to_string()))
}

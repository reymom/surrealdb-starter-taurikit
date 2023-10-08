use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Thing, Value};
use ts_rs::TS;

// Record is used to deserialize returned ids
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

impl Record {
    pub fn get_table(&self) -> &str {
        &self.id.tb
    }

    pub fn get_id(&self) -> String {
        self.id.id.to_raw()
    }

    pub fn get_full_id(&self) -> String {
        self.id.to_raw()
    }
}

const PAGE_LIMIT: u8 = 100;

#[derive(Deserialize)]
pub struct Page {
    limit: u8,
    page: u32,
}

impl Page {
    pub fn new(limit: u8, page: u32) -> Self {
        Page { limit, page }
    }

    pub fn get_limit(&self) -> Result<u8> {
        if self.limit > PAGE_LIMIT {
            return Err(Error::ErrLimitOutOfBonds);
        }
        Ok(self.limit)
    }

    pub fn get_offset(&self) -> u32 {
        self.limit as u32 * self.page
    }
}

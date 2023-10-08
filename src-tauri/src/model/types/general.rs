use crate::{Error, Result};

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use ts_rs::TS;

// Record is used to deserialize returned ids
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: IdWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdWrapper(Thing);

impl IdWrapper {
    pub fn get_id(&self) -> String {
        self.0.id.to_raw()
    }

    pub fn get_full_id(&self) -> String {
        self.0.to_raw()
    }
}

const PAGE_LIMIT: u8 = 100;

#[derive(Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Page {
    pub limit: u8,
    pub page: u32,
}

impl Page {
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

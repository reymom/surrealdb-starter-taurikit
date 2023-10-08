use super::{Page, Record};
use crate::model::{Creatable, Patchable};
use crate::{Error, Result, Store};
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};
use ts_rs::TS;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Debug, Serialize, TS)]
struct Responsibility {
    marketing: bool,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Person {
    pub id: String,
    pub title: String,
    pub name: Name,
    pub marketing: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PersonForCreate {
    pub title: String,
    pub name: Name,
    pub marketing: Option<bool>,
}

impl Creatable for PersonForCreate {}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PersonForUpdate {
    pub title: Option<String>,
    pub name: Option<Name>,
    pub marketing: Option<bool>,
}

impl Patchable for PersonForUpdate {}

pub struct PersonController;

impl PersonController {
    const ENTITY: &'static str = "person";

    pub async fn get(store: Arc<Store>, id: &str) -> Result<Person> {
        store.get().exec_get(Self::ENTITY, id).await?.try_into()
    }

    pub async fn create(store: Arc<Store>, data: PersonForCreate) -> Result<String> {
        Ok(store
            .get()
            .exec_create(Self::ENTITY, data)
            .await?
            .get_full_id())
    }

    pub async fn update(store: Arc<Store>, id: &str, data: PersonForUpdate) -> Result<String> {
        Ok(store
            .get()
            .exec_update(Self::ENTITY, id, data)
            .await?
            .get_full_id())
    }

    pub async fn delete(store: Arc<Store>, id: &str) -> Result<String> {
        Ok(store
            .get()
            .exec_delete(Self::ENTITY, id)
            .await?
            .get_full_id())
    }

    pub async fn list(store: Arc<Store>, page: Option<Page>) -> Result<Vec<Person>> {
        let res = store.get().exec_list(Self::ENTITY, page).await?;
        res.into_iter().map(|o| o.try_into()).collect::<Result<_>>()
    }
}

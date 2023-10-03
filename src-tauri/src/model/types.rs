use crate::model::{Creatable, Patchable};
use crate::Store;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Thing, Value};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct Name {
    pub first: String,
    pub last: String,
}

impl From<Name> for Value {
    fn from(val: Name) -> Self {
        BTreeMap::from([
            ("first".into(), val.first.into()),
            ("last".into(), val.last.into()),
        ])
        .into()
    }
}

#[derive(Debug, Serialize, TS)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Person {
    pub id: String,
    pub title: String,
    pub name: Name,
    pub marketing: bool,
}

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PersonForCreate {
    pub title: String,
    pub name: Name,
    pub marketing: Option<bool>,
}

impl From<PersonForCreate> for Value {
    fn from(val: PersonForCreate) -> Self {
        BTreeMap::from([
            ("title".into(), val.title.into()),
            ("name".into(), val.name.into()),
            ("marketing".into(), val.marketing.unwrap_or(false).into()),
        ])
        .into()
    }
}

impl Creatable for PersonForCreate {}

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct PersonForUpdate {
    pub title: Option<String>,
    pub name: Option<Name>,
    pub marketing: Option<bool>,
}

impl From<PersonForUpdate> for Value {
    fn from(val: PersonForUpdate) -> Self {
        let mut data = BTreeMap::new();
        if let Some(title) = val.title {
            data.insert("title".into(), title.into());
        }
        if let Some(name) = val.name {
            data.insert("name".into(), name.into());
        }
        if let Some(marketing) = val.marketing {
            data.insert("marketing".into(), marketing.into());
        }
        Value::Object(data.into())
    }
}

impl Patchable for PersonForUpdate {}

pub struct PersonController;

impl PersonController {
    const ENTITY: &'static str = "person";

    pub async fn get(store: Arc<Store>, id: &str) -> Result<Person> {
        store.get().exec_get(id).await?.try_into()
    }

    pub async fn create(store: Arc<Store>, data: PersonForCreate) -> Result<String> {
        Ok(store.get().exec_create(Self::ENTITY, data).await?)
    }

    pub async fn update(store: Arc<Store>, id: &str, data: PersonForUpdate) -> Result<String> {
        Ok(store.get().exec_update(id, data).await?)
    }

    pub async fn delete(store: Arc<Store>, id: &str) -> Result<String> {
        Ok(store.get().exec_delete(id).await?)
    }

    pub async fn list(
        store: Arc<Store>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Person>> {
        let objects = store.get().exec_list(Self::ENTITY, limit, offset).await?;
        objects
            .into_iter()
            .map(|o| o.try_into())
            .collect::<Result<_>>()
    }
}

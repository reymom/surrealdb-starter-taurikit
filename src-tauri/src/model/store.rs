use std::collections::BTreeMap;

use crate::macros::map;
use crate::model::types::{Page, Record};
use crate::model::W;
use crate::{Error, Result};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::sql::{thing, Array, Object, Value};
use surrealdb::sql::{Id, Thing};
use surrealdb::{Response, Surreal};

pub struct SurrealStore {
    pub db: Surreal<Db>,
}

pub trait Creatable: Serialize {}
pub trait Patchable: Serialize {}

impl SurrealStore {
    pub(in crate::model) async fn new() -> Result<Self> {
        // Create database connection
        let db = Surreal::new::<Mem>(()).await?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;

        Ok(SurrealStore { db })
    }

    // pub(in crate::model) async fn exec_get(&self, tb: &str, tid: &str) -> Result<Object> {
    //     println!("[exec_get]");
    //     let res: surrealdb::Result<Option<Value>> = self.db.select((tb, tid)).await;
    //     println!("[exec_get] res = {:?}", res);
    //     // res.map(|o| W(o).try_into())
    //     //     .ok_or(Error::XValueNotFound((format!("{tb}:{tid}"))))?
    //     Ok(Object { 0: BTreeMap::new() })
    // }

    pub(in crate::model) async fn exec_get(&self, tb: &str, tid: &str) -> Result<Object> {
        let mut sql = String::from("SELECT * FROM type::thing($tb, $tid)");

        let mut bindings: BTreeMap<String, String> = map![
            "tb".into() => tb.into(),
            "tid".into() => tid.into()];

        let mut response = self.db.query(sql).bind((bindings)).await?.check()?;
        let array: Array = W(response.take(0)?).try_into()?;

        W(array
            .into_iter()
            .next()
            .ok_or(Error::XValueNotFound(format!("{tb}:{tid}")))?)
        .try_into()
    }

    pub(in crate::model) async fn exec_create<T: Creatable>(
        &self,
        tb: &str,
        data: T,
    ) -> Result<Record> {
        let res: Vec<Record> = self.db.create(tb).content(data).await?;
        res.into_iter()
            .next()
            .ok_or(Error::StoreFailToCreate(format!(
                "exec_create {tb} got empty result."
            )))
    }

    pub(in crate::model) async fn exec_update<T: Patchable>(
        &self,
        tb: &str,
        tid: &str,
        data: T,
    ) -> Result<Record> {
        let res: Option<Record> = self.db.update((tb, tid)).content(data).await?;
        res.ok_or(Error::StoreFailToCreate(format!(
            "exec_merge {tid} got empty result."
        )))
    }

    pub(in crate::model) async fn exec_delete(&self, tb: &str, tid: &str) -> Result<Record> {
        let res: Option<Record> = self.db.delete((tb, tid)).await?;
        res.ok_or(Error::StoreFailToCreate(format!(
            "exec_delete {tid} got empty result."
        )))
    }

    pub(in crate::model) async fn exec_list(
        &self,
        tb: &str,
        page: Option<Page>,
    ) -> Result<Vec<Object>> {
        let mut sql = String::from("SELECT * FROM type::table($tb)");

        // --- Apply the limit and offset
        if let Some(page) = page {
            let limit = page.get_limit()?.to_string();
            let offset = page.get_offset().to_string();
            sql.push_str(&format!(" LIMIT {limit} START {offset}"));
        }

        let mut response = self.db.query(sql).bind(("tb", tb)).await?;
        let array: Array = W(response.take(0)?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error::XValueNotOfType;
    use crate::model::types::{Name, Page, Person, PersonForCreate, PersonForUpdate};
    use crate::model::W;
    use crate::model::{Result, Store};
    use std::sync::Arc;
    use surrealdb::sql::Object;
    use tokio::sync::OnceCell;

    const PERSON_LENGTH: i8 = 4;
    static STORE_ONCE: OnceCell<Arc<Store>> = OnceCell::const_new();

    pub async fn init(store: Arc<Store>) -> Result<()> {
        for i in 1..=PERSON_LENGTH {
            let marketing = Some(i % 2 == 0);
            let name = Name {
                first: format!("name-first-{i}"),
                last: format!("name-last-{i}"),
            };
            let person = PersonForCreate {
                title: format!("person#{i}"),
                name,
                marketing,
            };
            let _ = store
                .get()
                .exec_create::<PersonForCreate>("person", person)
                .await?;
        }
        Ok(())
    }

    /// Initialize store once for this unit test group.
    /// Will panic if can't create store.
    async fn get_shared_test_store() -> Arc<Store> {
        STORE_ONCE
            .get_or_init(|| async {
                // create and seed the store
                let store = Store::new().await.unwrap();
                let store = Arc::new(store);

                init(store.clone()).await.unwrap();
                store
            })
            .await
            .clone()
    }

    #[tokio::test]
    async fn test_list_persons() -> anyhow::Result<()> {
        let store = get_shared_test_store().await;

        let max_page = PERSON_LENGTH / 10;
        let last_len = PERSON_LENGTH % 10;
        for i in 0..=max_page {
            let mut res = store
                .get()
                .exec_list("person", Some(Page::new(10, 0)))
                .await?;
            if i == max_page && last_len > 0 {
                assert_eq!(
                    res.len(),
                    last_len as usize,
                    "number of persons returned in page {}",
                    i
                );
                continue;
            }
            assert_eq!(res.len(), 10, "number of persons returned in page {}", i);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_update_person() -> anyhow::Result<()> {
        let store = get_shared_test_store().await;
        let res = store
            .get()
            .exec_list("person", Some(Page::new(1, 0)))
            .await?;
        let person = res.get(0);
        assert_eq!(person.is_some(), true);
        let person: Person = person.unwrap().clone().try_into()?;
        let id = person.id.clone();
        let first_name = person.name.first.clone();

        let update = PersonForUpdate {
            title: Some("new_title".to_string()),
            name: Some(Name {
                first: first_name,
                last: "update_last".to_string(),
            }),
            marketing: Some(false),
        };
        let (table, id) = id.split_once(':').unwrap();
        let ret_id = store.get().exec_update(table, id, update).await?;
        assert_eq!(ret_id.get_id(), *id);

        println!("ret_id = {:?}", ret_id);

        let health = store.get().db.health().await?;
        println!("health = {:?}", health);
        let updated_person: Person = store.get().exec_get("person", &id).await?.try_into()?;

        println!("updated_person = {:?}", updated_person);

        assert_ne!(person.name, updated_person.name);
        assert_eq!(updated_person.name.last, "update_last");

        Ok(())
    }

    // #[tokio::test]
    // async fn test_create_delete_person() -> anyhow::Result<()> {
    //     let store = get_shared_test_store().await;
    //     let name = Name {
    //         first: format!("first"),
    //         last: format!("last"),
    //     };
    //     let person = PersonForCreate {
    //         title: format!("title"),
    //         name,
    //         marketing: Some(false),
    //     };
    //     let id = store
    //         .get()
    //         .exec_create::<PersonForCreate>("person", person)
    //         .await?;

    //     let mut res = store.get().exec_list("person", None).await?;
    //     assert_eq!(res.len(), (PERSON_LENGTH + 1) as usize);

    //     let person: Person = store
    //         .get()
    //         .exec_get(id.get_table(), &id.get_id())
    //         .await?
    //         .try_into()?;
    //     assert_eq!(person.title, "title");

    //     let _ = store.get().exec_delete("person", &id.get_full_id()).await?;
    //     let res = store.get().exec_list("person", None).await?;
    //     assert_eq!(res.len(), PERSON_LENGTH as usize);

    //     let person = store.get().exec_get("person", &id.get_full_id()).await;
    //     assert_eq!(
    //         XValueNotOfType("Object").to_string(),
    //         person.unwrap_err().to_string()
    //     );

    //     Ok(())
    // }
}

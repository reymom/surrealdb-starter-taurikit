use super::types::{Page, Record};
use crate::{Error, Result};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

pub struct SurrealStore {
    pub db: Surreal<Db>,
}

pub trait Castable: DeserializeOwned {}
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

    pub(in crate::model) async fn exec_get<T: Castable>(&self, tb: &str, tid: &str) -> Result<T> {
        let res: Option<T> = self.db.select((tb, tid)).await?;
        res.ok_or(Error::XValueNotFound(format!("{tb}:{tid}")))
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
        res.ok_or(Error::StoreFailToPatch {
            method: "update".into(),
            tb: tb.into(),
            tid: tid.into(),
        })
    }

    pub(in crate::model) async fn exec_delete(&self, tb: &str, tid: &str) -> Result<Record> {
        let res: Option<Record> = self.db.delete((tb, tid)).await?;
        res.ok_or(Error::StoreFailToPatch {
            method: "delete".into(),
            tb: tb.into(),
            tid: tid.into(),
        })
    }

    pub(in crate::model) async fn exec_list<T: Castable>(
        &self,
        tb: &str,
        page: Option<Page>,
    ) -> Result<Vec<T>> {
        let mut sql = String::from("SELECT * FROM type::table($tb)");

        // --- Apply the limit and offset
        if let Some(page) = page {
            let limit = page.get_limit()?.to_string();
            let offset = page.get_offset().to_string();
            sql.push_str(&format!(" LIMIT {limit} START {offset}"));
        }

        let mut res = self.db.query(sql).bind(("tb", tb)).await?;
        Ok(res.take(0)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error::XValueNotFound;
    use crate::model::types::{
        Name, Page, Person, PersonForCreate, PersonForUpdate, PersonMapping,
    };
    use crate::model::{Result, Store};
    use std::sync::Arc;
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
    async fn test_a_list_persons() -> anyhow::Result<()> {
        let store = get_shared_test_store().await;

        //get results by page
        let max_page = PERSON_LENGTH / 10;
        let last_len = PERSON_LENGTH % 10;
        for i in 0..=max_page {
            let res = store
                .get()
                .exec_list::<PersonMapping>(
                    "person",
                    Some(Page {
                        limit: 10,
                        page: i as u32,
                    }),
                )
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

        //get all results
        let res = store
            .get()
            .exec_list::<PersonMapping>("person", None)
            .await?;
        assert_eq!(
            res.len(),
            PERSON_LENGTH as usize,
            "number of persons returned in total"
        );
        let persons = res
            .into_iter()
            .map(|o| o.try_into())
            .collect::<Result<Vec<Person>>>()?;
        assert_eq!(
            persons.len(),
            PERSON_LENGTH as usize,
            "list of persons converted from mapping"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_b_update_person() -> anyhow::Result<()> {
        let store = get_shared_test_store().await;

        let res = store
            .get()
            .exec_list::<PersonMapping>("person", Some(Page { limit: 10, page: 0 }))
            .await?;
        let person = res.get(0);
        assert_eq!(person.is_some(), true);
        let person = person.unwrap();
        let id = person.id.get_id();
        let first_name = person.name.first.clone();

        let update = PersonForUpdate {
            title: Some("new_title".to_string()),
            name: Some(Name {
                first: first_name,
                last: "update_last".to_string(),
            }),
            marketing: Some(false),
        };
        let ret_id = store
            .get()
            .exec_update("person", id.as_str(), update)
            .await?;
        assert_eq!(ret_id.id.get_id(), *id);

        let _ = store.get().db.health().await?;
        let updated_person: Person = store
            .get()
            .exec_get::<PersonMapping>("person", &id)
            .await?
            .try_into()?;

        assert_ne!(person.name, updated_person.name);
        assert_eq!(updated_person.name.last, "update_last");

        Ok(())
    }

    #[tokio::test]
    async fn test_c_create_delete_person() -> anyhow::Result<()> {
        let store = get_shared_test_store().await;

        let name = Name {
            first: format!("first"),
            last: format!("last"),
        };
        let person = PersonForCreate {
            title: format!("title"),
            name,
            marketing: Some(false),
        };
        let id = store
            .get()
            .exec_create::<PersonForCreate>("person", person)
            .await?;

        let res = store
            .get()
            .exec_list::<PersonMapping>("person", None)
            .await?;
        assert_eq!(res.len(), (PERSON_LENGTH + 1) as usize);

        let person: Person = store
            .get()
            .exec_get::<PersonMapping>("person", &id.id.get_id())
            .await?
            .try_into()?;
        assert_eq!(person.title, "title");

        let _ = store.get().exec_delete("person", &&id.id.get_id()).await?;
        let res = store
            .get()
            .exec_list::<PersonMapping>("person", None)
            .await?;
        assert_eq!(res.len(), PERSON_LENGTH as usize);

        let person = store
            .get()
            .exec_get::<PersonMapping>("person", &id.id.get_id())
            .await;
        assert_eq!(
            XValueNotFound(format!("{}", id.id.get_full_id())).to_string(),
            person.unwrap_err().to_string()
        );

        Ok(())
    }
}

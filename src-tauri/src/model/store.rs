use crate::macros::map;
use crate::model::{take_string, W};
use crate::{Error, Result};
use std::collections::BTreeMap;
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Array, Object, Value};

pub struct SurrealStore {
    ds: Datastore,
    ses: Session,
}

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

impl SurrealStore {
    pub async fn new() -> Result<Self> {
        let ds = Datastore::new("memory").await?;
        let ses = Session::for_db("appns", "appdb");
        Ok(SurrealStore { ds, ses })
    }

    pub async fn exec_get(&self, tid: &str) -> Result<Object> {
        let sql = "SELECT * FROM $th";

        let vars = map!["th".into() => thing(tid)?.into()];

        let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub(in crate::model) async fn exec_create<T: Creatable>(
        &self,
        tb: &str,
        data: T,
    ) -> Result<String> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

        let mut data: Object = W(data.into()).try_into()?;

        let vars = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

        let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        if let Value::Object(mut val) = first_val.first() {
            take_string(val, "id")
                .map_err(|ex| Error::StoreFailToCreate(format!("exec_create {tb} {ex}")))
        } else {
            Err(Error::StoreFailToCreate(format!(
                "exec_create {tb}, nothing returned."
            )))
        }
    }

    pub(in crate::model) async fn exec_update<T: Patchable>(
        &self,
        tid: &str,
        data: T,
    ) -> Result<String> {
        let sql = "UPDATE $th MERGE $data RETURN id";

        let vars = map![
			"th".into() => thing(tid)?.into(),
			"data".into() => data.into()];

        let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        let result = first_res.result?;

        if let Value::Object(mut val) = result.first() {
            take_string(val, "id")
        } else {
            Err(Error::StoreFailToCreate(format!(
                "exec_merge {tid}, nothing returned."
            )))
        }
    }

    pub async fn exec_delete(&self, tid: &str) -> Result<String> {
        let sql = "DELETE $th";

        let vars = map!["th".into() => thing(tid)?.into()];

        let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");
        // Return the error if result failed
        first_res.result?;

        // return success
        Ok(tid.to_string())
    }

    pub async fn exec_list(
        &self,
        tb: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Object>> {
        let mut sql = String::from("SELECT * FROM type::table($tb)");

        // --- Apply the limit
        if let Some(limit) = limit {
            sql.push_str(&format!(" LIMIT {limit}"));
        }

        // --- Apply the offset
        if let Some(offset) = offset {
            sql.push_str(&format!(" START {offset}"));
        }

        let vars = BTreeMap::from([("tb".into(), tb.into())]);
        let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        // Get the result value as value array (fail if it is not)
        let array: Array = W(first_res.result?).try_into()?;

        // build the list of objects
        array.into_iter().map(|value| W(value).try_into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error::XValueNotOfType;
    use crate::model::types::{Name, Person, PersonForCreate, PersonForUpdate};
    use crate::model::{take_bool, take_object, take_string, Result, Store};
    use std::sync::Arc;
    use tokio::sync::OnceCell;

    static STORE_ONCE: OnceCell<Arc<Store>> = OnceCell::const_new();

    pub async fn init(store: Arc<Store>) -> Result<()> {
        for i in 1..=50 {
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
        let mut res = store.get().exec_list("person", Some(10), Some(0)).await?;
        assert_eq!(res.len(), 10, "number of persons returned in page 1");
        let mut res2 = store.get().exec_list("person", Some(10), Some(10)).await?;
        assert_eq!(res.len(), 10, "number of persons returned in page 2");
        assert_ne!(res, res2);
        let mut res = store.get().exec_list("person", Some(100), None).await?;
        assert_eq!(res.len(), 50, "number of persons returned");
        let mut res = store.get().exec_list("person", None, None).await?;
        assert_eq!(res.len(), 50, "total number of persons returned");

        Ok(())
    }

    #[tokio::test]
    async fn test_update_person() -> anyhow::Result<()> {
        let store = get_shared_test_store().await;
        let res = store.get().exec_list("person", Some(1), None).await?;
        let id = take_string(res[0].clone(), "id")?;

        let name_obj = take_object(res[0].clone(), "name")?;
        let first_name = take_string(name_obj, "first")?;

        let update = PersonForUpdate {
            title: Some("new_title".to_string()),
            name: Some(Name {
                first: first_name.clone(),
                last: "update_second".to_string(),
            }),
            marketing: None,
        };
        let ret_id = store.get().exec_update(&id, update).await?;
        assert_eq!(ret_id, id);

        let person = store.get().exec_get(&id).await?;
        assert_ne!(person, res[0]);
        let name_obj = take_object(person.clone(), "name")?;
        let updt_first_name = take_string(name_obj.clone(), "first")?;
        let updt_last_name = take_string(name_obj, "last")?;
        assert_eq!(first_name, updt_first_name);
        assert_eq!(updt_last_name, "update_second");

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_person() -> anyhow::Result<()> {
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

        let mut res = store.get().exec_list("person", None, None).await?;
        assert_eq!(res.len(), 51);

        let person = store.get().exec_get(&id).await?;
        let title = take_string(person.clone(), "title")?;
        assert_eq!(title, "title");

        let _ = store.get().exec_delete(&id).await?;
        let res = store.get().exec_list("person", None, None).await?;
        assert_eq!(res.len(), 50);

        let person = store.get().exec_get(&id).await;
        assert_eq!(
            XValueNotOfType("Object").to_string(),
            person.unwrap_err().to_string()
        );

        Ok(())
    }
}

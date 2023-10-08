use crate::model::types::Page;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateParams<D> {
    pub data: D,
}

#[derive(Deserialize)]
pub struct UpdateParams<D> {
    pub id: String,
    pub data: D,
}

#[derive(Deserialize)]
pub struct DeleteParams {
    pub id: String,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub page: Option<Page>,
}

#[derive(Deserialize)]
pub struct GetParams {
    pub id: String,
}

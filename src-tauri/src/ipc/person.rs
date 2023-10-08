use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::model::types::{Person, PersonController, PersonForCreate, PersonForUpdate};
use crate::model::Store;

use tauri::{command, AppHandle, Manager, Wry};

use std::sync::Arc;

#[command]
pub async fn get_person(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Person> {
    let store = (*app.state::<Arc<Store>>()).clone();
    PersonController::get(store, &params.id).await.into()
}

#[command]
pub async fn create_person(
    app: AppHandle<Wry>,
    params: CreateParams<PersonForCreate>,
) -> IpcResponse<String> {
    let store = (*app.state::<Arc<Store>>()).clone();
    PersonController::create(store, params.data).await.into()
}

#[command]
pub async fn update_person(
    app: AppHandle<Wry>,
    params: UpdateParams<PersonForUpdate>,
) -> IpcResponse<String> {
    let store = (*app.state::<Arc<Store>>()).clone();
    PersonController::update(store, &params.id, params.data)
        .await
        .into()
}

#[command]
pub async fn delete_person(app: AppHandle<Wry>, params: DeleteParams) -> IpcResponse<String> {
    let store = (*app.state::<Arc<Store>>()).clone();
    PersonController::delete(store, &params.id).await.into()
}

#[command]
pub async fn list_persons(app: AppHandle<Wry>, params: ListParams) -> IpcResponse<Vec<Person>> {
    let store = (*app.state::<Arc<Store>>()).clone();

    PersonController::list(store, params.page).await.into()
}

use crate::{
    model::user::JsonCreateUser,
    module::{Modules, ModulesExt},
};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::error;

#[tracing::instrument(skip(modules))]
pub async fn create(
    Extension(modules): Extension<Arc<Modules>>,
    Json(source): Json<JsonCreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_usecase().create(source.into()).await;

    res.map(|_| StatusCode::CREATED).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

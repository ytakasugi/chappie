use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use tracing::error;

use crate::{
    model::user::JsonCreateUser,
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn create(
    Json(source): Json<JsonCreateUser>,
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_usecase().create(source.into()).await;

    res.map(|_| StatusCode::CREATED).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

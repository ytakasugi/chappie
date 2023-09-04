use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use tracing::error;

use crate::{
    model::user_project::JsonCreateUserProject,
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn create(
    Extension(modules): Extension<Arc<Modules>>,
    Json(source): Json<JsonCreateUserProject>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_project_usecase().create(source.into()).await;

    res.map(|_| StatusCode::CREATED).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

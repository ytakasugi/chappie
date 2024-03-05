use crate::{
    model::user_project::JsonCreateUserProject,
    module::{Modules, ModulesExt},
};
use axum::{extract::{Extension, Path}, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::error;

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

#[tracing::instrument(skip(modules))]
pub async fn delete(
    Extension(modules): Extension<Arc<Modules>>,
    Path((user_id, project_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .user_project_usecase()
        .delete(user_id, project_id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

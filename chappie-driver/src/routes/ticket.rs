use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::error;

use crate::{
    model::ticket::{JsonCreateTicket, JsonTicketView},
    module::{Modules, ModulesExt},
};

#[tracing::instrument(skip(modules))]
pub async fn create(
    Extension(modules): Extension<Arc<Modules>>,
    Json(source): Json<JsonCreateTicket>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.ticket_usecase().create(source.into()).await;

    res.map(|_| StatusCode::CREATED).map_err(|err| {
        error!("Unexpected error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip(modules))]
pub async fn find(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.ticket_usecase().find(id).await;

    match res {
        Ok(view) => view
            .map(|view| {
                let json: JsonTicketView = view.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or(StatusCode::NOT_FOUND),
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

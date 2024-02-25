use crate::{
    model::ticket::{JsonCreateTicket, JsonTicketListView, JsonTicketView},
    module::{Modules, ModulesExt},
};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tracing::error;

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

#[tracing::instrument(skip(modules))]
pub async fn list(
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.ticket_usecase().list().await;

    match res {
        Ok(ticket_list) => match ticket_list {
            Some(ticket_list) => {
                let ticket = ticket_list.into_iter().map(|t| t.into()).collect();
                let json: JsonTicketListView = JsonTicketListView::new(ticket);
                Ok((StatusCode::OK, Json(json)))
            }
            None => {
                let json: JsonTicketListView = JsonTicketListView::new(vec![]);
                Ok((StatusCode::OK, Json(json)))
            }
        },
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

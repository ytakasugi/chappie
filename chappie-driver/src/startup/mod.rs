use crate::{module::Modules, routes::{user, porject, ticket}};
use axum::{extract::Extension, routing::post, Router};

use std::{net::SocketAddr, sync::Arc};

use crate::util::logger;

pub async fn startup(modules: Arc<Modules>) {
    logger::init();

    let user_router = Router::new().route("/", post(user::create));
    let project_router = Router::new().route("/", post(porject::create));
    let ticket_router = Router::new().route("/", post(ticket::create));


    let app = Router::new()
        .nest("/users", user_router)
        .nest("/projects", project_router)
        .nest("/tickets", ticket_router)
        .layer(Extension(modules));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

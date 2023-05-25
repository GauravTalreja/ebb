#![cfg(engine)]

use axum::{routing::get, Router};
use perseus::{
    i18n::TranslationsManager, server::ServerOptions, stores::MutableStore, turbine::Turbine,
};
use std::net::SocketAddr;

pub async fn main<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let app = get_router(turbine, opts).await;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_router<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
) -> Router {
    let api_routes = Router::new().route("/status", get(hello_world));

    let combined_router = perseus_axum::get_router(turbine, opts)
        .await
        .nest("/api", api_routes);

    combined_router
}

async fn hello_world() -> &'static str {
    "hello world!"
}

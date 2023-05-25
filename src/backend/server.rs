use std::net::SocketAddr;
use std::num::{NonZeroU16, NonZeroUsize};

use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::{routing, Extension, Router, Server};
use perseus::i18n::TranslationsManager;
use perseus::server::ServerOptions;
use perseus::stores::MutableStore;
use perseus::turbine::Turbine;
use tower_http::cors::{Any, CorsLayer};

use crate::backend::http;
use crate::backend::storage::{CourseStore, MySqlClient};
use crate::config::DatabaseConfig;

pub async fn main<M, T>(
    turbine: &'static Turbine<M, T>,
    options: ServerOptions,
    (host, port): (String, u16),
) where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let api_router = create_router(turbine, options).await;

    Server::bind(&addr)
        .serve(api_router.into_make_service())
        .await
        .unwrap();
}

async fn create_router<M, T>(turbine: &'static Turbine<M, T>, opts: ServerOptions) -> Router
where
    M: MutableStore + 'static,
    T: TranslationsManager + 'static,
{
    let mysql_client = MySqlClient::new(&DatabaseConfig {
        username: "root".to_string(),
        password: "password".to_string(),
        hostname: "127.0.0.1".to_string(),
        database: "ebbjustin".to_string(),
        port: NonZeroU16::new(3306).unwrap(),
        pool_size: NonZeroUsize::new(16).unwrap(),
    })
    .await
    .expect("cannot connect to mysql database");
    let course_store = CourseStore::new(mysql_client);

    let cors_options = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);
    let api_routes = Router::new()
        .route("/status", routing::get(hello_world))
        .route("/courses/:course_name", routing::get(http::list_courses))
        .layer(Extension(course_store))
        .layer(cors_options);
    let combined_router = perseus_axum::get_router(turbine, opts)
        .await
        .nest("/api/v1", api_routes);

    combined_router
}

async fn hello_world() -> &'static str {
    "hello world!"
}

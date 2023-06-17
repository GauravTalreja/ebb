use crate::backend::http;
use crate::backend::open_api;
use crate::backend::storage::CourseStore;
use crate::backend::storage::StorageConfig;
use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing, Extension, Router, Server,
};
use perseus::web_log;
use perseus::{
    i18n::TranslationsManager, server::ServerOptions, stores::MutableStore, turbine::Turbine,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

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
    dotenv::dotenv().expect("Couldn't find a .env file in the proct root");

    let pool =
        sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_ENV url not found"))
            .await
            .expect("Could not connect to database.");

    let configuration = open_api::configuration();
    let storage_configuration = StorageConfig::new(std::env::var("STORAGE_MODE").expect("STORAGE_MODE not found"));
    web_log!(
        "The current term is {:#?}",
        openapi::apis::terms_api::v3_terms_current_get(&configuration).await
    );

    let course_store = CourseStore::new(pool, storage_configuration);

    let cors_options = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let api_routes = Router::new()
        .route("/status", routing::get(hello_world))
        .route("/courses/:course_name", routing::get(http::list_courses))
        .layer(Extension(course_store))
        .layer(cors_options);

    perseus_axum::get_router(turbine, opts)
        .await
        .nest("/api/v1", api_routes)
}

async fn hello_world() -> &'static str {
    "hello world!"
}

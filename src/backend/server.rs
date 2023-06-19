use std::net::SocketAddr;

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing, Extension, Router, Server,
};
use perseus::{
    i18n::TranslationsManager, server::ServerOptions, stores::MutableStore, turbine::Turbine,
};
use tower_http::cors::{Any, CorsLayer};

use crate::backend::http;
use crate::backend::storage::CourseStore;

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
    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL"))
        .await
        .expect("Could not connect to database.");

    let course_store = CourseStore::new(pool);

    let cors_options = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);
    let api_routes = Router::new()
        .route("/status", routing::get(hello_world))
        .route("/course/:course_code", routing::get(http::get_course))
        .route("/courses/:course_code", routing::get(http::list_courses))
        .layer(Extension(course_store))
        .layer(cors_options);

    perseus_axum::get_router(turbine, opts)
        .await
        .nest("/api/v1", api_routes)
}

async fn hello_world() -> &'static str {
    "hello world!"
}

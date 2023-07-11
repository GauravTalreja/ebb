use crate::backend::{http, open_api};
use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing, Extension, Router, Server,
};
use perseus::{
    i18n::TranslationsManager, server::ServerOptions, stores::MutableStore, turbine::Turbine,
    web_log,
};
use production;
use sample;
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
    dotenvy::dotenv().expect("Couldn't find a .env file in the project root");

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");

    let openapi_config = open_api::configuration();
    web_log!(
        "The current term is {:#?}",
        openapi::apis::terms_api::v3_terms_current_get(&openapi_config).await
    );

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
    let store = match std::env::var("STORAGE_MODE")
        .expect("STORAGE_MODE must be defined in the .env file")
        .as_str()
    {
        "PROD" => production::prod_store().await,
        "SAMPLE" => sample::sample_store().await,
        _ => unimplemented!(),
    };

    let cors_options = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let api_routes = Router::new()
        .route("/status", routing::get(hello_world))
        .route("/courses/:course_code", routing::get(http::list_courses))
        .route("/course/:course_code", routing::get(http::get_course))
        .route("/course_offerings/:course_code", routing::get(http::list_course_offerings))
        .layer(Extension(store))
        .layer(cors_options);

    perseus_axum::get_router(turbine, opts)
        .await
        .nest("/api/v1", api_routes)
}

async fn hello_world() -> &'static str {
    "hello world!"
}

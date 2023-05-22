use std::error::Error;
use std::path::Path;

use clap::Parser;
use perseus::plugins::Plugins;
use perseus::prelude::*;
use tokio::runtime::{Builder, Runtime};
use tokio::task::LocalSet;

use crate::config::{AppArgs, AppConfig};
use crate::storage::{CourseStore, MySqlClient};

mod config;
mod error_views;
pub mod models;
mod storage;

/// Creates and starts the default Perseus server with Axum. This should be run
/// in a `main` function annotated with `#[tokio::main]` (which requires the
/// `macros` and `rt-multi-thread` features on the `tokio` dependency).
#[cfg(feature = "dflt-server")]
pub async fn dflt_server<M: MutableStore + 'static, T: TranslationsManager + 'static>(
    turbine: &'static Turbine<M, T>,
    opts: ServerOptions,
    (host, port): (String, u16),
) {
    use std::net::SocketAddr;

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address provided to bind to.");
    let app = get_router(turbine, opts).await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    // TODO: replace with logger
    println!("starting ebb...");
    let args = AppArgs::parse();

    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("cannot create Tokio runtime")
        .block_on(async move {
            let local_thread = LocalSet::new();
            local_thread.run_until(run(&args)).await
        })
        .expect("cannot run ebb backed server");

    PerseusApp::new()
        .plugins(Plugins::new().plugin(
            perseus_tailwind::get_tailwind_plugin,
            perseus_tailwind::TailwindOptions {
                in_file: "src/tailwind.css".into(),
                out_file: "dist/tailwind.css".into(),
            },
        ))
        .static_alias("/tailwind.css", "dist/tailwind.css")
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(crate::error_views::get_error_views())
}

async fn run(args: &AppArgs) -> Result<(), Box<dyn Error>> {
    let config = config::read::<AppConfig>(Path::new(&args.config_file_path))?;

    let mysql_client = MySqlClient::new(&config.database).await?;
    let course_store = CourseStore::new(mysql_client).await?;
    course_store.add_courses(&Vec::new())?;

    Ok(())
}

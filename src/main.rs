use perseus::plugins::Plugins;
use perseus::prelude::*;

mod components;
mod config;
mod error_views;
mod models;
mod templates;
mod backend;

#[perseus::main(backend::server::main)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .plugins(Plugins::new().plugin(
            perseus_tailwind::get_tailwind_plugin,
            perseus_tailwind::TailwindOptions {
                in_file: "src/tailwind.css".into(),
                out_file: "dist/tailwind.css".into(),
            },
        ))
        .static_alias("/tailwind.css", "dist/tailwind.css")
        .template(templates::index::get_template())
        .template(templates::about::get_template())
        .error_views(error_views::get_error_views())
}

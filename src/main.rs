mod components;
mod error_views;
mod server;
mod templates;

use perseus::plugins::Plugins;
use perseus::prelude::*;

#[perseus::main(server::main)]
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
        .template(crate::templates::index::get_template())
        .template(crate::templates::about::get_template())
        .error_views(crate::error_views::get_error_views())
}

use crate::components::{footer::Footer, search_bar::SearchBar};
use crate::global_state::AppStateRx;
use models::{CourseSummary, LastUpdated};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// TODO: This should be global (maybe?)
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
pub struct IndexPageState {
    search_input: String,
    search_results: Vec<CourseSummary>,
    last_updated_time: LastUpdated,
}

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPageStateRx) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);

    #[cfg(client)]
    spawn_local_scoped(cx, async {
        let body = reqwasm::http::Request::get("/api/v1/last_updated_time")
        .send()
        .await
        .unwrap()
        .json::<LastUpdated>()
        .await
        .unwrap();
        state.last_updated_time.set(body);
    });

    view! { cx, div (data-theme=global_state.theme) {
        link ( rel="stylesheet", href="/tailwind.css")

        div (class="hero min-h-screen bg-base-300") {
            div (class="hero-content text-center") {
                div (class="max-w-7xl") {
                    h1 (class="text-5xl font-bold") { "UW Ebb" }
                    p (class="py-6") {
                        "Explore thousands of courses offered by the University of \
                        Waterloo. Plan your courses. Get Recommendations."
                    }
                    SearchBar (input=&state.search_input, results=&state.search_results)
                }
            }
        }
        Footer(last_updated_time=&state.last_updated_time)
    }}
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        search_input: "".to_string(),
        search_results: vec![],
        last_updated_time: LastUpdated { date_time: None },
    }
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "UW Ebb" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}

use crate::components::search_bar::SearchBar;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[cfg(client)]
use crate::models::Course;

// TODO: This should be global (maybe?)
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
pub struct IndexPageState {
    search_input: String,
    search_results: Vec<String>,
}

fn index_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a IndexPageStateRx) -> View<G> {
    #[cfg(client)]
    create_effect_scoped(cx, |cx| {
        if !state.search_input.get().is_empty() {
            spawn_local_scoped(cx, async {
                let body = reqwasm::http::Request::get(
                    format!(
                        "https://ebb.csclub.cloud/api/v1/courses/{}",
                        state.search_input.get()
                    )
                    .as_str(),
                )
                .send()
                .await
                .unwrap()
                .json::<Vec<Course>>()
                .await
                .unwrap()
                .iter()
                .map(|course| course.name.clone())
                .collect();
                state.search_results.set(body);
            })
        }
    });

    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        div (class="hero min-h-screen bg-base-200") {
            div (class="hero-content text-center") {
                div (class="max-w-7xl") {
                    h1 (class="text-5xl font-bold") { "UW Ebb" }
                    p (class="py-6") {"Explore thousands of courses offered by the University of Waterloo. Plan your courses. Get Recommendations."}
                    SearchBar (input=&state.search_input, results=&state.search_results)
                }
            }
        }
        footer (class="footer items-center p-4 bg-neutral text-neutral-content") {
            div (class="items-center grid-flow-col") {
                p {"Made for CS 348 with ♥"}
            }
            div (class="grid-flow-col gap-4 md:place-self-center md:justify-self-end") {
                a (class ="link link-hover", href = "about", id = "about-link") {"About"}
            }
        }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        search_input: "".to_string(),
        search_results: vec![],
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

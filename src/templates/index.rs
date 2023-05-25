use std::collections::HashMap;
use std::string::String;

use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::components::search_bar::SearchBar;
use crate::models::{Course, ListCourseResponse};

// TODO: This should be global (maybe?)
#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
pub struct IndexPageState {
    search_input: String,
    search_results: Vec<String>,
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
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
    // TODO: fix api requests to backend endpoint for list of courses.
    // let found_courses = reqwest::get("http://localhost:8080/api/v1/courses/<user search input here>")
    //     .await
    //     .expect("cannot make api request for courses")
    //     .json::<ListCourseResponse>()
    //     .await
    //     .expect("cannot make api request for courses");

    IndexPageState {
        search_input: "".to_string(),
        search_results: vec![],
        // TODO: surface all courses in search text bar.
        // found_courses
        //     .courses
        //     .iter()
        //     .map(|course| course.name.clone())
        //     .collect::<Vec<String>>(),
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

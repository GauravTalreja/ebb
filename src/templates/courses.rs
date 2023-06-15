use crate::components::layout::{Layout, SearchBarProps};
use crate::components::course_table::CourseTable;
use crate::components::filter::Filter;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CoursesStateRx")]
pub struct CoursesState {
    search_input: String,
    search_results: Vec<String>,
}

fn courses_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a CoursesStateRx) -> View<G> {
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };
    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout (search_bar=search_bar_props) {
            div (class="w-full px-8 h-24 bg-primary relative") {
                p(class="absolute bottom-3 font-bold text-2xl text-primary-content") {"Result for testing"}
            }
            div (class="flex justify-center w-full") {
                div (class="py-6 flex flex-row gap-2 w-5/6 ") {
                    div (class = "flex-initial w-2/3 py-4 px-8") {
                        CourseTable()
        
                    }
                    div (class="flex-1 w-1/6 py-4 px-8 ") {
                        div (class="shadow-lg rounded-md") {
                            Filter()
                        }
                    }
                }
            }
           
               
            
        }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> CoursesState {
    CoursesState {
        search_input: "".to_string(),
        search_results: vec![],
    }
}

#[engine_only_fn]
fn head(cx: Scope, _state: CoursesState) -> View<SsrNode> {
    view! { cx,
        title { "Search for courses" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("courses")
        .build_state_fn(get_build_state)
        .view_with_state(courses_page)
        .head_with_state(head)
        .build()
}

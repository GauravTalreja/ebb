use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

use crate::components::course_table::CourseTable;
use crate::components::filter::Filter;
use crate::components::layout::{Layout, SearchBarProps, ThemeProps};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CoursesStateRx")]
pub struct CoursesState {
    search_input: String,
    search_results: Vec<String>,
}

fn courses_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a CoursesStateRx) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let theme_props = ThemeProps {
        state: &global_state.theme,
    };
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };
    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout (search_bar=search_bar_props, theme=theme_props) {
            div (class="w-full px-8 h-24 bg-primary relative") {
                p(class="absolute bottom-3 font-bold text-2xl text-primary-content") {"Result for testing"}
            }
            div (class="flex justify-center w-full") {
                // the responsive part doesn't work.....
                 div (class="md:flex md:flex-row-reverse w-full lg:w-5/6 py-6 gap-4 justify-center px-4") {
                    div (class="w-full md:flex-1 md:w-1/3") {
                        Filter()
                    }
                    div (class="divider md:divider-horizontal"){}
                    div (class = "w-full md:flex-initial md:w-2/3") {
                        CourseTable()
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

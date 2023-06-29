use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::global_state::AppStateRx;


use models::CourseSummary;

use crate::components::course_table::CourseTable;
use crate::components::filter::{Filter, FilterProps};
use crate::components::layout::{Layout, SearchBarProps, ThemeProps};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CoursesStateRx")]
pub struct CoursesState {
    // search bar props
    search_input: String,
    search_results: Vec<String>,

    // query resut for table, 
    // TODO: should based on user input & filter
    table_content: Vec<CourseSummary>,

    // filter
    // term
    selectterm: String, // values: "currentterm", "nextterm"
    // level
    level1: bool,
    level2: bool,
    level3: bool,
    level4: bool,
    // status
    include_closed: bool,
    // period
    morning: bool,
    afternoon: bool,
    evening: bool,
    // dates
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,

}


fn courses_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a CoursesStateRx) -> View<G> {
    // temp table_content
    let table_content = &state.table_content;
    #[cfg(client)]
    create_effect_scoped(cx, |cx| {
        if !&state.search_input.get().is_empty() {
            spawn_local_scoped(cx, async {
                let body = reqwasm::http::Request::get(
                    format!("/api/v1/courses/{}", &state.search_input.get()).as_str(),
                )
                .send()
                .await
                .unwrap()
                .json::<Vec<CourseSummary>>()
                .await
                .unwrap()
                .to_vec();

                table_content.set(body);
                
            })
        } else {
            table_content.set(vec![]);
        }
    });

    // variables
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let theme_props = ThemeProps {
        state: &global_state.theme,
    };
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };

    let filterprops = FilterProps {
        // term
        selectterm: &state.selectterm,
        // level
        level1: &state.level1,
        level2: &state.level2,
        level3: &state.level3,
        level4: &state.level4,
        // status
        include_closed: &state.include_closed,
        // period
        morning: &state.morning,
        afternoon: &state.afternoon,
        evening: &state.evening,
        // dates
        monday: &state.monday,
        tuesday: &state.tuesday,
        wednesday: &state.wednesday,
        thursday: &state.thursday,
        friday: &state.friday,
    };

    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout (search_bar=search_bar_props, theme=theme_props) {
            div (class="w-full px-8 h-20 bg-primary relative") {
                p(class="absolute bottom-3 font-bold text-2xl text-primary-content") {"Result for testing"}
            }
            div (class="flex justify-center w-full") {
                 div (class="md:flex md:flex-row-reverse w-full lg:w-5/6 py-6 gap-4 justify-center px-4") {
                    div (class="w-full md:flex-1 md:w-1/3") {
                        Filter( filterprops )
                    }
                    div (class="divider md:divider-horizontal"){}                    
                    div (class = "w-full md:flex-initial md:w-2/3") {
                        CourseTable(table_content=table_content)    
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
        table_content: vec![],
        // term
        selectterm: "".to_string(),
        // level
        level1: false,
        level2: false,
        level3: false,
        level4: false,
        // status
        include_closed: false,
        // period
        morning: false,
        afternoon: false,
        evening: false,
        // dates
        monday: false,
        tuesday: false,
        wednesday: false,
        thursday: false,
        friday: false,
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

use crate::components::layout::{Layout, SearchBarProps};
use crate::components::course_table::CourseTable;
use crate::components::filter::Filter;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CoursesStateRx")]
pub struct CoursesState {
    // search bar props
    search_input: String,
    search_results: Vec<String>,

    // TODO: populate course_table with query result
    // table content
    table_content: Vec<Vec<String>>,

    // filter
    // term
    currentterm: bool,
    nextterm: bool,
    // level
    level1: bool,
    level2: bool,
    level3: bool,
    level4: bool,
    all_levels: bool,
    // status
    open: bool,
    closed: bool,
    all_status: bool,
    // period
    morning: bool,
    afternoon: bool,
    evening: bool,
    all_periods: bool,
    // dates
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    all_dates: bool,

}

fn courses_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a CoursesStateRx) -> View<G> {
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };
    // term
    let currentterm = &state.currentterm;
    let nextterm = &state.nextterm;
    // level
    let level1 = &state.level1;
    let level2 = &state.level2;
    let level3 = &state.level3;
    let level4 = &state.level4;
    let all_levels = &state.all_levels;
    // status
    let open = &state.open;
    let closed = &state.closed;
    let all_status = &state.all_status;
    // period
    let morning = &state.morning;
    let afternoon = &state.afternoon;
    let evening = &state.evening;
    let all_periods = &state.all_periods;
    // dates
    let monday = &state.monday;
    let tuesday = &state.tuesday;
    let wednesday = &state.wednesday;
    let thursday = &state.thursday;
    let friday = &state.friday;
    let all_dates = &state.all_dates;

    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout (search_bar=search_bar_props) {
            div (class="w-full px-8 h-24 bg-primary relative") {
                p(class="absolute bottom-3 font-bold text-2xl text-primary-content") {"Result for testing"}
            }
            div (class="flex justify-center w-full") {
                // the responsive part doesn't work..... 
                 div (class="md:flex md:flex-row-reverse w-full lg:w-5/6 py-6 gap-4 justify-center px-4") {
                    div (class="w-full md:flex-1 md:w-1/3") {
                        Filter(
                            currentterm=currentterm,
                            nextterm = nextterm,
                            // level
                            level1 = level1,
                            level2 = level2,
                            level3 = level3,
                            level4 = level4,
                            all_levels = all_levels,
                            // status
                            open = open,
                            closed = closed,
                            all_status = all_status,
                            // period
                            morning = morning,
                            afternoon = afternoon,
                            evening = evening,
                            all_periods = all_periods,
                            // dates
                            monday = monday,
                            tuesday = tuesday,
                            wednesday = wednesday,
                            thursday = thursday,
                            friday = friday,
                            all_dates = all_dates,
                        )
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
        table_content: vec![],
        currentterm: false,
        nextterm: false,
        // level
        level1: false,
        level2: false,
        level3: false,
        level4: false,
        all_levels: false,
        // status
        open: false,
        closed: false,
        all_status: false,
        // period
        morning: false,
        afternoon: false,
        evening: false,
        all_periods: false,
        // dates
        monday: false,
        tuesday: false,
        wednesday: false,
        thursday: false,
        friday: false,
        all_dates: false,
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

use crate::{
    components::{
        course_table::CourseTable,
        filter::{Filter, FilterProps},
        layout::{Layout, SearchBarProps, ThemeProps},
    },
    global_state::AppStateRx,
};
use models::CourseSummary;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CoursesStateRx")]
pub struct CoursesState {
    path: String,

    // Searchbar
    search_input: String,
    search_results: Vec<CourseSummary>,

    // TODO: Change based on filters
    table_content: Vec<CourseSummary>,

    // Filters
    term: String,
    level1: bool,
    level2: bool,
    level3: bool,
    level4: bool,
    include_closed: bool,
    morning: bool,
    afternoon: bool,
    evening: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
}

fn courses_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a CoursesStateRx) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let table_content = &state.table_content;

    #[cfg(client)]
    if !&state.path.get_untracked().is_empty() {
        spawn_local_scoped(cx, async {
            let body = reqwasm::http::Request::get(
                format!("/api/v1/courses/{}", &state.path.get_untracked()).as_str(),
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
    }

    let theme_props = ThemeProps {
        state: &global_state.theme,
    };
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };
    let filterprops = FilterProps {
        term: &state.term,
        level1: &state.level1,
        level2: &state.level2,
        level3: &state.level3,
        level4: &state.level4,
        include_closed: &state.include_closed,
        morning: &state.morning,
        afternoon: &state.afternoon,
        evening: &state.evening,
        monday: &state.monday,
        tuesday: &state.tuesday,
        wednesday: &state.wednesday,
        thursday: &state.thursday,
        friday: &state.friday,
        saturday: &state.saturday,
        sunday: &state.sunday,
    };

    #[cfg(client)]
    create_effect_scoped(cx, |cx| {
        let _ = filterprops.term.get();
        let _ = filterprops.level1.get();
        let _ = filterprops.level2.get();
        let _ = filterprops.level3.get();
        let _ = filterprops.level4.get();
        let _ = filterprops.include_closed.get();
        let _ = filterprops.morning.get();
        let _ = filterprops.afternoon.get();
        let _ = filterprops.evening.get();
        let _ = filterprops.monday.get();
        let _ = filterprops.tuesday.get();
        let _ = filterprops.wednesday.get();
        let _ = filterprops.thursday.get();
        let _ = filterprops.friday.get();
        let _ = filterprops.saturday.get();
        let _ = filterprops.sunday.get();

        spawn_local_scoped(cx, async {
            let body = reqwasm::http::Request::get(
                format!("/api/v1/courses/{}/term/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}/{}", 
                &state.path.get_untracked(),
                &state.term.get_untracked(),
                &state.level1.get_untracked(),
                &state.level2.get_untracked(),
                &state.level3.get_untracked(),
                &state.level4.get_untracked(),
                &state.include_closed.get_untracked(),
                &state.morning.get_untracked(),
                &state.afternoon.get_untracked(),
                &state.evening.get_untracked(),
                &state.monday.get_untracked(),
                &state.tuesday.get_untracked(),
                &state.wednesday.get_untracked(),
                &state.thursday.get_untracked(),
                &state.friday.get_untracked(),
                &state.saturday.get_untracked(),
                &state.sunday.get_untracked(),
            ).as_str(),
                
            )
            .send()
            .await
            .unwrap()
            .json::<Vec<CourseSummary>>()
            .await
            .unwrap()
            .to_vec();

            // for course_index in 0..body.len() {
            //     let course_code = format!(
            //         "{}{}",
            //         &body.get(course_index).unwrap().subject_code,
            //         &body.get(course_index).unwrap().catalog_number,
            //     );
            //     let offering_details = reqwasm::http::Request::get(
            //         format!(
            //             "api/v1/course_offerings/{}",
            //             course_code
            //         ).as_str(),
            //     )
            //     .send()
            //     .await
            //     .unwrap()
            //     .json::<Vec<OfferingDetail>>()
            //     .await
            //     .unwrap()
            //     .to_vec();

            //     course_offering_details.modify().push(offering_details);
            // }

            table_content.set(body);
        })
    });


    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout (search_bar=search_bar_props, theme=theme_props) {
            div (class="w-full px-8 h-20 bg-primary relative") {
                p(class="absolute bottom-3 font-bold text-2xl text-primary-content") {
                    (if state.path.get_untracked().is_empty() {
                        "Showing all courses".to_owned()
                    } else {
                        format!("Search results for {}", state.path.get_untracked())
                    })
                }
            }
            div (class="flex justify-center w-full") {
                 div (class="md:flex md:flex-row-reverse py-6 px-5") {
                    div (class="w-full md:flex-1 md:w-2/6") {
                        Filter( filterprops )
                    }
                    div (class="divider md:divider-horizontal"){}
                    div (class = "w-full md:flex-initial md:w-4/6") {
                        CourseTable(table_content=table_content)
                    }
                }
            }
        }
    }
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> CoursesState {
    CoursesState {
        path: info.path,
        search_input: "".to_string(),
        search_results: vec![],
        table_content: vec![],
        term: "".to_string(),
        level1: false,
        level2: false,
        level3: false,
        level4: false,
        include_closed: false,
        morning: false,
        afternoon: false,
        evening: false,
        monday: false,
        tuesday: false,
        wednesday: false,
        thursday: false,
        friday: false,
        saturday: false,
        sunday: false,
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["".to_string()],
        extra: ().into(),
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
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .incremental_generation()
        .view_with_state(courses_page)
        .head_with_state(head)
        .build()
}

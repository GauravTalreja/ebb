use crate::components::layout::{Layout, SearchBarProps, ThemeProps};
use crate::components::schedule::Schedule;
use crate::components::course_intro::CourseIntro;
use crate::global_state::AppStateRx;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use models::{CourseDetail, ClassSchedule};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "CourseDetailStateRx")]
pub struct CourseDetailState {
    // search bar props
    search_input: String,
    search_results: Vec<String>,

    // description props
    // TODO: change it to corresponding backend structure (model/src/course.rs)
    intro_content: CourseDetail,

    // schedule table query
    // TODO: change it to corresponding backend structure
    schedule_content: Vec<ClassSchedule>,
}

fn course_detail<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a CourseDetailStateRx) -> View<G> {
    // variables
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let theme_props = ThemeProps {
        state: &global_state.theme,
    };
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };
    let intro_content = &state.intro_content;
    let schedule_content = &state.schedule_content;


    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout (search_bar=search_bar_props, theme=theme_props) { 
            div (class="flex justify-center w-full") {  

                div (class="flex flex-col w-full lg:w-5/6 py-6 gap-8 px-4") {           
                    // CourseIntro (
                    //     code="CS136".to_string(),
                    //     name="Elementary Algorithm Design and Data Abstraction".to_string(),
                    //     description="This course builds on the techniques and patterns learned in CS 135 while making the transition to use an imperative language. It introduces the design and analysis of algorithms, the management of information, and the programming mechanisms and methodologies required in implementations. Topics discussed include iterative and recursive sorting algorithms; lists, stacks, queues, trees, and their application; abstract data types and their implementations.".to_string(),
                    //     prerequisite="None".to_string(),
                    // )   
                    CourseIntro ( intro_content=intro_content) 
                    Schedule (schedule_content=schedule_content)

                    div (class="flex justify-center w-full") {
                        button (class="btn btn-primary w-1/6") {"Finish Enroll"}
                    }
                }              
            }
        }
    }
}




#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> CourseDetailState {
    CourseDetailState {
        search_input: "".to_string(),
        search_results: vec![],
        intro_content: CourseIntro {
            catalog_number: "".to_string(),
            subject_code: "".to_string(),
            title: "".to_string(),
            course_description: "".to_string(),
            prerequisite_description: "".to_string(),
        },
        schedule_content: vec![],
    }
}

#[engine_only_fn]
fn head(cx: Scope, _state: CourseDetailState) -> View<SsrNode> {
    view! { cx,
        title { "Search for courses" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("detail")
        .build_state_fn(get_build_state)
        .view_with_state(course_detail)
        .head_with_state(head)
        .build()
}

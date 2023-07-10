use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::global_state::AppStateRx;

use models::CourseSummary;

use crate::components::layout::{Layout, SearchBarProps, ThemeProps};
use crate::components::schedule::Schedule;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "PersonalPageStateRx")]
pub struct PersonalPageState {
    // search bar props
    search_input: String,
    search_results: Vec<String>,

    // query resut for table,
    schedule_content: Vec<CourseSummary>,

    // courses taken
    course_taken: Vec<CourseSummary>,    
}

fn personal_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a PersonalPageStateRx) -> View<G> {

    // variables
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let theme_props = ThemeProps {
        state: &global_state.theme,
    };
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };

   let schedule_content = &state.schedule_content;
   let course_taken = &state.course_taken;

    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout (search_bar=search_bar_props, theme=theme_props) {
            div (class="md:flex justify-center py-6 px-5") {
                    div (class = "w-full md:flex-initial md:w-4/6") {
                       Schedule(schedule_content=schedule_content)
                    }
            }
 
        }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> PersonalPageState {
    PersonalPageState {
        search_input: "".to_string(),
        search_results: vec![],
        schedule_content: vec![],
        course_taken: vec![],
        
    }
}

#[engine_only_fn]
fn head(cx: Scope, _state: PersonalPageState) -> View<SsrNode> {
    view! { cx,
        title { "My Schedule" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("personal_schedule")
        .build_state_fn(get_build_state)
        .view_with_state(personal_page)
        .head_with_state(head)
        .build()
}

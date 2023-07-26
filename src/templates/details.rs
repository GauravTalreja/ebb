use crate::{
    components::layout::{Layout, SearchBarProps, ThemeProps},
    global_state::AppStateRx,
};
use models::{CourseDetail, CourseSummary};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "DetailStateRx")]
pub struct DetailState {
    path: String,
    course_detail: CourseDetail,
    search_input: String,
    search_results: Vec<CourseSummary>,
}

fn details_page<'a, G: Html>(cx: BoundedScope<'_, 'a>, state: &'a DetailStateRx) -> View<G> {
    let global_state = Reactor::<G>::from_cx(cx).get_global_state::<AppStateRx>(cx);
    let theme_props = ThemeProps {
        state: &global_state.theme,
    };
    let search_bar_props = SearchBarProps {
        input: &state.search_input,
        results: &state.search_results,
    };

    #[cfg(client)]
    if !&state.path.get_untracked().is_empty() {
        spawn_local_scoped(cx, async {
            let body = reqwasm::http::Request::get(
                format!("/api/v1/course/{}", &state.path.get_untracked()).as_str(),
            )
            .send()
            .await
            .unwrap()
            .json::<CourseDetail>()
            .await
            .unwrap();

            state.course_detail.set(body);
        })
    }

    if !state.path.get().is_empty() {
        view! { cx,
            link ( rel="stylesheet", href="/tailwind.css")
            Layout (search_bar=search_bar_props, theme=theme_props) {
                div (class="prose prose-2xl w-full mx-16") {
                    div (class = "h-16")
                    h1 { (state.course_detail.get().subject_code.clone() + " " + &state.course_detail.get().catalog_number + " - " + &state.course_detail.get().title) }
                    p { (state.course_detail.get().description) }
                    (if !state.course_detail.get().required_prerequisites.is_empty() {
                        view! { cx,
                            div {
                                h2 { "Required Prerequisites:" }
                                Keyed (
                                    iterable=create_signal(cx, state.course_detail.get().required_prerequisites.clone()),
                                    view=|cx, content| view! { cx,
                                        p {(content)}
                                    },
                                    key=|content| content.clone(),
                                )
                            }
                        }
                    } else {
                        view! {cx, }
                    })
                    (if !state.course_detail.get().optional_prerequisites.is_empty() {
                        view! { cx,
                            div {
                                h2 { "Optional Prerequisites:" }
                                Keyed (
                                    iterable=create_signal(cx, state.course_detail.get().optional_prerequisites.clone()),
                                    view=|cx, content| view! { cx,
                                        p {(content)}
                                    },
                                    key=|content| content.clone(),
                                )
                            }
                        }
                    } else {
                        view! {cx, }
                    })
                }
            }
        }
    } else {
        view! {cx, }
    }
}

#[engine_only_fn]
async fn get_build_state(info: StateGeneratorInfo<()>) -> DetailState {
    DetailState {
        path: info.path,
        course_detail: CourseDetail {
            id: -1,
            catalog_number: "".to_owned(),
            subject_code: "".to_owned(),
            title: "Loading...".to_owned(),
            external_id: "".to_owned(),
            description: "".to_owned(),
            academic_level: "".to_owned(),
            optional_prerequisites: vec!["null".to_owned()],
            required_prerequisites: vec!["undefined".to_owned()],
        },
        search_input: "".to_string(),
        search_results: vec![],
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["".to_owned()],
        extra: ().into(),
    }
}

#[engine_only_fn]
fn head(cx: Scope, state: DetailState) -> View<SsrNode> {
    view! { cx,
        title { (state.course_detail.title) }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("c")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .incremental_generation()
        .view_with_state(details_page)
        .head_with_state(head)
        .build()
}

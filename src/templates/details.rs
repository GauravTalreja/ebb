use crate::{
    components::layout::{FooterProps, Layout, SearchBarProps, ThemeProps},
    global_state::AppStateRx,
};
use models::{CourseDetail, CourseSummary, LastUpdated, OfferingDetail};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "DetailStateRx")]
pub struct DetailState {
    path: String,
    course_detail: CourseDetail,
    offering_details: Vec<OfferingDetail>,
    search_input: String,
    search_results: Vec<CourseSummary>,
    last_updated_time: LastUpdated,
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

    let footer_props = FooterProps {
        last_updated_time: &state.last_updated_time,
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
        });

        spawn_local_scoped(cx, async {
            let body = reqwasm::http::Request::get(
                format!("/api/v1/course_offerings/{}", &state.path.get_untracked()).as_str(),
            )
            .send()
            .await
            .unwrap();
            use std::cmp::Ordering;
            match body.json::<Vec<OfferingDetail>>().await {
                Ok(mut details) => state.offering_details.set({
                    details.iter_mut().for_each(|offering| {
                        offering
                            .schedules
                            .0
                            .sort_unstable_by(|a, b| a.class_section.cmp(&b.class_section));

                        offering.schedules.0.sort_unstable_by(|a, b| {
                            match (&a.component, &b.component) {
                                (Some(a_str), Some(b_str)) => a_str.cmp(b_str),
                                (Some(_), None) => Ordering::Less,
                                (None, Some(_)) => Ordering::Greater,
                                (None, None) => Ordering::Equal,
                            }
                        })
                    });
                    details
                }),
                Err(e) => web_log!("{body:?} {e}"),
            }
        });

        spawn_local_scoped(cx, async {
            let body = reqwasm::http::Request::get("/api/v1/last_updated_time")
                .send()
                .await
                .unwrap()
                .json::<LastUpdated>()
                .await
                .unwrap();
            state.last_updated_time.set(body);
        });
    }

    if !state.path.get().is_empty() {
        view! { cx,
            link ( rel="stylesheet", href="/tailwind.css")
            Layout (search_bar=search_bar_props, theme=theme_props, footer=footer_props) {
                div (class="prose prose-xl w-full mx-16") {
                    div (class = "h-16")
                    h1 { (state.course_detail.get().subject_code.clone() + " " + &state.course_detail.get().catalog_number + " - " + &state.course_detail.get().title) }
                    h3 { (String::from("Course Description")) }
                    p { (state.course_detail.get().description) }
                    h3 { (String::from("Course Requirements")) }
                    p { (state.course_detail.get().requirements_description.clone().unwrap_or(String::from("This course has no requirements!"))) }
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
                    Keyed (
                        iterable=&state.offering_details,
                        view=|cx, offering| view! { cx,
                            h3 { (format!("{} {}", offering.term, offering.year)) }
                            Keyed (
                                iterable=create_signal(cx, offering.schedules.0.clone()),
                                view=|cx, schedule| view! { cx,
                                    h4 {(format!("{} {}\n",
                                        schedule.component.as_ref().unwrap_or(&"".to_owned()),
                                        schedule.class_section,
                                    ))
                                    (match &schedule.instructor_name {
                                        Some(instructor) => if !instructor.is_empty() {
                                            format!(" - {instructor}")
                                        } else {
                                            "".to_owned()
                                        },
                                        None => "".to_owned()
                                    })
                                    (match &schedule.location {
                                        Some(location) => if !location.is_empty() {
                                            format!(" @ {}", location)
                                        } else {
                                            "".to_owned()
                                        },
                                        None => "".to_owned()
                                    })
                                    " on "
                                    ({
                                        let mut days = Vec::new();
                                        if schedule.monday {
                                            days.push("Monday");
                                        }
                                        if schedule.tuesday {
                                            days.push("Tuesday");
                                        }
                                        if schedule.wednesday {
                                            days.push("Wednesday");
                                        }
                                        if schedule.thursday {
                                            days.push("Thursday");
                                        }
                                        if schedule.friday {
                                            days.push("Friday");
                                        }
                                        if schedule.saturday {
                                            days.push("Saturday");
                                        }
                                        if schedule.sunday {
                                            days.push("Sunday");
                                        }
                                        // Join the days using the appropriate separators (e.g., "and" or ",")
                                        match days.len() {
                                            0 => "No days".to_owned(),
                                            1 => days[0].to_owned(),
                                            2 => format!("{} and {}", days[0], days[1]),
                                            _ => {
                                                let last_day = days.pop().unwrap();
                                                let other_days = days.join(", ");
                                                format!("{} and {}", other_days, last_day)
                                            }
                                        }
                                    })
                                    ", "
                                    (schedule.start_time.format("%H:%M").to_string())
                                    " - "
                                    (schedule.end_time.format("%H:%M").to_string())
                                    (format!(" ({}/{})",
                                        schedule.current_enrollment,
                                        schedule.max_enrollment,
                                    ))
                                    }
                                },
                                key=|content| content.clone(),
                            )
                        },
                        key=|content| content.clone(),
                    )
                    div (class = "h-16")
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
            requirements_description: Some("".to_owned()),
            academic_level: "".to_owned(),
            optional_prerequisites: vec!["null".to_owned()],
            required_prerequisites: vec!["undefined".to_owned()],
        },
        offering_details: vec![],
        search_input: "".to_string(),
        search_results: vec![],
        last_updated_time: LastUpdated { date_time: None },
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
        .revalidate_after("1s")
        .build()
}

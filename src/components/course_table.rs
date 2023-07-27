use perseus::prelude::*;
use sycamore::prelude::*;

use models::CourseSummary;

// table input
#[derive(Prop)]
pub struct CourseTableProps<'a> {
    pub table_content: &'a RcSignal<Vec<CourseSummary>>,
}

// table structure
#[component]
pub fn CourseTable<'a, G: Html>(
    cx: Scope<'a>,
    CourseTableProps { table_content }: CourseTableProps<'a>,
) -> View<G> {
    view! {cx, (if !table_content.get().is_empty() {
        view! { cx,
            div (class="overflow-x-auto w-full shadow-md bg-base-100") {
                // TODO: change header color, change hover color
                table (class="table w-full") {
                    thead(class = "bg-primary") {
                        tr() {
                            TableColumnHead(name="Code".to_string())
                            TableColumnHead(name="Course Name".to_string())
                            TableColumnHead(name="Location".to_string())
                        }
                    }
                    tbody() {
                        Keyed (
                            iterable=table_content,
                            view=|cx, content| view! { cx,
                                TableContent(
                                    code=content.subject_code.clone() + &content.catalog_number.to_string(),
                                    coursename=content.title.clone(),
                                    location={
                                        if content.offerings.summaries.len() >= 1 {
                                            "UW"
                                        }
                                        else {
                                            "N/A"
                                        }
                                    }.to_string(),
                                )
                            },
                            key=|content| content.clone(),
                        )
                    }
                }
            }
        }
    } else {
        view! { cx,
            div (class = "prose") {
                h2 { "No courses found..." }
            }
        }
    })}
}

#[derive(Prop)]
pub struct TableColumnHeadProps {
    name: String,
}

#[component]
fn TableColumnHead<G: Html>(
    cx: Scope,
    TableColumnHeadProps { name }: TableColumnHeadProps,
) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        th(class="bg-primary") { p(class="text-base normal-case text-primary-content") { (name) } }
    }
}

// table content
#[derive(Prop)]
pub struct TableContentProps {
    code: String,
    coursename: String,
    location: String,
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn TableContent<G: Html>(
    cx: Scope,
    TableContentProps {
        code,
        coursename,
        location,
    }: TableContentProps,
) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        tr (class="hover ") {

            td() { (code) }
            td() { (coursename) }
            td() { (location) }

        }
    }
}

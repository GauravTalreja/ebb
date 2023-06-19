use sycamore::prelude::*;

// table structure
#[component]
pub fn CourseTable<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="overflow-x-auto w-full shadow-md bg-base-200") {
            // TODO: change header color, change hover color
            table (class="table w-full") {
                thead() {
                    tr() {
                        TableColumnHead(name="".to_string())
                        TableColumnHead(name="Code".to_string())
                        TableColumnHead(name="Course Name".to_string())
                        TableColumnHead(name="Location".to_string())
                        TableColumnHead(name="Status".to_string())
                    }
                }
                tbody() {
                    TableContent(idx="1".to_string(),
                                code="CS136".to_string(),
                                coursename="Elementary Algorithm Design and Data Abstraction".to_string(),
                                location="ONLN/UW".to_string(),
                                status="Open".to_string())
                    TableContent(idx="2".to_string(),
                                code="CS246".to_string(),
                                coursename="Object-Oriented Software Development".to_string(),
                                location="ONLN/UW".to_string(),
                                status="Closed".to_string())
                    TableContent(idx="3".to_string(),
                                code="MUSIC140".to_string(),
                                coursename="Popular Music and Culture".to_string(),
                                location="ONLN".to_string(),
                                status="Open".to_string())
                    TableContent(idx="4".to_string(),
                                code="ECON102".to_string(),
                                coursename="Introduction to Macroeconomics".to_string(),
                                location="REN".to_string(),
                                status="Open".to_string())

                }

            }
        }

    }
}

// table heading
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
    
    idx: String,
    code: String,
    coursename: String,
    location: String,
    status: String,
    
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn TableContent<G: Html>(
    cx: Scope,
    TableContentProps {
        idx,
        code,
        coursename,
        location,
        status,
    }: TableContentProps,
) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        tr (class="hover ") {
            th() { (idx) } // row index
            td() { (code) }
            td() { (coursename) }
            td() { (location) }
            td() { (status) }

        }
    }
}

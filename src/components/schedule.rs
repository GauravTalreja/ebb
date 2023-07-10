use perseus::prelude::*;
use sycamore::prelude::*;


use models::CourseSummary;

// table input
#[derive(Prop)]
pub struct ScheduleProps<'a>  {
    pub schedule_content: &'a RcSignal<Vec<CourseSummary>>,

}


// table structure
#[component]
pub fn Schedule<'a, G: Html>(
    cx: Scope<'a>,
    ScheduleProps { schedule_content }: ScheduleProps<'a>,
) -> View<G> {

    view! { cx,
        div (class="overflow-x-auto w-full shadow-md bg-base-200") {
            // TODO: change header color, change hover color
            table (class="table w-full") {
                thead() {
                    tr() {
                        TableColumnHead(name="Time".to_string())
                        TableColumnHead(name="Monday".to_string())
                        TableColumnHead(name="Tuesday".to_string())
                        TableColumnHead(name="Wednessday".to_string())
                         TableColumnHead(name="Thursday".to_string())
                        TableColumnHead(name="Friday".to_string())
                    }
                }
                tbody() {
                    // Keyed (
                    //     iterable=schedule_content,
                    //     view=|cx, content| view! { cx,
                    //         // need change here
                    //         TableContent(
                    //             code=content.subject_code.clone() + &content.catalog_number.to_string(),
                    //             coursename=content.title.clone(),
                    //             location="UW".to_string(),
                    //             status="status".to_string()
                    //         )
                    //     },
                    //     key=|content| content.clone(),
                    // )

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
pub struct SectionProps {
    
    catalog_number: i16,
    subject_code: String,
    section: String,
    start_time: String,
    
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn TableContent<G: Html>(
    cx: Scope,
    SectionProps {
        catalog_number,
        subject_code,
        section,
        start_time,
    }: SectionProps,
) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        tr (class="hover ") {
            
            td() { (catalog_number) }
            td() { (subject_code) }
            td() { (section) }
            td() { (start_time) }

        }
    }
}

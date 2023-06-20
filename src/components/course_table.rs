use sycamore::prelude::*;

// table input
#[derive(Prop)]
pub struct CourseTableProps<'a>  {
    // true version (use true version when populatin data)
    // pub table_content: &'a RcSignal<Vec<Vec<String>>>,

    // test version (remove or comment when )
    pub table_content: &'a Signal<Vec<Vec<String>>>,
}


// table structure
#[component]
pub fn CourseTable<'a, G: Html>(
    cx: Scope<'a>,
    CourseTableProps { table_content }: CourseTableProps<'a>,
) -> View<G> {
    view! { cx,
        div (class="overflow-x-auto w-full shadow-md bg-base-200") {
            // TODO: change header color, change hover color
            table (class="table w-full") {
                thead() {
                    tr() {
                        TableColumnHead(name="Code".to_string())
                        TableColumnHead(name="Course Name".to_string())
                        TableColumnHead(name="Location".to_string())
                        TableColumnHead(name="Status".to_string())
                    }
                }
                tbody() {
                    Indexed (
                        iterable=table_content,
                        view=|cx, content| view! { cx,
                            TableContent(
                                
                                code=content[0].clone(),
                                coursename=content[1].clone(),
                                location=content[2].clone(),
                                status=content[3].clone()
                            )
                        }
                    )
                    TableContent(
                                code="CS136".to_string(),
                                coursename="Elementary Algorithm Design and Data Abstraction".to_string(),
                                location="ONLN/UW".to_string(),
                                status="Open".to_string())
                    TableContent(
                                code="CS246".to_string(),
                                coursename="Object-Oriented Software Development".to_string(),
                                location="ONLN/UW".to_string(),
                                status="Closed".to_string())
                    TableContent(
                                code="MUSIC140".to_string(),
                                coursename="Popular Music and Culture".to_string(),
                                location="ONLN".to_string(),
                                status="Open".to_string())
                    TableContent(
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
        code,
        coursename,
        location,
        status,
    }: TableContentProps,
) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        tr (class="hover ") {
            
            td() { (code) }
            td() { (coursename) }
            td() { (location) }
            td() { (status) }

        }
    }
}

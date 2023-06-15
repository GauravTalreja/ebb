use sycamore::prelude::*;

// table structure
#[component]
pub fn CourseTable<'a, G: Html>(
    cx: Scope<'a>,
) -> View<G> {
    view! { cx,
        div (class="overflow-x-auto w-full") {
            // TODO: change header color, change hover color
            table (class="table table-md w-full") {
                // table header
                thead() {
                    tr() {
                        th(){} // row index
                        th(class="text-lg") { "Course Code" }
                        th(class="text-lg") { "Course Name" }
                        th(class="text-lg") { "Location" }
                        th(class="text-lg") { "Status" }
                    }
                }
                // table content
                tbody() {
                    TableContent(name = "testing".to_string())
                    TableContent(name = "testing".to_string())
                    TableContent(name = "testing".to_string())
                    TableContent(name = "testing".to_string())
                }

            }
        }

    }
        
}

// table content
#[derive(Prop)]
pub struct TableContentProps {
    
    // TODO: include more content
    name: String,
    // subject + code e.g. CS136
    // coursecode: String,
    // courseName: String, 
    // location: String,
    // status: String,
    
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn TableContent<G: Html>(cx: Scope, 
    TableContentProps { name }: TableContentProps) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        tr (class="hover ") {
            th() { "1"} // row index
            td() { (name) }
            td() { (name) }
            td() { (name) }
            td() { (name) }       
            
        }
    }
}
use sycamore::prelude::*;

// table structure
#[component]
pub fn Shcedule<G: Html>(
    cx: Scope,
) -> View<G> {
    view! { cx,
                div (class="overflow-x-auto w-full shadow-md rounded-md") {
                    // TODO: change header color, change hover color
                    table (class="table w-full") {
                        // table header
                        thead(class="") {
                            tr() {
                                TableColumnHead(name="Section".to_string())
                                TableColumnHead(name="Class Number".to_string())
                                TableColumnHead(name="Enrolled".to_string())
                                TableColumnHead(name="Time".to_string())
                                TableColumnHead(name="Date".to_string())
                                TableColumnHead(name="Location".to_string())
                                TableColumnHead(name="Instructor".to_string())
                                TableColumnHead(name="Select".to_string())
                            }
                        }
                        // table content
                        tbody() {
                            
                            TableContent(section="001".to_string(),
                                        class_num="1247".to_string(), 
                                        enrolled="50/80".to_string(),
                                        time="9:00-11:00".to_string(),
                                        date="MWF".to_string(),
                                        location="UW".to_string(),
                                        instructor="Henry Lans".to_string(),
                                    )     
                            TableContent(section="002".to_string(),
                                        class_num="1248".to_string(), 
                                        enrolled="10/20".to_string(),
                                        time="12:00-14:00".to_string(),
                                        date="MWF".to_string(),
                                        location="ONLN".to_string(),
                                        instructor="Henry Lans".to_string(),
                                    )                 
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
fn TableColumnHead<G: Html>(cx: Scope, 
    TableColumnHeadProps { name  }: TableColumnHeadProps) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        th(class="bg-primary") { p(class="text-base normal-case text-primary-content") { (name) } }
    }
}

// table content
#[derive(Prop)]
pub struct TableContentProps {
    
    // TODO: include more content
    section: String,
    class_num: String,
    enrolled: String,
    time: String,
    date: String,
    location: String,
    instructor: String
    
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn TableContent<G: Html>(cx: Scope, 
    TableContentProps { section, class_num, enrolled, time, date, location, instructor }: TableContentProps) -> View<G> {
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        tr (class="hover ") {
            td() { (section) }
            td() { (class_num) }
            td() { (enrolled) }
            td() { (time) } 
            td() { (date) }  
            td() { (location) }
            td() { (instructor) }  
            td() {
                input (type="checkbox", class="checkbox checkbox-primary")
            }     
            
        }
    }
}




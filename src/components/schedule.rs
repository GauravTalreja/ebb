use sycamore::prelude::*;
use models::ClassSchedule;

// schedule input
#[derive(Prop)]
pub struct ScheduleProps<'a> {
    pub schedule_content: &'a RcSignal<Vec<ClassSchedule>>,
}

// table structure
#[component]
pub fn Schedule<'a, G: Html>(
    cx: Scope<'a>,
    ScheduleProps {schedule_content}: ScheduleProps<'a>,
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
                            // TODO: waiting to be fullfiled
                            Keyed (
                                iterable=schedule_content,
                                view=|cx, content| view! { cx,
                                    // need change here
                                    TableContent(section=content.component.unwrap_or_default() + &content.class_section.to_string(),
                                        class_num=content.class_number, 
                                        enrolled=format!("{}/{}", content.current_enrollment.to_string(), content.max_enrollment.to_string()),
                                        time=format!("{}--{}", content.start_time.clone(), content.end_time.clone()),
                                        mon=content.monday,
                                        tue=content.tuesday,
                                        wed=content.wednesday,
                                        thur=content.thursday,
                                        fri=content.friday,
                                        location=content.location.unwrap_or_default(),
                                        instructor=content.instructor_name.unwrap_or_default(),
                                    )  
                                },
                                key=|content| content.clone(),
                            )
                            
                            TableContent (
                                section="001".to_string(),
                                class_num=1247, 
                                enrolled="50/80".to_string(),
                                time="9:00-11:00".to_string(),
                                mon=true,
                                tue=false,
                                wed=true,
                                thur=false,
                                fri=false,
                                location="UW".to_string(),
                                instructor="Henry Lans".to_string(),
                            )     
                            TableContent (
                                section="002".to_string(),
                                class_num=1248, 
                                enrolled="10/20".to_string(),
                                time="12:00-14:00".to_string(),
                                mon=false,
                                tue=false,
                                wed=true,
                                thur=true,
                                fri=false,
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
    class_num: i16,
    enrolled: String,
    time: String,
    mon: bool,
    tue: bool,
    wed: bool,
    thur: bool,
    fri: bool,
    location: String,
    instructor: String
    
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn TableContent<G: Html>(cx: Scope, 
    TableContentProps { section, class_num, enrolled, time, mon, tue, wed, thur, fri, location, instructor }: TableContentProps) -> View<G> {
    
    let mut dates = String::new();
    if mon {
        dates += "M ";
    }
    if tue {
        dates += "Tu ";
    }
    if wed {
        dates += "W ";
    }
    if thur {
        dates += "Th ";
    }
    if fri {
        dates += "F";
    }
    view! { cx,
        // TODO: change hover color hover:bg-primary-content
        tr (class="hover ") {
            td() { (section) }
            td() { (class_num) }
            td() { (enrolled) }
            td() { (time) } 
            td() { (dates) }  
            td() { (location) }
            td() { (instructor) }  
            td() {
                input (type="checkbox", class="checkbox checkbox-primary")
            }     
            
        }
    }
}




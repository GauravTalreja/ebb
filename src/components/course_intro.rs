use sycamore::prelude::*;

// table heading
#[derive(Prop)]
pub struct CourseIntroProps {
    code: String,
    name: String,
    description: String,
    prerequisite: String,
}

#[component]
pub fn CourseIntro<G: Html>(
    cx: Scope,
    CourseIntroProps {code, name, description, prerequisite}: CourseIntroProps,
) -> View<G> {
    view! { cx,
        div (class="flex flex-col md:flex-row") {
                div (class="w-full md:flex-initial md:w-2/3") {
                    div { p (class="text-xl font-bold") { (code) } }
                    div { p (class="text-xl font-bold mb-4") { (name) } }
                    div (class="mb-4") { p  { (description) } }
                    
                    button (class="btn btn-primary") {"Add to wish list"} 
                    
                }
                div (class="divider md:divider-horizontal")
                div (class="w-full md:flex-1 md:w-1/3") {
                    div { p (class="text-bold") { "Prerequisites:" } 
                        p { (prerequisite) }
                        }
                }
            }
        }
        
}






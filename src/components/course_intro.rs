use sycamore::prelude::*;
use models::CourseDetail;

// table heading
#[derive(Prop)]
pub struct CourseIntroProps<'a> {
    pub intro_content: &'a RcSignal<CourseDetail>,
}
// #[derive(Prop)]
// pub struct CourseIntroProps {
//     code: String,
//     name: String,
//     description: String,
//     prerequisite: String,
// }

#[component]
pub fn CourseIntro<'a, G: Html>(
    cx: BoundedScope<'_, 'a>,
    CourseIntroProps {intro_content}: CourseIntroProps<'a>,
) -> View<G> {
    view! { cx,
        div (class="flex flex-col md:flex-row") {
                div (class="w-full md:flex-initial md:w-2/3") {
                    div { p (class="text-xl font-bold") { (intro_content.get().subject_code.clone() + &intro_content.get().catalog_number.clone()) } }
                    div { p (class="text-xl font-bold mb-4") { (intro_content.get().title.clone()) } }
                    div (class="mb-4") { p  { (intro_content.get().description.clone()) } }
                    
                    button (class="btn btn-primary") {"Add to wish list"} 
                    
                }
                div (class="divider md:divider-horizontal")
                div (class="w-full md:flex-1 md:w-1/3") {
                    div { p (class="text-bold") { "Prerequisites:" } 

                        Keyed (
                            iterable=create_signal(cx, intro_content.get().required_prerequisites.clone()),
                            view=|cx, content| view! { cx,
                                p { (content)}
                            },
                            key=|content| content.clone(),
                        )
                        
                    }
                }
            }
        }
        
}






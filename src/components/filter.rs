use sycamore::prelude::*;


#[derive(Prop)]
pub struct FilterElement<'a> {
    // term
    pub currentterm: &'a RcSignal<bool>,
    pub nextterm: &'a RcSignal<bool>,
    //level
    pub level1: &'a RcSignal<bool>,
    pub level2: &'a RcSignal<bool>,
    pub level3: &'a RcSignal<bool>,
    pub level4: &'a RcSignal<bool>,
    pub all_levels: &'a RcSignal<bool>,    
    // status
    pub open: &'a RcSignal<bool>,
    pub closed: &'a RcSignal<bool>,
    pub all_status: &'a RcSignal<bool>,
    // period
    pub morning: &'a RcSignal<bool>,
    pub afternoon: &'a RcSignal<bool>,
    pub evening: &'a RcSignal<bool>,
    pub all_periods: &'a RcSignal<bool>,
    // dates
    pub monday: &'a RcSignal<bool>,
    pub tuesday: &'a RcSignal<bool>,
    pub wednesday: &'a RcSignal<bool>,
    pub thursday: &'a RcSignal<bool>,
    pub friday: &'a RcSignal<bool>,
    pub all_dates: &'a RcSignal<bool>,
}


#[derive(Prop)]
pub struct FilterProps<'a> {
    filterelement: FilterElement<'a>
}

// TODO: for status, use a toggle with "include closed course"
// TODO: change level, period, dates to button-toggle, remove all_xxx
// TODO: change term to dropdown
#[component]
pub fn Filter<'a, G: Html>(
    cx: Scope<'a>,
    FilterProps { filterelement }: FilterProps<'a>,
) -> View<G> {
    view! { cx,
        div (class="flex flex-col shadow-md rounded-box bg-base-100") {
            p(class="rounded-box bg-primary p-4 font-bold text-primary-content text-lg") { "Filters" }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Term" }
                div (class="flex justify-center") {
                    div (class="flex flex-row w-full gap-x-8") {
                        CheckBox(name="Spring 2023".to_string(), checked=filterelement.currentterm)
                        CheckBox(name="Fall 2023".to_string(), checked=filterelement.nextterm)                        
                    }
                }
            }

            // level    
            div(class="p-4 bg-base-100 rounded-md w-full ") {
                h3 (class="text-lg font-semibold") { "Course Level" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="1--".to_string(), checked=filterelement.level1)
                        CheckBox(name="2--".to_string(), checked=filterelement.level2) 
                        CheckBox(name="3--".to_string(), checked=filterelement.level3)
                        CheckBox(name="4--".to_string(), checked=filterelement.level4) 
                        CheckBox(name="All".to_string(), checked=filterelement.all_levels) 
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Course status" }
                div (class="flex justify-center") {
                    div (class="flex flex-row flex-wrap w-full gap-x-8") {
                        CheckBox(name="Open".to_string(), checked=filterelement.open)
                        CheckBox(name="Closed".to_string(), checked=filterelement.closed) 
                        CheckBox(name="All".to_string(), checked=filterelement.all_status)
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Period" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="Morning".to_string(), checked=filterelement.morning)
                        CheckBox(name="Afternoon".to_string(), checked=filterelement.afternoon) 
                        CheckBox(name="Evening".to_string(), checked=filterelement.evening)
                        CheckBox(name="All".to_string(), checked=filterelement.all_periods)
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Date" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="Mon".to_string(), checked=filterelement.monday)
                        CheckBox(name="Tues".to_string(), checked=filterelement.tuesday) 
                        CheckBox(name="Wed".to_string(), checked=filterelement.wednesday)
                        CheckBox(name="Thur".to_string(), checked=filterelement.thursday)
                        CheckBox(name="Fri".to_string(), checked=filterelement.friday)
                        CheckBox(name="All".to_string(), checked=filterelement.all_dates)
                    }
                }
            }
            
            

            
        }       
    }        
}

// // filter section
// #[derive(Prop)]
// pub struct FilterScetionProps {
//     title: String,
//     // TODO: allow multiple filter selections 
//     // names: Vec<String>,
// }

// #[component]
// pub fn FilterSection<G: Html>(
//     cx: Scope,
//     FilterScetionProps { title }: FilterScetionProps,
// ) -> View<G> {
//     view! {cx,
//         div(class="p-4 bg-base-100 rounded-md w-full shadow-md") {
//             h3 (class="text-lg font-semibold") { (title) }
//             div (class="flex justify-center") {
//                 div (class="flex flex-col w-1/2") {
//                     CheckBox(name="testing".to_string())
//                     CheckBox(name="testing".to_string())  
//                 }
//             }          
//         }
//     }
// }

// check box
#[derive(Prop)]
pub struct CheckBoxProps<'a> {
    pub name: String,  
    pub checked: &'a RcSignal<bool>,    
}

#[component]
pub fn CheckBox<'a, G: Html>(
    cx: Scope<'a>,
    CheckBoxProps { name, checked }: CheckBoxProps<'a>,
) -> View<G> {
    view! {cx,
        div (class="form-control") {
            label (class="cursor-pointer label") {
                span (class="label-text mr-8") { (name) }
                input (
                    type="checkbox", 
                    class="checkbox checkbox-primary",
                    bind:checked=checked)
            }
        }
    }
}

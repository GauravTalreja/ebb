use sycamore::prelude::*;


#[derive(Prop)]
pub struct FilterProps<'a> {
    // term
    pub selectterm: &'a RcSignal<String>,
    pub currentterm: &'a RcSignal<bool>,
    pub nextterm: &'a RcSignal<bool>,
    //level
    pub level1: &'a RcSignal<bool>,
    pub level2: &'a RcSignal<bool>,
    pub level3: &'a RcSignal<bool>,
    pub level4: &'a RcSignal<bool>,  
    // status
    pub open: &'a RcSignal<bool>,
    pub closed: &'a RcSignal<bool>,
    // period
    pub morning: &'a RcSignal<bool>,
    pub afternoon: &'a RcSignal<bool>,
    pub evening: &'a RcSignal<bool>,
    // dates
    pub monday: &'a RcSignal<bool>,
    pub tuesday: &'a RcSignal<bool>,
    pub wednesday: &'a RcSignal<bool>,
    pub thursday: &'a RcSignal<bool>,
    pub friday: &'a RcSignal<bool>,
    
}

// TODO: for status, use a toggle with "include closed course"
// TODO: change level, period, dates to button-toggle, remove all_xxx
// TODO: change term to dropdown
#[component]
pub fn Filter<'a, G: Html>(
    cx: Scope<'a>,
    FilterProps { 
        selectterm,
        currentterm,
        nextterm,
        // level
        level1,
        level2,
        level3,
        level4,
        // status
        open,
        closed,
        // period
        morning,
        afternoon,
        evening,
        // dates
        monday,
        tuesday,
        wednesday,
        thursday,
        friday,              

     }: FilterProps<'a>,
) -> View<G> {

    view! { cx,
        div (class="flex flex-col shadow-md rounded-box bg-base-100") {
            p(class="rounded-box bg-primary p-4 font-bold text-primary-content text-lg") { "Filters" }

            // todo: associate state with the selected values
            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Term" }

                select (class="select select-primary w-full max-w-xs", bind:value=) {
                    option (disabled=true, selected=true) { "Select term"}
                    option (selected=selectcurrent) { "Spring 2023" }
                    option () { "Fall 2023" }
                }
                


                // div (class="dropdown") {
                //     label (tabindex="0", class="btn m-1") {
                //         "test"
                //     }
                //     ul (tabindex="0", class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52") {
                //         li { a {"Spring 2023"} }
                //         li { a {"Fall 2023"} }
                //     }
                // }

                // div (class="flex justify-center") {
                //     div (class="flex flex-row w-full gap-x-8") {
                //         CheckBox(name="Spring 2023".to_string(), checked=currentterm)
                //         CheckBox(name="Fall 2023".to_string(), checked=nextterm)                        
                //     }
                // }
            }

            // level    
            div(class="p-4 bg-base-100 rounded-md w-full ") {
                h3 (class="text-lg font-semibold") { "Course Level" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="1--".to_string(), checked=level1)
                        CheckBox(name="2--".to_string(), checked=level2) 
                        CheckBox(name="3--".to_string(), checked=level3)
                        CheckBox(name="4--".to_string(), checked=level4) 
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Course status" }
                div (class="flex justify-center") {
                    div (class="flex flex-row flex-wrap w-full gap-x-8") {
                        CheckBox(name="Open".to_string(), checked=open)
                        CheckBox(name="Closed".to_string(), checked=closed) 
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Period" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="Morning".to_string(), checked=morning)
                        CheckBox(name="Afternoon".to_string(), checked=afternoon) 
                        CheckBox(name="Evening".to_string(), checked=evening)
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Date" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="Mon".to_string(), checked=monday)
                        CheckBox(name="Tues".to_string(), checked=tuesday) 
                        CheckBox(name="Wed".to_string(), checked=wednesday)
                        CheckBox(name="Thur".to_string(), checked=thursday)
                        CheckBox(name="Fri".to_string(), checked=friday)
                    }
                }
            }
            
            

            
        }       
    }        
}


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

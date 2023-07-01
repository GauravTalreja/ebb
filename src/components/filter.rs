use sycamore::prelude::*;

#[derive(Prop)]
pub struct FilterProps<'a> {
    // term
    pub selectterm: &'a RcSignal<String>,
    //level
    pub level1: &'a RcSignal<bool>,
    pub level2: &'a RcSignal<bool>,
    pub level3: &'a RcSignal<bool>,
    pub level4: &'a RcSignal<bool>,  
    // status
    pub include_closed: &'a RcSignal<bool>,
    // period
    pub morning: &'a RcSignal<bool>,
    pub afternoon: &'a RcSignal<bool>,
    pub evening: &'a RcSignal<bool>,
    pub monday: &'a RcSignal<bool>,
    pub tuesday: &'a RcSignal<bool>,
    pub wednesday: &'a RcSignal<bool>,
    pub thursday: &'a RcSignal<bool>,
    pub friday: &'a RcSignal<bool>,
    pub saturday: &'a RcSignal<bool>,
    pub sunday: &'a RcSignal<bool>,
}

// TODO: for status, use a toggle with "include closed course"
// TODO: change level, period, dates to button-toggle, remove all_xxx
// TODO: change term to dropdown
#[component]
pub fn Filter<'a, G: Html>(
    cx: Scope<'a>,
    FilterProps { 
        // term
        selectterm,
        // level
        level1,
        level2,
        level3,
        level4,
        // status
        include_closed,
        // period
        morning,
        afternoon,
        evening,
        monday,
        tuesday,
        wednesday,
        thursday,
        friday,
        saturday,
        sunday,
    }: FilterProps<'a>,
) -> View<G> {

    view! { cx,
        div (class="flex flex-col shadow-md bg-base-100") {
            p(class="bg-primary p-4 font-bold text-primary-content text-lg") { "Filters" }

            // todo: associate state with the selected values
            div(class="p-4") {
                h3 (class="text-lg font-semibold mb-2") { "Term" }

                select (class="select select-primary w-full max-w-xs select-md", bind:value=selectterm) {
                    option (disabled=true, selected=true, value="selectterm".to_string()) { "Select term"}
                    option (value="currentterm") { "Spring 2023" }
                    option (value="nextterm") { "Fall 2023" }
                }


            }

            // level    
            div(class="p-4 bg-base-100 w-full ") {
                h3 (class="text-lg font-semibold") { "Course Level" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="1--".to_string(), checked=level1)
                        CheckBox(name="2--".to_string(), checked=level2) 
                        CheckBox(name="3--".to_string(), checked=level3)
                        CheckBox(name="4--".to_string(), checked=level4) 
                        // div (class="h-10 w-10") {
                        //     input (
                        //             type="checkbox",
                        //             class="btn btn-primary"
                        //         ) 
                        // }
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Course status" }
                div (class="flex") {
                    div (class="form-control w-52") {
                        label (class="cursor-pointer label") {
                            input (
                                    type="checkbox", 
                                    class="toggle toggle-primary",
                                    bind:checked=include_closed,
                                )
                            span (class="label-text") {"Include closed courses"}
                        }
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
                h3 (class="text-lg font-semibold") { "Day" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="Monday".to_string(), checked=monday)
                        CheckBox(name="Tuesday".to_string(), checked=tuesday)
                        CheckBox(name="Wednesday".to_string(), checked=wednesday)
                        CheckBox(name="Thursday".to_string(), checked=thursday)
                        CheckBox(name="Friday".to_string(), checked=friday)
                        CheckBox(name="Saturday".to_string(), checked=saturday)
                        CheckBox(name="Sunday".to_string(), checked=sunday)
                    }
                }
            }
        }
    }
}

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
        div (class="form-control w-full") {
            label (class="label cursor-pointer") {
                span (class="label-text") { (name) }
                input (
                    type="checkbox",
                    class="checkbox checkbox-primary",
                    bind:checked=checked)
            }
        }
    }
}

#[derive(Prop)]
pub struct ToggleProps<'a> {
    pub name: String,
    pub checked: &'a RcSignal<bool>,
}

#[component]
pub fn Toggle<'a, G: Html>(
    cx: Scope<'a>,
    ToggleProps { name, checked }: ToggleProps<'a>,
) -> View<G> {
    view! {cx,
        div (class="form-control w-full") {
            label (class="label cursor-pointer") {
                span (class="label-text") { (name) }
                input (
                    type="checkbox",
                    class="checkbox checkbox-primary",
                    bind:checked=checked)
            }
        }
    }
}

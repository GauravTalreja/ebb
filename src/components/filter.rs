use sycamore::prelude::*;

#[derive(Prop)]
pub struct FilterProps<'a> {
    pub term: &'a RcSignal<String>,
    pub level1: &'a RcSignal<bool>,
    pub level2: &'a RcSignal<bool>,
    pub level3: &'a RcSignal<bool>,
    pub level4: &'a RcSignal<bool>,
    pub include_closed: &'a RcSignal<bool>,
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
        term,
        level1,
        level2,
        level3,
        level4,
        include_closed,
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
                select (bind:value=term, class="select select-primary w-full") {
                    option (disabled=true) { "Select term"}
                    option (value="Spring 2023") { "Spring 2023" }
                    option (value="Fall 2023") { "Fall 2023" }
                }
            }

            div(class="p-4 bg-base-100 w-full ") {
                h3 (class="text-lg font-semibold") { "Level" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="1XX".to_string(), checked=level1)
                        CheckBox(name="2XX".to_string(), checked=level2)
                        CheckBox(name="3XX".to_string(), checked=level3)
                        CheckBox(name="4XX".to_string(), checked=level4)
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Status" }
                Toggle(name="Include closed".to_string(), checked=include_closed)
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

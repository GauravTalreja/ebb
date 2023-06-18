use sycamore::prelude::*;

#[component]
pub fn Filter<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="flex flex-col shadow-md rounded-box bg-base-100") {
            p(class="rounded-box bg-primary p-4 font-bold text-primary-content text-lg") { "Filters" }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Term" }
                div (class="flex justify-center") {
                    div (class="flex flex-row w-full gap-x-8") {
                        CheckBox(name="Spring 2023".to_string())
                        CheckBox(name="Fall 2023".to_string())
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Course Level" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="1--".to_string())
                        CheckBox(name="2--".to_string())
                        CheckBox(name="3--".to_string())
                        CheckBox(name="4--".to_string())
                        CheckBox(name="5--".to_string())
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Course status" }
                div (class="flex justify-center") {
                    div (class="flex flex-row flex-wrap w-full gap-x-8") {
                        CheckBox(name="Open".to_string())
                        CheckBox(name="Closed".to_string())
                        CheckBox(name="All".to_string())
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Period" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="Morning".to_string())
                        CheckBox(name="Afternoon".to_string())
                        CheckBox(name="Evening".to_string())
                        CheckBox(name="All".to_string())
                    }
                }
            }

            div(class="p-4") {
                h3 (class="text-lg font-semibold") { "Date" }
                div (class="flex justify-center") {
                    div (class="flex flex-wrap w-full gap-x-8") {
                        CheckBox(name="Mon".to_string())
                        CheckBox(name="Tues".to_string())
                        CheckBox(name="Wed".to_string())
                        CheckBox(name="Thur".to_string())
                        CheckBox(name="Fri".to_string())
                        CheckBox(name="All".to_string())
                    }
                }
            }
        }
    }
}

// filter section
#[derive(Prop)]
pub struct FilterScetionProps {
    title: String,
    // TODO: allow multiple filter selections
    // names: Vec<String>,
}

#[component]
pub fn FilterSection<G: Html>(
    cx: Scope,
    FilterScetionProps { title }: FilterScetionProps,
) -> View<G> {
    view! {cx,
        div(class="p-4 bg-base-100 rounded-md w-full shadow-md") {
            h3 (class="text-lg font-semibold") { (title) }
            div (class="flex justify-center") {
                div (class="flex flex-col w-1/2") {
                    CheckBox(name="testing".to_string())
                    CheckBox(name="testing".to_string())
                }
            }
        }
    }
}

// check box
#[derive(Prop)]
pub struct CheckBoxProps {
    name: String,
}

#[component]
pub fn CheckBox<G: Html>(cx: Scope, CheckBoxProps { name }: CheckBoxProps) -> View<G> {
    view! {cx,
        div (class="form-control") {
            label (class="cursor-pointer label") {
                span (class="label-text mr-8") { (name) }
                input (type="checkbox", class="checkbox checkbox-primary")
            }
        }
    }
}

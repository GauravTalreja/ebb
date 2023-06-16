use sycamore::prelude::*;

// table structure
#[component]
pub fn Filter<G: Html>(
    cx: Scope,
) -> View<G> {
    view! { cx,
        div (class="flex flex-col shadow-md rounded-md ") {
            // filter title
            div (class="bg-primary p-4 font-bold rounded-t-md") { 
                p(class="text-primary-content text-lg") { "Filter" }
            }
        
            // Term
            div(class="flex rounded-b-md") {
                FilterSection(title="Term".to_string())
                
            }
            div(class="flex rounded-b-md") {
                FilterSection(title="Time".to_string())
                
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
pub fn CheckBox<G: Html>(
    cx: Scope,
    CheckBoxProps { name }: CheckBoxProps,
) -> View<G> {
    view! {cx,
        div (class="form-control") {
            label (class="cursor-pointer label") {
                span (class="label-text") { (name) }
                input (type="checkbox", class="checkbox checkbox-primary")
            }
        }
    }
}



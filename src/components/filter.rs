use sycamore::prelude::*;

// table structure
#[component]
pub fn Filter<'a, G: Html>(
    cx: Scope<'a>,
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
pub fn FilterSection<'a, G: Html>(
    cx: Scope<'a>,
    FilterScetionProps { title }: FilterScetionProps,
) -> View<G> {
    view! {cx,
        div(class="relative p-4 bg-base-100 rounded-b-md w-full") {
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
pub fn CheckBox<'a, G: Html>(
    cx: Scope<'a>,
    CheckBoxProps { name }: CheckBoxProps,
) -> View<G> {
    view! {cx,
        label (class="cursor-pointer label") {
            span (class="label-text") { (name) }
            input (type="checkbox", class="checkbox checkbox-primary")
        }
    }
}



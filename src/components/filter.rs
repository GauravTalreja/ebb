use sycamore::prelude::*;

// table structure
#[component]
pub fn Filter<'a, G: Html>(
    cx: Scope<'a>,
) -> View<G> {
    view! { cx,
        div (class="relative flex flex-col") {
            // filter title
            div (class="text-xl font-bold") { "Filter" }
        
            // Term
            div(class="left-6") {
                FilterSection(title="Term".to_string())
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
        div(class="relative p-4 bg-base-100 shadow-md") {
            h3 (class="text-lg font-semibold") { (title) }
            div (class="flex flex-row justify-items-end w-1/2 left-1/2") {
                div (class="flex flex-col w-full") {
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



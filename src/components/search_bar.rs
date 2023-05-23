use sycamore::prelude::*;

#[derive(Prop)]
pub struct SearchBarProps<'a> {
    input: &'a RcSignal<String>,
}

#[component]
pub fn SearchBar<'a, G: Html>(cx: Scope<'a>, props: SearchBarProps<'a>) -> View<G> {
    view! { cx,
        div ( class = "dropdown dropdown-bottom w-full") {
            input (type="text", bind:value=props.input, placeholder="Search for courses", class="input input-bordered input-lg input-primary w-full")
            ul (
                class=format!(
                    "dropdown-content menu p-2 shadow bg-base-100 rounded-box w-full mt-2 {}",
                    if props.input.get().is_empty() {
                        "hidden"
                    } else {
                        ""
                    }
                )) {
                li {a (class="hover:bg-primary hover:text-white") {"Item 1"}}
                li {a (class="hover:bg-primary hover:text-white") {"Item 2"}}
                li {a (class="hover:bg-primary hover:text-white") {"Item 3"}}
                li {a (class="hover:bg-primary hover:text-white") {"Item 4"}}
                li {a (class="hover:bg-primary hover:text-white") {"Item 5"}}
                
            }
        }
    }
}

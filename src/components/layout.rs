pub use crate::components::search_bar::SearchBarProps;
use crate::components::{footer::Footer, search_bar::SearchBar};
use sycamore::prelude::*;

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        children,
        search_bar: SearchBarProps { input, results },
    }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header() {
            div (class="navbar bg-base-300 text-base-content") {
                a (class="hidden sm:flex btn btn-md lg:btn-lg btn-ghost normal-case font-bold text-2xl lg:text-4xl", href = "") { "UW Ebb" }
                SearchBar (input=input, results=results)
            }
        }
        main(class = "p-0 min-h-screen bg-base-200 w-full") {
            (children)
        }
        Footer
    }
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub children: Children<'a, G>,
    pub search_bar: SearchBarProps<'a>,
}

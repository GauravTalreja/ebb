use crate::components::{footer::Footer, search_bar::SearchBar, theme_change::ThemeChange};
pub use crate::components::{
    footer::FooterProps, search_bar::SearchBarProps, theme_change::ThemeProps,
};
use sycamore::prelude::*;

#[component]
pub fn Layout<'a, G: Html>(
    cx: Scope<'a>,
    LayoutProps {
        children,
        search_bar: SearchBarProps { input, results },
        theme: ThemeProps { state },
        footer: FooterProps { last_updated_time },
    }: LayoutProps<'a, G>,
) -> View<G> {
    let children = children.call(cx);

    view! { cx, div (data-theme=state) {
        header() {
            div (class="navbar bg-base-300 text-base-content") {
                a (class="hidden sm:inline-flex btn btn-md xl:btn-lg btn-ghost normal-case font-bold text-3xl xl:text-4xl", href = "") {
                    span (class="text-base-content") { "UW" }
                    span (class="text-primary") { "Ebb" }
                }
                SearchBar (input=input, results=results)
                ThemeChange (state=state)
            }
        }
        main(class = "p-0 min-h-screen bg-base-200 w-full") {
            (children)
        }
        Footer (last_updated_time=last_updated_time)
    }}
}

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub children: Children<'a, G>,
    pub search_bar: SearchBarProps<'a>,
    pub theme: ThemeProps<'a>,
    pub footer: FooterProps<'a>,
}

use perseus::prelude::*;
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        div (class="hero min-h-screen bg-base-200") {
            div (class="hero-content text-center") {
                div (class="max-w-md") {
                    h1 (class="text-5xl font-bold") { "UW Ebb" }
                    p (class="py-6") {"Explore thousands of models offered by the University of Waterloo. Plan your models. Get Recommendations."}
                    input (type="text", placeholder="Search for models", class="input input-bordered input-lg input-primary w-full max-w-xl")
                }
            }
        }
        footer (class="footer items-center p-4 bg-neutral text-neutral-content") {
            div (class="items-center grid-flow-col") {
                p {"Made for CS 348 with ♥"}
            }
            div (class="grid-flow-col gap-4 md:place-self-center md:justify-self-end") {
                a (class ="link link-hover", href = "about", id = "about-link") {"About"}
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "UW Ebb" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}

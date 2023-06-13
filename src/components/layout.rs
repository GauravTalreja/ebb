use sycamore::prelude::*;

#[component]
pub fn Layout<'a, G: Html>(cx: Scope<'a>, LayoutProps { children }: LayoutProps<'a, G>) -> View<G> {
    let children = children.call(cx);

    view! { cx,
        // These elements are styled with bright colors for demonstration purposes
        header() {
            div (class="navbar bg-base-100")
            a (class="btn btn-ghost normal-case text-xl") { "Ebb" }
        }
        main(class = "p-4") {
            (children)
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

#[derive(Prop)]
pub struct LayoutProps<'a, G: Html> {
    pub children: Children<'a, G>,
}

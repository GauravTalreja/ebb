use sycamore::prelude::*;

#[derive(Prop)]
pub struct SearchBarProps<'a> {
    input: &'a RcSignal<String>,
    // TODO: This is a vector of strings for Milestone 0, but we should have a fancier state with (cached?) strongly typed search results
    results: &'a RcSignal<Vec<String>>,
}

#[component]
pub fn SearchBar<'a, G: Html>(
    cx: Scope<'a>,
    SearchBarProps { input, results }: SearchBarProps<'a>,
) -> View<G> {
    view! { cx,
        div ( class="dropdown dropdown-bottom w-full") {
            input (type="text", bind:value=input, placeholder="Search for courses", class="input input-bordered input-lg input-primary w-full", tabindex="0")
            ul (tabindex="0", class=format!(
                    "dropdown-content menu p-2 shadow bg-base-100 rounded-box w-full mt-2 {}",
                    if input.get().is_empty() {
                        "hidden"
                    } else {
                        ""
                    }
                )) {
                // TODO: Used Keyed with a UID Key instead
                Indexed(
                    iterable=results,
                    view=|cx, result| view! { cx,
                        SearchResult(name=result)
                    },
                )
                SearchResult(name="BOTTOM TEXT".to_string())
            }
        }
    }
}

#[derive(Prop)]
pub struct SearchResultProps {
    name: String,
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn SearchResult<G: Html>(cx: Scope, SearchResultProps { name }: SearchResultProps) -> View<G> {
    view! { cx,
        li {a (class="hover:bg-primary hover:text-primary-content") { (name) }}
    }
}

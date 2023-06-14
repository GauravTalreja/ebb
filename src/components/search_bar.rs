#[cfg(client)]
use crate::models::Course;
use perseus::prelude::*;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct SearchBarProps<'a> {
    pub input: &'a RcSignal<String>,
    // TODO: This is a vector of strings for Milestone 0, but we should have a fancier state with (cached?) strongly typed search results
    pub results: &'a RcSignal<Vec<String>>,
}

// TODO: Improve this with support for mobile and arrow keys, see https://github.com/metonym/svelte-typeahead/blob/master/src/Typeahead.svelte for inspiration
#[component]
pub fn SearchBar<'a, G: Html>(
    cx: Scope<'a>,
    SearchBarProps { input, results }: SearchBarProps<'a>,
) -> View<G> {
    #[cfg(client)]
    create_effect_scoped(cx, |cx| {
        if !input.get().is_empty() {
            spawn_local_scoped(cx, async {
                let body = reqwasm::http::Request::get(
                    format!("/api/v1/courses/{}", input.get()).as_str(),
                )
                .send()
                .await
                .unwrap()
                .json::<Vec<Course>>()
                .await
                .unwrap()
                .iter()
                .map(|course| course.name.clone())
                .collect();
                results.set(body);
            })
        } else {
            results.set(vec![]);
        }
    });

    view! { cx,
        div (class="w-full px-2") {
            div ( class="dropdown dropdown-bottom w-full") {
                input (
                    type="search",
                    bind:value=input,
                    placeholder="Search for courses",
                    class="input input-bordered input-md md:input-lg input-primary w-full",
                    tabindex="0"
                )
                ul (
                    tabindex="0",
                    class="dropdown-content menu p-2 shadow bg-base-100 rounded-box mt-2 w-full display:none focus-within:display:block",
                ) {
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

use models::CourseSummary;
use perseus::prelude::*;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct SearchBarProps<'a> {
    pub input: &'a RcSignal<String>,
    // TODO: This is a vector of strings for Milestone 0, but we should have a fancier state with (cached?) strongly typed search results
    pub results: &'a RcSignal<Vec<CourseSummary>>,
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
                let mut body = reqwasm::http::Request::get(
                    format!("/api/v1/courses/{}", input.get()).as_str(),
                )
                .send()
                .await
                .unwrap()
                .json::<Vec<CourseSummary>>()
                .await
                .unwrap();

                let max_len = 6;
                if body.len() > max_len {
                    (0..(body.len() - max_len)).for_each(|_| {
                        body.pop();
                    })
                }
                results.set(body);
            })
        } else {
            results.set(vec![]);
        }
    });

    view! { cx,
        div (class="w-full px-2") {
            div ( class="dropdown dropdown-bottom w-full z-30") {
                input (
                    type="search",
                    bind:value=input,
                    placeholder="Search for courses",
                    class="input input-bordered input-md text-base xl:input-lg input-primary w-full",
                    tabindex="0"
                )
                ul (
                    tabindex="0",
                    class="dropdown-content z-20 menu xl:menu-lg p-2 shadow-lg bg-base-200 rounded-box mt-2 w-full display:none focus-within:display:block",
                ) {
                    // TODO: Used Keyed with a UID Key instead
                    Indexed(
                        iterable=results,
                        view=|cx, result| view! { cx,
                            SearchResult(subject = result.subject_code, catalogue = result.catalog_number , title=result.title)
                        },
                    )
                    li {
                        a (href=format!("courses/{}", input.get().as_str()), class="bg-primary text-primary-content hover:bg-primary-focus") {
                            (format!("View all {} courses", input.get().as_str()))
                        }
                    }
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct SearchResultProps {
    subject: String,
    catalogue: String,
    title: String,
}
// TODO: Make results fancier than plain text
// TODO: Use custom styles based on what the search result is, would require pattern matching a more strongly typed result
// (ex. Math Course -> Primary Color, Eng Course -> Secondary Color etc.)
#[component]
fn SearchResult<G: Html>(
    cx: Scope,
    SearchResultProps {
        subject,
        catalogue,
        title,
    }: SearchResultProps,
) -> View<G> {
    let path = subject.clone() + &catalogue;
    view! { cx,
        li {a (href=format!("c/{path}")) { (format!("{subject} {catalogue} - {title}")) }}
    }
}

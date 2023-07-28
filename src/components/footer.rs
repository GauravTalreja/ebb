use sycamore::prelude::*;
use models::LastUpdated;

#[component]
pub fn Footer<'a, G: Html>(cx: Scope<'a>, FooterProps { last_updated_time }: FooterProps<'a>) -> View<G> {
    view! {cx,
        footer (class="footer items-center p-4 bg-neutral text-neutral-content") {
            div (class="items-center grid-flow-col") {
                p {"Made for CS 348 with ♥"}
                p { 
                    (match last_updated_time.get().date_time {
                        Some(time) => format!("\t\t\t\tLast Updated @ {} (UTC)", time.format("%b %e, %Y %H:%M:%S")),
                        None => String::from("Not Updated Yet"),
                    }) 
                }
            }
            div (class="grid-flow-col gap-4 md:place-self-center md:justify-self-end") {
                a (class ="link link-hover", href = "about", id = "about-link") {"About"}
            }
        }
    }
}

#[derive(Prop)]
pub struct FooterProps<'a> {
    pub last_updated_time: &'a RcSignal<LastUpdated>,
}
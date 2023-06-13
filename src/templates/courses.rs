use crate::components::layout::Layout;
use perseus::prelude::*;
use sycamore::prelude::*;

fn courses_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        link ( rel="stylesheet", href="/tailwind.css")
        Layout {
            p {"AAAAAA"}
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Search for courses" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("courses")
        .view(courses_page)
        .head(head)
        .build()
}

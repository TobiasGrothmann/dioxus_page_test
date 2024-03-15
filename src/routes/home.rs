use dioxus::prelude::*;

use crate::{
    components::{footer::Footer, intro::Intro, nav::Nav},
    routes::router::Router,
};

#[component]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./home.css") }
        div { class: "Home main-page",
            Nav {
                current_route: Router::Home {},
            }
            Intro {
                headline: "a headline".to_string(),
                sub_headline: "a subheadline".to_string()
            }
            div { class: "content-row",
                img {
                    src: "img/cat.jpg",
                }
            }
            Footer {}
        }
    })
}

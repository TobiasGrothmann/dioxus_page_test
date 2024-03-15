use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

use crate::routes::router::Router;

#[derive(PartialEq, Props)]
pub struct NavProps {
    current_route: Router,
}

#[component]
pub fn Nav(cx: Scope<NavProps>) -> Element {
    let mut news_class = "";
    match cx.props.current_route {
        Router::Home { .. } => {
            news_class = "nav_button_selected";
        }
    }

    cx.render(rsx! {
        style { include_str!("./nav.css") }
        div { class: "Nav",
            div { class: "nav_button_group",
                div { class: "nav_button",
                    button { class: "nav_button {news_class}",
                        onclick: move |_event| {
                            use_navigator(cx).push(Router::Home {  });
                        },
                        "Home",
                    }
                }
            }
        }
    })
}

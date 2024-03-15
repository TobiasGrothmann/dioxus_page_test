use dioxus::prelude::*;

#[component]
pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./footer.css") }
        div { class: "Footer",
            div { class: "footer_social",
                a { class: "footer_social_link", href: "www.google.com", "Google" }
            }
        }
    })
}

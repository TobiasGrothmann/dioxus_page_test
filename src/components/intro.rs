use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct IntroProps {
    pub headline: String,
    pub sub_headline: Option<String>,
}

#[component]
pub fn Intro(cx: Scope<IntroProps>) -> Element {
    cx.render(rsx! {
        style { include_str!("./intro.css") }
        div { class: "Intro",
            div { class: "logo",
                a { href: "/",
                    img { src: "logo/logo.svg" }
                }
            }
            div { class: "intro-content",
                h1 { "{cx.props.headline}" }
                if cx.props.sub_headline.is_some() {
                    cx.render(rsx!{
                        h2 { "{cx.props.sub_headline.clone().unwrap()}" }
                    })
                }
            }
        }
    })
}

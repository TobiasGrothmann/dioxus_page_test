#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_web::Config;
use log::LevelFilter;
use routes::router;

mod components;
mod routes;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("./main.css") }
        Router::<router::Router> {}
    })
}

fn main() {
    dioxus_logger::DioxusLogger::new(LevelFilter::Debug)
        .use_format("[{LEVEL}] {PATH} - {ARGS}")
        .build()
        .expect("failed to init logger");

    dioxus_web::launch_cfg(App, Config::default());
}

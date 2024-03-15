use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::routes::home::Home;

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Router {
    #[route("/")]
    Home {},
}

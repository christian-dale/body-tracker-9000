use dioxus::prelude::*;

mod routes;
pub mod components;

use routes::Route;

fn main() {
    dioxus::launch(|| rsx! { Router::<Route> {} });
}

use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Wrapper() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") },
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") },

        header {
            class: "h-16 m-4 border rounded-sm flex items-center px-8 justify-between",
            Link {
                to: Route::Home {},
                "Body Tracker 9000"
            }
        },
        main {
            class: "container mx-auto px-4 my-8",
            Outlet::<Route> {}
        }
    }
}

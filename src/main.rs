use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| rsx! { Router::<Route> {} });
}

#[derive(Routable, Clone)]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/add-entry")]
        AddEntry {}
}

#[component]
fn Wrapper() -> Element {
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

#[component]
fn Home() -> Element {
    rsx! {
        h1 {
            class: "text-3xl font-bold tracking-tight",
            "Welcome!"
        },
        div {
            class: "quick-actions my-8 flex space-x-4",
            Link {
                to: Route::AddEntry {  },
                class: "border px-4 py-2 rounded",
                "Add Entry"
            }
        },
        div {
            class: "bg-white rounded-sm border p-4",
            "Info"
        }
    }
}

#[component]
fn AddEntry() -> Element {
    rsx! {
        h1 { "Add Entry" }
    }
}

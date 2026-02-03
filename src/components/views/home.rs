use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Home() -> Element {
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

use dioxus::prelude::*;

#[component]
pub fn AddEntry() -> Element {
    rsx! {
        div { class: "p-8 mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Add New Entry" },

            div {
                class: "flex gap-8 w-full my-8",

                div {
                    class: "flex-1",
                    input {
                        class: "border p-2 rounded w-full",
                        placeholder: "Enter title..."
                    },
                },

                div {
                    class: "flex-1 flex flex-col gap-4",
                    input { class: "border p-2 rounded w-full", placeholder: "Weight" },
                    input { class: "border p-2 rounded w-full", placeholder: "Waist (cm)" },
                    input { class: "border p-2 rounded w-full", placeholder: "Chest (cm)" },
                    input { class: "border p-2 rounded w-full", placeholder: "Arms (cm)" },
                    input { class: "border p-2 rounded w-full", placeholder: "Forearms (cm)" },
                    input { class: "border p-2 rounded w-full", placeholder: "Thighs (cm)" }
                }
            },

            div {
                class: "flex flex-col gap-4",
                button {
                    class: "bg-gray-400 text-white p-2 rounded cursor-pointer hover:bg-gray-500",
                    "Save Entry"
                }
            }
        }
    }
}

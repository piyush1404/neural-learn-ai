use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    rsx!(
        div {
            class: "flex justify-between items-center p-4 border-b",
            div {
                class: "flex items-center space-x-4",
                span { class: "text-blue-500 font-semibold text-xl", "Neural Vision" }
                button { class: "text-sm px-3 py-1 border rounded", "Home" }
            }
            div {
                class: "flex items-center space-x-3",
                input {
                    r#type: "text",
                    placeholder: "Search",
                    class: "border px-3 py-1 rounded"
                }
                select {
                    class: "border px-3 py-1 rounded",
                    option { value: "default", "Select Engine" }
                    option { value: "neuro", "Neuro Shield" }
                    option { value: "brilliant", "Brilliant" }
                }
            }
        }
    )
}

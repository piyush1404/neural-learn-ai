use dioxus::prelude::*;

#[component]
pub fn NewProjectCard() -> Element {
    rsx!(
        div {
            class: "border border-dashed rounded-lg flex items-center justify-center h-48 text-blue-500 cursor-pointer",
            "+ New Project"
        }
    )
}

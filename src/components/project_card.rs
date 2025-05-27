use dioxus::prelude::*;

#[component]
pub fn ProjectCard(
    title: String,
    engine: String,
    file_roi: String,
    min_if: String,
    max_if: String,
    search_area: String,
    nn_capacity: String,
    neurons: String,
    created: String,
    edited: String,
) -> Element {
    rsx!(
        div {
            class: "border rounded-lg p-4 shadow-sm",
            h2 { class: "font-semibold", "{title}" }
            p { class: "text-xs", "Created: {created} | Last edited: {edited}" }
            div { class: "mt-2 text-sm",
                p { "🧠 Engine: {engine}" }
                p { "📁 File ROI: {file_roi}" }
                p { "Min if: {min_if}, Max if: {max_if}, Search area: {search_area}" }
                span {
                    class:"flex gap-2",
                    p { class: "bg-gray-200 px-3 py-1 rounded" ,"NN Capacity: {nn_capacity}" }
                    p { class: "bg-gray-200 px-3 py-1 rounded" ,"Neurons: {neurons}"}
                }
            }
            div { class: "mt-3 flex justify-end gap-2",
                button { class: "bg-gray-200 px-3 py-1 rounded", "Delete" }
                button { class: "bg-black text-white px-3 py-1 rounded", "Edit" }
            }
        }
    )
}

use dioxus::prelude::*;

use crate::project_store::{NeuronConfig, Project};

#[component]
pub fn ProjectCard(
    name: String,
    platform: String,
    interface: String,
    description: String,
    created_at: String,
    updated_at: String,
    neurons: Option<NeuronConfig>,
) -> Element {
    let (min_if, max_if, search_area_str, total_neurons, committed_neurons) =
        if let Some(n) = &neurons {
            (
                Some(n.min_if),
                Some(n.max_if),
                Some(
                    n.search_area
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
                Some(n.total_neurons),
                Some(n.committed_neurons),
            )
        } else {
            (None, None, None, None, None)
        };

    rsx! {
        div {
            class: "border rounded-lg p-4 shadow-sm",
            h2 { class: "font-semibold text-lg", "{name}" }
            p { class: "text-xs text-gray-600", "Created: {created_at} | Last edited: {updated_at}" }
            div { class: "mt-2 text-sm text-gray-800",
                p { "💻 Platform: {platform}" }
                // p { "🔌 Interface: {interface}" }
                // Uncomment below if description is needed
                // p { "📝 Description: {description}" }

                if let (Some(min), Some(max)) = (min_if, max_if) {
                    p { "🧠 Min IF: {min}, Max IF: {max}" }
                }
                if let Some(search) = &search_area_str {
                    p { "📊 Search Area: {search}" }
                }
                if let Some(total) = total_neurons {
                    p { "📐 Total Neurons: {total}" }
                }
                if let Some(committed) = committed_neurons {
                    p { "✅ Committed Neurons: {committed}" }
                }
            }

            div { class: "mt-3 flex justify-end gap-2",
                button { class: "bg-gray-200 px-3 py-1 rounded", "Delete" }
                button { class: "bg-black text-white px-3 py-1 rounded", "Edit" }
            }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn ProjectCard(
    name: String,
    platform: String,
    interface: String,
    description: String,
    created_at: String,
    updated_at: String,
    // roi_width: u32,
    // roi_height: u32,
    // min_if: u32,
    // max_if: u32,
    // search_area: String, // formatted as "x, y, width, height"
    // total_neurons: u32,
    // committed_neurons: u32,
) -> Element {
    rsx! {
        div {
            class: "border rounded-lg p-4 shadow-sm",
            h2 { class: "font-semibold text-lg", "{name}" }
            p { class: "text-xs text-gray-600", "Created: {created_at} | Last edited: {updated_at}" }
            div { class: "mt-2 text-sm text-gray-800",
                p { "💻 Platform: {platform}" }
                p { "🔌 Interface: {interface}" }
                p { "📝 Description: {description}" }
                // p { "📐 ROI: {roi_width}x{roi_height} px" }
                // p { "🧠 Min IF: {min_if}, Max IF: {max_if}" }
                // p { "📊 Search Area: {search_area}" }
                span {
                    class: "flex gap-2 mt-2",
                    // p { class: "bg-gray-200 px-3 py-1 rounded", "Total Neurons: {total_neurons}" }
                    // p { class: "bg-gray-200 px-3 py-1 rounded", "Committed Neurons: {committed_neurons}" }
                }
            }
            div { class: "mt-3 flex justify-end gap-2",
                button { class: "bg-gray-200 px-3 py-1 rounded", "Delete" }
                button { class: "bg-black text-white px-3 py-1 rounded", "Edit" }
            }
        }
    }
}

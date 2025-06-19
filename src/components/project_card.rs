use crate::project_store::NeuronConfig;
use dioxus::prelude::*;

use crate::date_format::format_date_mmddyyyy;
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

    // Pre-formatted display strings
    let min_if_str = min_if.map(|v| format!("Min if: {:02}", v));
    let max_if_str = max_if.map(|v| format!("Max if: {}", v));
    let search_area_display = search_area_str
        .as_ref()
        .map(|s| format!("Search area: [{}]", s));
    let nn_capacity_str = total_neurons.map(|v| format!("NN Capacity: {}", v));
    let committed_str = committed_neurons.map(|v| format!("Neurons:{}", v));

    let min_parts = min_if_str.as_ref().and_then(|s| {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    });

    let max_parts = max_if_str.as_ref().and_then(|s| {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    });

    let search_parts = search_area_display.as_ref().and_then(|s| {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    });

    let nn_parts = nn_capacity_str.as_ref().and_then(|s| {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    });

    let committed_parts = committed_str.as_ref().and_then(|s| {
        let parts: Vec<&str> = s.split(":").collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    });

    created_at = format_date_mmddyyyy(created_at.as_str());
    updated_at = format_date_mmddyyyy(updated_at.as_str());

    rsx! {
        div {
            class: "border border-[#BEBEBE] rounded-xl p-[15px] shadow-sm flex flex-col gap-1",

            // Header with icon and name
            div {
                class: "flex items-center gap-3",
                div {
                    class: "w-8 h-8 border-[0.5px] border-[#BEBEBE] rounded-full bg-[#FFFFFF] flex items-center justify-center",
                    span {
                        class:"h-[15px] w-[15px] flex items-center justify-center",
                        "🖼️"
                    }
                }
                h2 {
                    class: "font-normal text-sm text-[#151515]",
                    "{name}"
                }
            }

            // Metadata
            p {
                class: "text-[9px] text-[#787878]",
                "Created : {created_at}    Last edited : {updated_at}"
            }

            hr { class: "border-t border-gray-200 my-1" }

            // Platform info
            div {
                class: "flex items-center gap-2 text-[10px] text-[#787878]",
                "🧰",
                span { "Platform:" }
                span { class: "text-[#158826] font-medium", "{platform}" }
                span { "🔗" }
            }




            // Metrics
            div {
                class: "flex flex-col gap-1 text-[13px] mt-1",

                // First row: min_if, max_if, search_area
                div {
                    class: "flex flex-wrap gap-1",
                    if let Some((key, val)) = min_parts {
                        span {
                            class: "bg-[#F0F0F0] px-2 py-1 font-normal rounded text-[10px] flex gap-1",
                            span { class: "text-[#555555]", "{key}:" }
                            span { class: "text-[#151515] ", "{val}" }
                        }
                    }
                    if let Some((key, val)) = max_parts {
                        span {
                            class: "bg-[#F0F0F0] px-2 py-1 font-normal rounded text-[10px] flex gap-1",
                            span { class: "text-[#555555]", "{key}:" }
                            span { class: "text-black ", "{val}" }
                        }
                    }
                    if let Some((key, val)) = search_parts {
                        span {
                            class: "bg-[#F0F0F0] px-2 py-1 font-normal rounded text-[10px] flex gap-1",
                            span { class: "text-[#555555]", "{key}:" }
                            span { class: "text-black ", "{val}" }
                        }
                    }
                }

                // Second row: nn_capacity, committed_neurons
                div {
                    class: "flex flex-wrap gap-1",
                    if let Some((key, val)) = nn_parts {
                        span {
                            class: "bg-[#F0F0F0] px-2 py-1 font-normal rounded text-[10px] flex gap-1",
                            span { class: "text-[#555555]", "{key}:" }
                            span { class: "text-black ", "{val}" }
                        }
                    }
                    if let Some((key, val)) = committed_parts {
                        span {
                            class: "bg-[#F0F0F0] px-2 py-1 font-normal rounded text-[10px] flex gap-1",
                            span { class: "text-[#555555]", "{key}:" }
                            span { class: "text-black", "{val}" }
                        }
                    }
                }
            }




            hr { class: "border-t border-gray-200 my-2" }

            // Footer actions
            div {
                class: "flex justify-between items-center",

                div {
                    class: "text-green-700 text-xl",
                    "🧠"
                }

                div {
                    class: "flex gap-[10px]",
                    button {
                        class: "bg-[#F0F0F0] px-[10px] py-1 rounded-[3px] text-xs font-medium text-[#101010] ",
                        "Delete"
                    }
                    button {
                        class: "bg-[#101010] px-[10px] py-1 rounded-[3px] text-xs font-medium text-[#FFFFFF] ",
                        "Edit"
                    }
                }
            }
        }
    }
}

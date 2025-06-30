use dioxus::prelude::*;

#[component]
pub fn KnowledgeModal(onclose: EventHandler<MouseEvent>) -> Element {
    let mut selected_tab = use_signal(|| "All".to_string());
    let mut active_subtab = use_signal(|| "Models".to_string());
    let mut dropdown_value = use_signal(|| "All Categories".to_string());

    let tab_items = vec!["All", "Image 1", "Image 2"];

    rsx!(
        div {
            class: "fixed inset-0 flex items-center justify-center bg-black bg-opacity-30 z-50",

            div {
                class: "w-[550px] h-[300px] bg-white rounded p-4",

                // Header
                div {
                    class: "flex justify-between items-center border-b pb-2 mb-4",
                    span {
                        class: "text-gray-500 text-sm font-medium",
                        "Knowledge"
                    }
                    button {
                        class: "text-xl text-gray-600 hover:text-black",
                        onclick: move |e| onclose.call(e),
                        "×"
                    }
                }

                // Body
                div {
                    class: "flex w-[500px] h-[245px]",

                    // Left Sidebar Tabs
                    div {
                        class: "w-1/4 border-r pr-2 space-y-2 text-sm text-[#404040]",
                        {
                            tab_items.iter().map(|item| {
                                let item = item.to_string();
                                let selected = *selected_tab.read() == item;

                                rsx!(
                                    div {
                                        class: format_args!(
                                            "cursor-pointer px-2 py-1 rounded {}",
                                            if selected {
                                                "text-[#0387D9] font-medium"
                                            } else {
                                                "hover:bg-gray-100"
                                            }
                                        ),
                                        onclick: move |_| selected_tab.set(item.clone()),
                                        "{item}"
                                    }
                                )
                            })
                        }
                    }

                    // Right Panel
                    div {
                        class: "w-3/4 pl-4",

                        // Subtabs: Models / Details
                        div {
                            class: "flex space-x-4 mb-3 border-b text-sm",
                            div {
                                class: format_args!(
                                    "pb-1 cursor-pointer {}",
                                    if *active_subtab.read() == "Models" {
                                        "text-[#0387D9] border-b-2 border-[#0387D9]"
                                    } else {
                                        "text-gray-500"
                                    }
                                ),
                                onclick: move |_| active_subtab.set("Models".to_string()),
                                "Models"
                            }
                            div {
                                class: format_args!(
                                    "pb-1 cursor-pointer {}",
                                    if *active_subtab.read() == "Details" {
                                        "text-[#0387D9] border-b-2 border-[#0387D9]"
                                    } else {
                                        "text-gray-500"
                                    }
                                ),
                                onclick: move |_| active_subtab.set("Details".to_string()),
                                "Details"
                            }
                        }

                        // Conditional Content
                        {
                            if *active_subtab.read() == "Models" {
                                rsx!(
                                    // Dropdown
                                    div {
                                        class: "mb-3 relative",
                                        select {
                                            class: "appearance-none border border-black rounded px-2 py-1 text-sm text-gray-500 w-full pr-6",
                                            value: "{dropdown_value}",
                                            onchange: move |e| dropdown_value.set(e.value()),

                                            option { value: "All Categories", "All Categories" }
                                            option { value: "All Categories 1", "All Categories 1" }
                                            option { value: "All Categories 2", "All Categories 2" }
                                        }

                                        // Dropdown arrow
                                        div {
                                            class: "pointer-events-none absolute top-1/2 right-2 transform -translate-y-1/2 text-gray-600",
                                            svg {
                                                width: "10",
                                                height: "6",
                                                view_box: "0 0 10 6",
                                                fill: "none",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                path {
                                                    d: "M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z",
                                                    fill: "#555555"
                                                }
                                            }
                                        }
                                    }

                                    // Masked Fields
                                    div {
                                        class: "space-y-2",
                                        input {
                                            r#type: "text",
                                            disabled: true,
                                            value: "■■■■■■■■■■■■■■■■■■",
                                            class: "w-full border border-gray-300 rounded px-2 py-1 bg-gray-200 text-gray-600 tracking-widest"
                                        }
                                        input {
                                            r#type: "text",
                                            disabled: true,
                                            value: "■■■■■■■■■■■■■■■■■■",
                                            class: "w-full border border-gray-300 rounded px-2 py-1 bg-gray-200 text-gray-600 tracking-widest"
                                        }
                                    }
                                )
                            } 
                            else {
                                rsx!(
                                    div {
                                        class: "grid grid-cols-6 text-sm",
                                
                                        // Neuron ID Column
                                        div { class: "bg-[#555555] text-black px-2 py-1 border text-center", "Neuron ID" }
                                        div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "1" }
                                        div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "2" }
                                        div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "3" }
                                        div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "4" }
                                        div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "5" }
                                
                                        // Category Column
                                        div { class: "bg-[#555555] text-black px-2 py-1 border text-center", "Category" }
                                        { (0..5).map(|_| rsx!(
                                            div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "1" }
                                        )) }
                                
                                        // NCR Column
                                        div { class: "bg-[#555555] text-black px-2 py-1 border text-center", "NCR" }
                                        { (0..5).map(|_| rsx!(
                                            div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "Object" }
                                        )) }
                                
                                        // Model Column
                                        div { class: "bg-[#555555] text-black px-2 py-1 border text-center", "Model" }
                                        { (0..5).map(|_| rsx!(
                                            div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "Model" }
                                        )) }
                                
                                        // Active IF Column
                                        div { class: "bg-[#555555] text-black px-2 py-1 border text-center", "Active IF" }
                                        { ["2378", "2379", "2380", "2381", "2382"].iter().map(|v| rsx!(
                                            div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "{v}" }
                                        )) }
                                
                                        // Min IF Column
                                        div { class: "bg-[#555555] text-black px-2 py-1 border text-center", "Min IF" }
                                        { (0..5).map(|_| rsx!(
                                            div { class: "bg-[#EDEDED] text-[#555555] px-2 py-1 border text-center", "2" }
                                        )) }
                                
                                        // Degenerated Column with checkbox
                                        div { class: "bg-[#555555] text-black px-2 py-1 border text-center", "Degenerated" }
                                        { (0..5).map(|_| rsx!(
                                            div {
                                                class: "bg-[#EDEDED] px-2 py-1 border flex justify-center items-center",
                                                input {
                                                    r#type: "checkbox",
                                                    class: "form-checkbox w-4 h-4 accent-[#0387D9]"
                                                }
                                            }
                                        )) }
                                    }
                                )
                                
                            }
                        }
                    }
                }
            }
        }
    )
}
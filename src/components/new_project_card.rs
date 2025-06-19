use dioxus::prelude::*;

#[component]
pub fn NewProjectCard() -> Element {
    let mut show_modal = use_signal(|| false);

    rsx! {
        // New Project Card Trigger
        div {
            class: "border border-[#BEBEBE] rounded-xl shadow-sm flex flex-col items-center justify-center gap-2 cursor-pointer",
            onclick: move |_| show_modal.set(true),

            // Dashed box with "+"
            div {
                class: "w-20 h-20 border-2 border-dashed border-[#999999] flex items-center justify-center text-[#0387D9] text-xl rounded-sm",
                "+"
            }

            // Label text
            span {
                class: "text-[#0387D9] text-sm font-medium",
                "New Project"
            }
        }


        // Modal
        if *show_modal.read() {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                div {
                    class: "bg-white p-6 rounded-lg w-[900px] max-h-[95vh] overflow-y-auto",
                    h2 { class: "text-xl font-bold mb-4", "Project Details" }

                    // Project Name
                    div { class: "mb-3",
                        label { class: "block text-sm font-medium mb-1", "Project Name" }
                        input {
                            class: "w-full border p-2 rounded",
                            placeholder: "Sensor Chip"
                        }
                    }

                    // Engine selection
                    div { class: "mb-3",
                        label { class: "block text-sm font-medium mb-1", "Select Engine" }
                        select {
                            class: "w-full border p-2 rounded",
                            option { "Simulation" }
                            option { "Neuro Shield" }
                            option { "Predictive Engine" }
                        }
                    }

                    // Description
                    div { class: "mb-3",
                        label { class: "block text-sm font-medium mb-1", "Description" }
                        textarea {
                            class: "w-full border p-2 rounded",
                            placeholder: "Project description..."
                        }
                        div { class: "text-right text-xs text-gray-500", "90/100 alphabets" }
                    }

                    // Categories
                    div { class: "mb-4",
                        label { class: "block text-sm font-medium mb-1", "Categories" }

                        // Background Category
                        div { class: "flex items-center mb-2 gap-2",
                            input {
                                r#type: "text",
                                value: "Background",
                                class: "flex-1 border p-2 rounded",
                                readonly: true
                            }
                            input {
                                r#type: "color",
                                value: "#000000",
                                class: "w-10 h-10 p-0 border rounded"
                            }
                        }

                        // Object Category
                        div { class: "flex items-center gap-2",
                            input {
                                r#type: "text",
                                value: "Object",
                                class: "flex-1 border p-2 rounded",
                                readonly: true
                            }
                            input {
                                r#type: "color",
                                value: "#FF5A5A",
                                class: "w-10 h-10 p-0 border rounded"
                            }
                        }
                    }

                    // Feature Extraction
                    div { class: "mb-4",
                        h3 { class: "text-sm font-semibold mb-2", "Feature Extraction" }
                        div { class: "grid grid-cols-2 gap-4",

                            // Width and Height
                            div {
                                label { class: "block text-sm mb-1", "Width" }
                                input {
                                    r#type: "number",
                                    value: "16",
                                    class: "w-full border p-2 rounded"
                                }
                            }
                            div {
                                label { class: "block text-sm mb-1", "Height" }
                                input {
                                    r#type: "number",
                                    value: "16",
                                    class: "w-full border p-2 rounded"
                                }
                            }

                            // Block Width and Height
                            div {
                                label { class: "block text-sm mb-1", "Block Width" }
                                input {
                                    r#type: "number",
                                    value: "1",
                                    class: "w-full border p-2 rounded"
                                }
                            }
                            div {
                                label { class: "block text-sm mb-1", "Block Height" }
                                input {
                                    r#type: "number",
                                    value: "1",
                                    class: "w-full border p-2 rounded"
                                }
                            }

                            // Max IF and Min IF
                            div {
                                label { class: "block text-sm mb-1", "Max" }
                                input {
                                    r#type: "number",
                                    value: "16878",
                                    class: "w-full border p-2 rounded"
                                }
                            }
                            div {
                                label { class: "block text-sm mb-1", "Min" }
                                input {
                                    r#type: "number",
                                    value: "16",
                                    class: "w-full border p-2 rounded"
                                }
                            }

                            // Algorithm
                            div {
                                label { class: "block text-sm mb-1", "Algorithm" }
                                select {
                                    class: "w-full border p-2 rounded",
                                    option { "Subsample" }
                                    option { "Full Sample" }
                                }
                            }

                            // Normalize
                            div { class: "flex items-center mt-6",
                                input {
                                    r#type: "checkbox",
                                    class: "mr-2"
                                }
                                label { "Normalize" }
                            }
                        }
                    }

                    // Buttons
                    div { class: "flex justify-between mt-6",
                        div {
                            button {
                                class: "px-4 py-2 bg-gray-200 rounded hover:bg-gray-300",
                                "Reset"
                            }
                        }
                        div { class: "flex gap-2",
                            button {
                                class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                                "Suggest"
                            }
                            button {
                                class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                                "Validate"
                            }
                            button {
                                class: "px-4 py-2 bg-black text-white rounded hover:bg-gray-800",
                                onclick: move |_| show_modal.set(false),
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 bg-blue-700 text-white rounded hover:bg-blue-800",
                                "Start"
                            }
                        }
                    }
                }
            }
        }
    }
}

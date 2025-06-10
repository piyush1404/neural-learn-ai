use dioxus::prelude::*;

#[component]
pub fn ProjectDetails(
    title: String,
    engine: String,
    file_roi: String,
    min_if: String,
    max_if: String,
    algorithm: String,
    block_width: String,
    block_height: String,
    step_xy: String,
    learning_mode: String,
    output_mode: String,
    categories: Vec<String>,
    nn_capacity: String,
    neurons: String,
    search_area: String,
    created: String,
    edited: String,
    timings: String,
) -> Element {
    rsx! {
        div {
            class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-2xl font-bold mb-6", "{title}" }

            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6",

                // Left Column - Learn + Recognize
                div {
                    class: "lg:col-span-2 space-y-6",

                    // Learn Block
                    div {
                        class: "border rounded-lg p-4 shadow bg-white",
                        h2 { class: "text-xl font-semibold mb-2", "Learn" }
                        img {
                            class: "w-full max-w-xs mx-auto mb-2",
                            src: "/path/to/learn-image.png",
                            alt: "Learn Image"
                        }
                        div {
                            class: "text-sm space-y-1",
                            p { "Search Area: {search_area}" }
                            p { "NN Capacity: {nn_capacity}" }
                            p { "Neurons: {neurons}" }
                            p { "Timings: {timings}" }
                            p { "Learning Mode: {learning_mode}" }
                            p { "Categories: {categories.join(\", \")}" }
                            p { "Step XY (px): {step_xy}" }
                        }
                    }

                    // Recognize Block
                    div {
                        class: "border rounded-lg p-4 shadow bg-white",
                        h2 { class: "text-xl font-semibold mb-2", "Recognize" }
                        img {
                            class: "w-full max-w-xs mx-auto mb-2",
                            src: "/path/to/recognize-image.png",
                            alt: "Recognize Image"
                        }
                        div {
                            class: "text-sm space-y-1",
                            p { "Output Mode: {output_mode}" }
                            p { "Step XY (px): {step_xy}" }
                        }
                    }
                },

                // Right Column - Settings
                div {
                    class: "border rounded-lg p-4 shadow bg-white space-y-4",

                    h2 { class: "text-xl font-semibold mb-2", "Settings" }

                    // Engine Settings
                    div {
                        class: "space-y-1 text-sm",
                        p { "Engine: {engine}" }
                        p { "Image File ROI: {file_roi}" }
                        p { "Algorithm: {algorithm}" }
                        p { "Block Size: {block_width} x {block_height}" }
                        p { "Influence Field Range: Min {min_if} - Max {max_if}" }
                    }

                    // Annotations Options
                    div {
                        class: "border-t pt-4 space-y-1 text-sm",
                        h3 { class: "font-semibold", "Options for Annotations" }
                        p { "Automatic Surrounding Examples: Counter Examples" }
                        p { "Positions: N, NE, E, SE, S, SW, W, NW" }
                        p { "Distance to Center: 10" }
                    }

                    // Metadata
                    div {
                        class: "border-t pt-4 text-xs text-gray-500",
                        p { "Created: {created}" }
                        p { "Last Edited: {edited}" }
                    }

                    // Control Buttons
                    div {
                        class: "flex justify-between mt-4",
                        button {
                            class: "bg-gray-200 px-3 py-1 rounded",
                            "Clear"
                        }
                        button {
                            class: "bg-blue-600 text-white px-3 py-1 rounded",
                            "Save"
                        }
                    }
                }
            }

            // Bottom Navigation
            div {
                class: "mt-6 flex justify-end gap-4",
                button {
                    class: "bg-gray-300 px-4 py-2 rounded",
                    "Back"
                }
                button {
                    class: "bg-black text-white px-4 py-2 rounded",
                    "Edit Project"
                }
            }
        }
    }
}

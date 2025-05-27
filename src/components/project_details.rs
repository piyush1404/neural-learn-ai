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
) -> Element {
    rsx!(
        div {
            class: "p-6 max-w-7xl mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "{title}" }
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                // Left Section (Learn + Recognize Visual Blocks)
                div {
                    class: "space-y-6",

                    // Learn Section
                    div {
                        class: "border rounded-lg p-4 shadow-md bg-white",
                        h2 { class: "text-lg font-semibold mb-2", "Learn" }
                        img {
                            class: "w-full max-w-sm mx-auto",
                            src: "/path/to/learn-image.png", // replace with actual
                            alt: "Learn Image"
                        }
                        p { "Learning Mode: {learning_mode}" }
                        p { "Categories: {categories.join(\", \")}" }
                        p { "Step XY: {step_xy}" }
                        p { "Search Area: {search_area}" }
                        p { "NN Capacity: {nn_capacity}" }
                        p { "Neurons: {neurons}" }
                    }

                    // Recognize Section
                    div {
                        class: "border rounded-lg p-4 shadow-md bg-white",
                        h2 { class: "text-lg font-semibold mb-2", "Recognize" }
                        img {
                            class: "w-full max-w-sm mx-auto",
                            src: "/path/to/recognize-image.png", // replace with actual
                            alt: "Recognize Image"
                        }
                        p { "Output Mode: {output_mode}" }
                        p { "Step XY: {step_xy}" }
                    }
                },

                // Right Section (Settings)
                div {
                    class: "border rounded-lg p-4 shadow-md bg-white space-y-4",
                    h2 { class: "text-lg font-semibold mb-2", "Settings" }

                    div {
                        class: "space-y-2",
                        p { "Engine: {engine}" }
                        p { "File ROI: {file_roi}" }
                        p { "Algorithm: {algorithm}" }
                        p { "Block Width: {block_width}" }
                        p { "Block Height: {block_height}" }
                        p { "Influence Field Range: Min {min_if}, Max {max_if}" }
                    }

                    div {
                        class: "border-t pt-4 space-y-2",
                        h3 { class: "font-semibold", "Options for Annotations" }
                        ul {
                            li { "Automatic Surrounding Examples: Counter Examples" }
                            li { "Positions: N, S, E, W, NE, NW, SE, SW" }
                            li { "Distance to Center: 10" }
                        }
                    }

                    div {
                        class: "border-t pt-4",
                        p { class: "text-sm text-gray-500", "Created: {created}" }
                        p { class: "text-sm text-gray-500", "Last Edited: {edited}" }
                    }
                }
            },

            div {
                class: "mt-6 flex justify-end gap-4",
                button {
                    class: "bg-gray-200 px-4 py-2 rounded",
                    "Back"
                }
                button {
                    class: "bg-black text-white px-4 py-2 rounded",
                    "Edit Project"
                }
            }
        }
    )
}

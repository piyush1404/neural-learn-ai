use dioxus::prelude::*;

#[component]
pub fn OptionForAnnotation(onclose: EventHandler<MouseEvent>) -> Element {
    let mut show_modal = use_signal(|| true);
    let mut surrounding_example = use_signal(|| "none".to_string());
    let mut positions = use_signal(|| vec![]);

    let mut distance = use_signal(|| 10);

    let mut toggle_position = {
        let mut positions = positions.clone();
        move |pos: &str| {
            let mut current = positions.read().clone();
            if current.contains(&pos.to_string()) {
                current.retain(|p| p != pos);
            } else {
                current.push(pos.to_string());
            }
            positions.set(current);
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50",
            onclick: move |_| show_modal.set(false),

            div {
                class: "w-[480px] h-[375px] bg-[#FAFAFA] rounded shadow p-6",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "flex justify-between text-black items-center mb-4",
                    h2 {
                        class: "text-[15px] font-medium text-[#404040] font-[Poppins]",
                        "Options for Annotations"
                    }
                    button {
                        class: "text-black text-xl",
                        onclick: move |e| onclose.call(e),
                        "×"
                    }
                }

                // Section Titles
                div {
                    class: "flex justify-between mb-1",
                    span { class: "text-[13px] w-[221px] text-black font-medium", "Automatic Surrounding Examples" },
                    span { class: "text-[13px] w-[202px] text-black font-medium", "Positions" },
                }

                // Main Body
                div {
                    class: "flex justify-between ",

                    // Left Column
                    div {
                        class: "w-[221px] h-[208px] bg-[#EFEFEF] text-black rounded-md p-4",
                        label {
                            class: "flex items-center mb-2 gap-2",
                            input {
                                r#type: "radio",
                                name: "surrounding",
                                checked: surrounding_example() == "none",
                                onchange: move |_| surrounding_example.set("none".to_string()),
                            }
                            span { "None" }
                        }
                        label {
                            class: "flex items-center gap-2",
                            input {
                                r#type: "radio",
                                name: "surrounding",
                                checked: surrounding_example() == "counter",
                                onchange: move |_| surrounding_example.set("counter".to_string()),
                            }
                            span { "Counter Examples" }
                        }
                    }

                    // Right Column - Hardcoded Checkboxes
                    div {
                        class: "px-3 w-[202px] h-[208px] bg-[#EFEFEF] grid grid-cols-3 gap-2 mb-3 text-sm text-[#404040]",
                        
                        // Row 1
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"NW".to_string()),
                                onclick: move |_| toggle_position("NW"),
                                class: "accent-[#0387D9] w-4 h-4"
                            }
                            span { "NW" }
                        }
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"N".to_string()),
                                onclick: move |_| toggle_position("N"),
                                class: "accent-[#0387D9] w-4 h-4"
                            }
                            span { "N" }
                        }
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"NE".to_string()),
                                onclick: move |_| toggle_position("NE"),
                                class: "accent-[#0387D9] w-4 h-4 !accent-[#0387D9]"
                            }
                            span { "NE" }
                        }
                    
                        // Row 2
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"W".to_string()),
                                onclick: move |_| toggle_position("W"),
                                class: "accent-[#0387D9] w-4 h-4"
                            }
                            span { "W" }
                        }
                        div {} // This is the center blank space
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"E".to_string()),
                                onclick: move |_| toggle_position("E"),
                                class: "accent-[#0387D9] w-4 h-4"
                            }
                            span { "E" }
                        }
                    
                        // Row 3
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"SW".to_string()),
                                onclick: move |_| toggle_position("SW"),
                                class: "accent-[#0387D9] w-4 h-4"
                            }
                            span { "SW" }
                        }
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"S".to_string()),
                                onclick: move |_| toggle_position("S"),
                                class: "accent-[#0387D9] w-4 h-4"
                            }
                            span { "S" }
                        }
                        label {
                            class: "flex items-center gap-1 text-xs text-[#404040]",
                            input {
                                r#type: "checkbox",
                                checked: positions.read().contains(&"SE".to_string()),
                                onclick: move |_| toggle_position("SE"),
                                class: "accent-[#0387D9] w-4 h-4"
                            }
                            span { "SE" }
                        }
                        div {
                            class: "flex items-center gap-2",
                            // Label
                            label {
                                class: "text-sm text-[#404040] font-medium whitespace-nowrap",
                                "Distance to center"
                            }
                
                            // Number input
                            input {
                                r#type: "number",
                                class: "w-[60px] px-2 py-1 border border-gray-300 rounded text-sm text-center",
                                value: "{distance}",
                                oninput: move |e| {
                                    if let Ok(val) = e.value().parse::<i32>() {
                                        distance.set(val);
                                    }
                                }
                            }
                        }
                    }
                    
                }

                // Footer
                div {
                    class: "flex py-3 border-t justify-end gap-3",
                    button {
                        class: "px-4 py-1 bg-[#E5E5E5] text-black text-sm rounded",
                        onclick: move |e| onclose.call(e),
                        "Cancel"
                    }
                    button {
                        class: "px-4 py-1 bg-black text-white text-sm rounded",
                        "Save"
                    }
                }
            }
        }
    }
}
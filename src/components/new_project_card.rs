use dioxus::prelude::*;

#[component]
pub fn NewProjectCard() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut show_advanced = use_signal(|| false);

    let mut categories = use_signal(|| vec![
        ("Background".to_string(), "bg-neutral-800".to_string()),
        ("Object".to_string(), "bg-red-600".to_string())
    ]);

    let default_categories = vec![
        ("Background".to_string(), "bg-neutral-800".to_string()),
        ("Object".to_string(), "bg-red-600".to_string()),
    ];

    let mut description = use_signal(|| "".to_string());

    rsx! {
        
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

        if *show_modal.read() {
            div {
                class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                div{
                div {
                    class: "bg-white w-[811px] rounded-[10px] overflow-hidden shadow-lg max-h-[580px] flex flex-col text-[13px]",

                    div {
                        class: "flex justify-between items-center px-6 py-4 border-b h-[42px]",
                        h2 { class: "text-sm font-normal text-[#404040]", "Project Details" }
                        button {
                            class: "text-gray-400 hover:text-black text-xl",
                            onclick: move |_| show_modal.set(false),
                            // "×"
                            svg {
                                width: "811",
                                height: "42",
                                view_box: "0 0 811 42",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                        
                                path {
                                    d: "M0 10C0 4.47715 4.47715 0 10 0H801C806.523 0 811 4.47715 811 10V42H0V10Z",
                                    fill: "white"
                                }
                                path {
                                    d: "M793.572 25.2862C793.389 25.2862 793.206 25.2165 793.066 25.0769L785.924 17.9341C785.645 17.655 785.645 17.203 785.924 16.9241C786.203 16.6452 786.655 16.645 786.934 16.9241L794.076 24.0669C794.356 24.346 794.356 24.7979 794.076 25.0769C793.937 25.2165 793.754 25.2862 793.572 25.2862Z",
                                    fill: "#555555"
                                }
                                path {
                                    d: "M786.429 25.2853C786.246 25.2853 786.063 25.2156 785.924 25.076C785.645 24.797 785.645 24.3449 785.924 24.066L793.066 16.9232C793.346 16.6441 793.798 16.6441 794.076 16.9232C794.355 17.2021 794.356 17.6542 794.076 17.9332L786.934 25.076C786.794 25.2156 786.611 25.2853 786.429 25.2853Z",
                                    fill: "#555555"
                                }
                            }
                        }
                    }

                  div {  
                    class:"bg-[#EFEFEF] h-[158px]",
                    div {
                        class: "px-6 pt-4 grid grid-cols-3 gap-4",
                        div {
                            label { class: "block mb-1 font-normal text-xs text-[#404040]", "Project Name" }
                            input {
                                class: "w-full border rounded px-3 py-1 h-[30px] outline-blue-500",
                                value: {"Sensor clip"},
                               
                            }
                        }
                        div {
                            label { class: "block mb-1 text-xs font-normal text-[#4D4D4D]", "Select Platform" }
                            select {
                                class: "w-full border rounded px-3 py-1 h-[30px] outline-blue-500",
                                option { "Simulation" }
                                option { "Real-Time" }
                            }
                        }
                        div {
                            label { class: "block mb-1 text-xs font-normal text-[#4D4D4D]", "Project Type" }
                            select {
                                class: "w-full border rounded px-3 py-1 h-[30px] outline-blue-500",
                                option { "Image" }
                                option { "Video" }
                            }
                        }
                    }

                    div {
                        class: "col-span-3 px-6 mt-2",
                        div {
                            class: "flex justify-between items-center bg-[#EFEFEF] mb-1 text-xs text-[#404040]",
                            span { "Description" }
                            span { "{description.read().len()}/100 alphabets" }
                        }
                        textarea {
                            class: "border rounded font-poppins font-normal w-[771px] p-2 text-xs h-[46px] resize-none bg-white appearance-none outline-none",
                            maxlength: "100",
                            value: "{description}",
                            oninput: move |e| description.set(e.value().clone())
                        }
                    }

                   
                  } 
                  div {
                        class: "text-center my-2",
                        button {
                            class: "bg-[#0D99FF] text-[#FFFFFF] font-normal text-xs rounded-full w-[84px] h-[26px] text-center",
                            onclick: move |_| {
                                let current = *show_advanced.read();
                                show_advanced.set(!current);
                            },
                            "Advance " { if *show_advanced.read() { "-" } else { "+" } }
                        }
                    }

                    if *show_advanced.read() {
                        div {
                            class: "flex px-6 gap-4 pb-4 border-t h-[266px]",
                    
                            // Categories (w-3/12)
                            div {
                                class: "pt-4 border-r w-[22%] flex flex-col justify-between h-[266px]",
                                div {
                                    class: "relative mb-2 space-y-2 h-[266px]",
                                
                                    // Header
                                    span { class: "block mb-2 text-xs text-[#404040] font-normal", "Categories" }
                                
                                    div {
                                        class: "flex items-center gap-2 text-xs mb-1",
                                        span {class:"w-[91px] text-[10px] h-[15px] rounded" ,"Name" }
                                        span {class:"w-[28px] text-[10px] h-[15px]", "Color" }
                                        span {} // For buttons column
                                    }
                                
                                    // Rows
                                    for (index, (name, color)) in categories.read().iter().cloned().enumerate() {
                                        div {
                                            class: "flex items-center gap-2",
                                
                                            // Name Input
                                            input {
                                                class: "border p-1 w-[91px] rounded text-sm h-[20px]",
                                                value: "{name}",
                                                oninput: move |e| {
                                                    let mut updated = categories.write().clone();
                                                    updated[index].0 = e.value().clone();
                                                    categories.set(updated);
                                                }
                                            }
                                
                                            // Color box
                                            div {
                                                class: "",
                                                div {
                                                    class: format_args!("w-[28px] h-[20px] rounded border border-gray-300 {}", color),
                                                }
                                            }
                                
                                            // Button
                                            div {
                                                class: "",
                    
                                                // Remove (×)
                                                if categories.read().len() > 1 && index != categories.read().len() - 1 {
                                                    button {
                                                        class: "w-[19px] h-[19px] rounded-sm border border-red-300 text-red-500 text-sm flex items-center justify-center",
                                                        onclick: move |_| {
                                                            let mut updated = categories.write().clone();
                                                            updated.remove(index);
                                                            categories.set(updated);
                                                        },
                                                        "×"
                                                    }
                                                }
                    
                                                // Add (+)
                                                if index == categories.read().len() - 1 {
                                                    button {
                                                        class: "w-[19px] h-[19px] rounded-sm border border-blue-300 text-blue-500 text-sm flex items-center justify-center",
                                                        onclick: move |_| {
                                                            let mut updated = categories.write().clone();
                                                            updated.push(("".to_string(), "bg-black".to_string()));
                                                            categories.set(updated);
                                                        },
                                                        "+"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                
                                    // Reset Button
                                    div {
                                        class: "absolute bottom-[20px] right-[10px]",
                                        button {
                                            class: "bg-blue-500 text-white px-4 py-1 rounded-full text-sm",
                                            onclick: move |_| {
                                                categories.set(vec![
                                                    ("Background".to_string(), "bg-zinc-800".to_string()),
                                                    ("Object".to_string(), "bg-red-500".to_string()),
                                                ]);
                                            },
                                            "Reset"
                                        }
                                    }
                                }
                            }
                    
                            // Context (w-2/12)
                            div {
                                class: "pt-4 border-r w-[22%]",
                                span { class: "block mb-2 text-xs font-normal", "Context" }
                                div {
                                    class: "space-y-1",
                                    span { class: "block mb-2 text-[10px] font-normal", "Context" }
                                    
                                    div {
                                        class: "flex items-center gap-1",
                                        button {
                                            class: "flex items-center justify-center w-5 h-5 text-sm border rounded text-blue-500",
                                            "+"
                                        }
                                        div {
                                            class: "px-2 py-1 text-xs text-gray-700 bg-white border rounded",
                                            "Enter Context"
                                        }
                                    }
                                    div {
                                        class: "flex",
                                        div {
                                            class: "flex flex-col items-center w-5 mr-1",
                                            div {
                                                class: "w-px h-3 bg-gray-300",
                                            }
                                            div {
                                                class: "w-full h-px bg-gray-300",
                                            }
                                            div {
                                                class: "w-px h-full grow",
                                            }
                                        }
                                        div {
                                            class: "flex items-center gap-1",
                                            button {
                                                class: "flex items-center justify-center w-5 h-5 text-sm border rounded text-blue-500",
                                                "+"
                                            }
                                            div {
                                                class: "px-2 py-1 text-xs text-gray-700 bg-white border rounded",
                                                "Sub Context"
                                            }
                                        }
                                    }
                                }
                            }
                    
                            // Feature Extraction (w-7/12)
                            div {
                                class: "pt-4 w-[56%]",
                                span { class: "block mb-2 text-xs font-normal", "Feature Extraction" }

                        // Algorithm & Normalize Row
                        div {
                            class: "mb-4",
                            label { class: "mb-1", "Algorithm" }
                            div {
                                class:"flex gap-4 items-center",   
                                // Algorithm Dropdown
                                div {
                                    class: "flex flex-col text-xs",
                                    select {
                                        class: "border rounded px-2 py-1 text-xs w-[140px]",
                                        option { "Subsample" }
                                        option { "Fullscan" }
                                    }
                                }

                                // Normalize Checkbox
                                div {
                                    class: "flex items-center mb-2 gap-2 mt-2",
                                    input { r#type: "checkbox", class: "w-4 h-4" }
                                    label { class: "text-xs", "Normalize" }
                                }
                            }
                        }
                        div { class:"flex items-center justify-end",
                            label{class:"text-xs", "Influence field range "} }
                                    // Input Fields Row (Width, Height, Block Width, Block Height, Max, Min)
                                    div {
                                        class: "grid grid-cols-6 gap-4 text-xs mb-4",

                                        // Width
                                        div {
                                            class: "flex flex-col w-[62px]",
                                            label { class: "mb-1 text-[10px]", "Width" }
                                            input {
                                                r#type: "number",
                                                value: "16",
                                                class: "border rounded px-2 py-1 text-xs",
                                            }
                                        }

                                        // Height
                                        div {
                                            class: "flex flex-col w-[62px]",
                                            label { class: "mb-1 text-[10px]", "Height" }
                                            input {
                                                r#type: "number",
                                                value: "16",
                                                class: "border rounded px-2 py-1 text-xs",
                                            }
                                        }

                                        // Block Width
                                        div {
                                            class: "flex flex-col w-[62px]",
                                            label { class: "mb-1 text-[10px]", "Block Width" }
                                            input {
                                                r#type: "number",
                                                value: "1",
                                                class: "border rounded px-2 py-1 text-xs",
                                            }
                                        }

                                        // Block Height
                                        div {
                                            class: "flex flex-col w-[62px]",
                                            label { class: "mb-1 text-[10px]", "Block Height" }
                                            input {
                                                r#type: "number",
                                                value: "1",
                                                class: "border rounded px-2 py-1 text-xs",
                                            }
                                        }

                                        // Max
                                        div {
                                            class: "flex flex-col w-[62px]",
                                            label { class: "mb-1 text-[10px]", "Max" }
                                            input {
                                                r#type: "number",
                                                value: "16878",
                                                class: "border rounded px-2 py-1 text-xs",
                                            }
                                        }

                                        // Min
                                        div {
                                            class: "flex flex-col w-[62px]",
                                            label { class: "mb-1 text-[10px]", "Min" }
                                            input {
                                                r#type: "number",
                                                value: "16",
                                                class: "border rounded px-2 py-1 text-xs",
                                            }
                                        }
                                    }

                                    // Buttons
                                    div {
                                        class: "flex justify-end gap-2",
                                        button {
                                            class: "bg-blue-500 hover:bg-blue-600 text-white text-xs font-medium py-1 px-4 rounded",
                                            "Suggest"
                                        }
                                        button {
                                            class: "bg-blue-500 hover:bg-blue-600 text-white text-xs font-medium py-1 px-4 rounded",
                                            "Validate"
                                        }
                                    }
                                }
                        
                            }
                        }
                        div {
                    class: "flex justify-between items-center px-6 py-4 bg-gray-50 border-t h-[36px]",
                        div {
                            class: "w-full flex justify-end items-center gap-3 bg-white",
                            button {
                                class: "text-xs rounded px-4 py-1 w[81px] bg-gray-200 text-gray-600",
                                onclick: move |_| show_modal.set(false),
                                "Cancel"
                            }
                            button {
                                class: "text-xs bg-black text-white rounded px-4 py-1 w[81px]",
                                "Start"
                            }
                        }
                    }
                }
                }
            }
        }
    }
}
        
    


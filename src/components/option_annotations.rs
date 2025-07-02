use dioxus::{html::select, prelude::*};

#[component]
pub fn OptionForAnnotation(show_modal_options: Signal<bool>,selected_annotation: Signal<String>) -> Element {

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50",
            div {
                class: "w-[480px] bg-[#FFFFFF] rounded-[10px] shadow-lg",
    
                // Title Bar
                div {
                    class: "h-[42px] flex justify-between items-center px-5 py-3",
                    h2 {
                        class: "text-sm font-normal text-[#404040]",
                        "Options for Annotations"
                    }
                    button {
                        class: "text-[#555555] hover:text-gray-700 text-xl font-bold",
                        onclick: move |_| { show_modal_options.set(false); selected_annotation.set("".to_string()); },
                        "×"
                    }
                }
    
                // Main content area
                div {
                    class: "bg-[#FAFAFA] flex gap-6",
    
                    // Left Section - Automatic Surrounding Examples
                    div {
                        class: "w-1/2 p-5 border-gray-200",
                        h3 {
                            class: "text-xs font-normal text-[#404040] mb-2",
                            "Automatic Surrounding Examples"
                        }
    
                        div {
                            class: "bg-[#EFEFEF] h-[213px] p-5 rounded-md space-y-3 text-sm text-gray-800",
                            // Radio 1
                            label {
                                class: "flex items-center space-x-2",
                                input {
                                    r#type: "radio",
                                    name: "surrounding_examples",
                                    checked: true,
                                    class: "w-5 h-5 accent-[#0387D9]"
                                }
                                span { class: "font-normal text-[11px] text-[#404040]", "None" }
                            }
    
                            // Radio 2
                            label {
                                class: "bg-[#EFEFEF] flex items-center space-x-2",
                                input {
                                    r#type: "radio",
                                    name: "surrounding_examples",
                                   class: "w-5 h-5 accent-[#0387D9]"
                                }
                                span { class: "font-normal text-[11px] text-[#404040]", "Counter Examples" }
                            }
                        }
                    }
    
                    // Right Section - Positions
                    div {
                        class: "w-1/2 p-5 border-gray-200",
    
                        h3 {
                            class: "text-xs font-normal text-[#404040] mb-2",
                            "Positions"
                        }

                        div {  class:"bg-[#EFEFEF] h-[213px] rounded-md p-5 space-y-3",
                            // 3x3 Grid for positions
                            div {
                                class: "grid grid-cols-3 gap-2 mb-4",
                                for (label, checked) in [
                                    ("NW", false), ("N", false), ("NE", false),
                                    ("W", false), ("", false), ("E", false),
                                    ("SW", false), ("S", false), ("SE", false),
                                ] {
                                    if label != "" {
                                        label {
                                            class: format!(
                                                "flex items-center justify-start font-normal text-[10px] text-[#404040] {}",
                                                if checked { "border-blue-500 bg-blue-50" } else { "border-gray-300" }
                                            ),
                                            input {
                                                r#type: "checkbox",
                                                checked: checked,
                                                class: "w-5 h-5 accent-[#0387D9] mr-1"
                                            }
                                            "{label}"
                                        }
                                    } else {
                                        div {} // Center spacer
                                    }
                                }
                            }
                            
                            
    
                        // Distance to center
                        div {
                            class: "flex items-center justify-between font-normal text-[10px] text-[#404040]",
                            span { "Distance to center" }
                            input {  
                                r#type: "number",
                                class: "w-12 h-[26px] border-[0.5px] border-[#8F8F8F] rounded px-2 py-1 font-normal text-xs text-[#313131]",
                                value: "10",
                                min: "0",
                                max: "100",
                                step: "1"
                            }
                        }

                        }
    
                        
                    }
                }
    
                hr { class: "border border-[#DDDDDD] w-full" }

                // Footer Buttons
                div {
                    class: "bg-[#FAFAFA] flex justify-end px-5 py-3 space-x-2",
                    button {
                        onclick: move |_| { 
                            show_modal_options.set(false);
                            selected_annotation.set("".to_string());
                        },
                        class: "px-4 py-1 bg-[#1010101A] border rounded-[3px] text-[#101010] text-xs font-normal",
                        "Cancel"
                    }
                    button {
                        onclick: move |_| { 
                            show_modal_options.set(false);
                            selected_annotation.set("".to_string());
                        },
                        class: "px-4 py-1 bg-[#101010] rounded-[3px] text-[#FFFFFF] text-xs font-normal",
                        "Save"
                    }
                }
            }
        }
    }
}
use dioxus::prelude::*;
use crate::state::tabs::TabContext;

#[component]
pub fn NewProjectCard() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut show_advanced = use_signal(|| false);
    let mut tab_context = use_context::<Signal<TabContext>>();


    let mut categories = use_signal(|| vec![
        ("Background".to_string(), "bg-neutral-800".to_string()),
        ("Object".to_string(), "bg-red-600".to_string())
    ]);

    let default_categories = vec![
        ("Background".to_string(), "bg-neutral-800".to_string()),
        ("Object".to_string(), "bg-red-600".to_string()),
    ];

    let mut description = use_signal(|| "".to_string());
    let mut open_image_roi = {
        move |id| {
            println!("Opening image ROI...");
            tab_context.write().add_tab(
                "Sensor Chip",
                // rsx! { crate::views::image_roi::ImageRoi {} },
                rsx! { crate::components::project_details::ProjectDetails {} },
                Some(rsx! {
                        svg {
                            width: "18",
                            height: "18",
                            view_box: "0 0 18 18",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",

                            path {
                                d: "M4.34067 10.9531C4.04918 10.9531 3.81287 10.7168 3.81287 10.4253C3.81287 10.1338 4.04918 9.89752 4.34067 9.89752C5.02786 9.89752 5.58694 9.35072 5.58694 8.67859C5.58694 8.00645 5.02786 7.45965 4.34067 7.45965C4.04918 7.45965 3.81287 7.22334 3.81287 6.93185C3.81287 6.64036 4.04918 6.40405 4.34067 6.40405C5.60992 6.40405 6.64254 7.42443 6.64254 8.67859C6.64254 9.93275 5.60992 10.9531 4.34067 10.9531Z",
                                fill: "#FF7700"
                            }

                            path {
                                d: "M18 8.39686C18 9.60162 17.2254 10.6281 16.1311 11.0069C16.2644 11.3336 16.3346 11.6849 16.3346 12.0406C16.3346 13.3836 15.3596 14.5062 14.0712 14.7557C13.9759 16.1965 12.7586 17.3398 11.2759 17.3398C10.3391 17.3398 9.50889 16.883 9.00002 16.1836C8.49115 16.8831 7.66088 17.3398 6.72414 17.3398C5.24144 17.3398 4.02409 16.1965 3.92884 14.7557C2.64041 14.5062 1.66535 13.3836 1.66535 12.0406C1.66535 11.6849 1.73562 11.3336 1.86887 11.0069C0.7746 10.6282 0 9.60162 0 8.39686C0 7.26269 0.696943 6.27183 1.70972 5.84889C1.55965 5.50508 1.4803 5.13186 1.4803 4.75307C1.4803 4.0126 1.78154 3.31481 2.32848 2.78831C2.76353 2.36956 3.30857 2.10256 3.89513 2.01505C4.05118 1.50872 4.34936 1.05164 4.75904 0.699104C5.28275 0.248468 5.95412 0.000297546 6.64948 0.000297546C7.6194 0.000297546 8.47816 0.476902 8.99995 1.20488C9.52173 0.476902 10.3805 0.000297546 11.3504 0.000297546C12.0457 0.000297546 12.7171 0.248468 13.2408 0.699104C13.6505 1.05164 13.9487 1.50875 14.1048 2.01505C14.6913 2.10256 15.2364 2.36956 15.6714 2.78831C16.2184 3.31481 16.5196 4.0126 16.5196 4.75307C16.5196 5.13189 16.4402 5.50508 16.2902 5.84889C17.3031 6.27187 18 7.26273 18 8.39686ZM6.64951 1.0559C5.74796 1.0559 4.97223 1.71417 4.845 2.58712C4.845 2.58719 4.845 2.58726 4.84496 2.58729C4.83275 2.67111 4.82656 2.75703 4.82656 2.84264C4.82656 3.82784 5.64433 4.62935 6.64951 4.62935C6.941 4.62935 7.17731 4.86567 7.17731 5.15715C7.17731 5.44864 6.941 5.68495 6.64951 5.68495C5.15774 5.68495 3.92761 4.55859 3.78503 3.12196C3.06962 3.3468 2.5359 4.00278 2.5359 4.75307C2.5359 5.15318 2.67243 5.52788 2.93077 5.83665C3.05505 5.98524 3.08798 6.18999 3.01652 6.37004C2.94506 6.55009 2.78066 6.67652 2.5883 6.69942C1.71451 6.80336 1.0556 7.5331 1.0556 8.39686C1.0556 9.31439 1.78777 10.0649 2.72239 10.1055C2.92229 10.1142 3.10005 10.2351 3.18155 10.4178C3.263 10.6006 3.23418 10.8137 3.10706 10.9682C2.85445 11.2752 2.72092 11.646 2.72092 12.0406C2.72092 12.8481 3.29576 13.5263 4.06603 13.7052C4.43162 12.6375 5.45848 11.8826 6.63684 11.8826C6.92833 11.8826 7.16464 12.1189 7.16464 12.4104C7.16464 12.7019 6.92833 12.9382 6.63684 12.9382C5.807 12.9382 5.10042 13.5449 4.9932 14.3493C4.98339 14.423 4.97839 14.4985 4.97839 14.5736C4.97839 15.5169 5.7615 16.2842 6.72407 16.2842C7.70979 16.2854 8.49291 15.4836 8.47243 14.521V2.85419C8.47236 2.85028 8.47183 2.84651 8.47183 2.84261C8.47183 2.83881 8.47211 2.83508 8.47211 2.83128C8.46592 1.85129 7.65082 1.0559 6.64951 1.0559ZM15.4117 6.69942C15.2193 6.67655 15.0549 6.55009 14.9834 6.37004C14.912 6.18999 14.9449 5.98524 15.0692 5.83665C15.3275 5.52788 15.4641 5.15318 15.4641 4.75307C15.4641 4.00278 14.9303 3.34676 14.215 3.12196C14.0724 4.55859 12.8422 5.68495 11.3505 5.68495C11.059 5.68495 10.8227 5.44864 10.8227 5.15715C10.8227 4.86567 11.059 4.62935 11.3505 4.62935C12.3556 4.62935 13.1734 3.82784 13.1734 2.84264C13.1734 2.7571 13.1672 2.67121 13.155 2.58729C13.155 2.58722 13.155 2.58715 13.155 2.58708C13.0277 1.71417 12.252 1.0559 11.3505 1.0559C10.3491 1.0559 9.53408 1.85133 9.52782 2.83131C9.52782 2.83511 9.5281 2.83884 9.5281 2.84264V14.5331C9.51209 15.4913 10.2956 16.2852 11.2759 16.2843C12.2385 16.2843 13.0215 15.5169 13.0215 14.5736C13.0215 14.4986 13.0165 14.4231 13.0067 14.3494C12.8995 13.5449 12.1929 12.9382 11.3631 12.9382C11.0716 12.9382 10.8353 12.7019 10.8353 12.4104C10.8353 12.1189 11.0716 11.8826 11.3631 11.8826C12.5415 11.8826 13.5683 12.6375 13.9339 13.7052C14.7041 13.5263 15.279 12.8482 15.279 12.0407C15.279 11.646 15.1455 11.2752 14.8929 10.9682C14.7658 10.8137 14.737 10.6006 14.8184 10.4179C14.8999 10.2352 15.0777 10.1142 15.2776 10.1055C16.2122 10.065 16.9444 9.31443 16.9444 8.3969C16.9444 7.5331 16.2855 6.80336 15.4117 6.69942Z",
                                fill: "#FF7700"
                            }

                            path {
                                d: "M13.6593 9.89752C13.9508 9.89752 14.1871 10.1338 14.1871 10.4253C14.1871 10.7168 13.9508 10.9531 13.6593 10.9531C12.39 10.9531 11.3574 9.93275 11.3574 8.67859C11.3574 7.42443 12.39 6.40405 13.6593 6.40405C13.9508 6.40405 14.1871 6.64036 14.1871 6.93185C14.1871 7.22334 13.9508 7.45965 13.6593 7.45965C12.9721 7.45965 12.413 8.00645 12.413 8.67859C12.413 9.35072 12.9721 9.89752 13.6593 9.89752Z",
                                fill: "#FF7700"
                            }
                        }
                    }
                )
            );
        }
    };
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
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    open_image_roi('1');
                                },
                                "Start",
                                
                            }
                        }
                    }
                }
                }
            }
        }
    }
}
        
    


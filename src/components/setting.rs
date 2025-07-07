use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::views::project_details::{AppState, SharedData};
use crate::store::project::ProjectState;
use crate::store::project::get_project_by_id;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    // pub id: String,
    // pub name: String,
    // pub platform: String,
    // pub interface: String,
    // pub r#type: String,
    // pub description: String,
    // pub created_at: Option<String>,
    // pub updated_at: Option<String>,
    pub feature_extraction: Option<FeatureExtraction>
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeatureExtraction {
    pub algorithm: String,
    pub normalized: bool,
    pub roi_width: u32,
    pub roi_height: u32,
    pub block_width: u32,
    pub block_height: u32,
    pub if_field_range: IfFieldRange,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IfFieldRange {
    pub min: u8,
    pub max: u8,
}

#[component]
pub fn Setting(app_state: AppState) -> Element {
println!("Setting component initialized");
    // 1️⃣ grab contexts
    let project_state = use_context::<ProjectState>();
    println!("ProjectState --------------------------->");

    println!("AppState --------------------------->");
    // 2️⃣ load + clone the project
    let project_id = (project_state.project_id)();
    println!("Project ID: {}", project_id);
        let project = get_project_by_id(&project_id.to_string()).unwrap();
        let project = project.clone();
        println!("project: {:?}", project);

    // 3️⃣ extract or early‑return
    let initial_feature = if let Some(f) = project.feature_extraction.clone() {
        f
    } else {
        return rsx!( div { "❌ No feature extraction data found." } );
    };

    // 4️⃣ put it in a signal
    let mut feature = use_signal(|| initial_feature);

    // 5️⃣ prepare shared setter
    let mut shared        = app_state.shared.clone();
    let shared_props  = shared();
    let clear_clicked = shared_props.clear_clicked.clone();

    // 6️⃣ keep shared context in sync when algorithm changes
    use_effect(move || {
        let algo = feature().algorithm.clone();
        if !algo.is_empty() {
            shared.set(SharedData {
                algorithm:    algo,
                clear_clicked: clear_clicked.clone(),
            });
        }
    });

    // let mut color_mode = use_signal(|| "grayscale".to_string());
    rsx! {
        // div {
        //     class:"flex gap-2 bg-[#CAEAD7] mx-[13px] my-[14px] px-[30px] py-[10px] rounded-[10px] ",
        //     span {
        //         svg {
        //             width: "14",
        //             height: "14",
        //             view_box: "0 0 14 14",
        //             fill: "none",
        //             xmlns: "http://www.w3.org/2000/svg",

        //             // First path (outer circle)
        //             path {
        //                 d: "M7 0C3.13111 0 0 3.13075 0 7C0 10.8688 3.13075 14 7 14C10.8689 14 14 10.8693 14 7C14 3.13116 10.8693 0 7 0ZM7 13.0233C3.67877 13.0233 0.976746 10.3213 0.976746 7C0.976746 3.67875 3.67877 0.976746 7 0.976746C10.3212 0.976746 13.0233 3.67875 13.0233 7C13.0233 10.3213 10.3212 13.0233 7 13.0233Z",
        //                 fill: "#308051"
        //             }

        //             // Second path (vertical line)
        //             path {
        //                 d: "M7.0005 5.83496C6.58586 5.83496 6.29102 6.01007 6.29102 6.26806V9.77861C6.29102 9.99977 6.58586 10.2209 7.0005 10.2209C7.39672 10.2209 7.71918 9.99977 7.71918 9.77861V6.268C7.71918 6.01004 7.39672 5.83496 7.0005 5.83496Z",
        //                 fill: "#308051"
        //             }

        //             // Third path (dot)
        //             path {
        //                 d: "M6.99968 3.66992C6.57582 3.66992 6.24414 3.97398 6.24414 4.32412C6.24414 4.67429 6.57585 4.98756 6.99968 4.98756C7.41432 4.98756 7.74605 4.67429 7.74605 4.32412C7.74605 3.97398 7.41429 3.66992 6.99968 3.66992Z",
        //                 fill: "#308051"
        //             }
        //         }
        //     }
        //     span {
        //         class:"font-normal text-sm text-[#308051]",
        //         "Recognize process is successfully completed. Please select another image."
        //     }
        // }

        div {
            class:"flex flex-col mx-[13px] my-[14px] gap-2 items-center cursor-pointer",
            div {
                class:"w-full flex justify-between items-center gap-2 text-xs font-normal",
                span {
                    class:"text-[#555555]",
                    "Search area :"
                }
                span {
                    class:"text-[#151515]",
                    "[0,0,0,0]"
                }
            }
            div {
                class:"w-full flex justify-between items-center gap-2 text-xs font-normal",
                span {
                    class:"text-[#555555]",
                    "NN Capacity: :"
                }
                span {
                    class:"text-[#151515]",
                    "16878"
                }
            }
            div {
                class:"w-full flex justify-between items-center gap-2 text-xs font-normal",
                span {
                    class:"text-[#555555]",
                    "Neurons :"
                }
                span {
                    class:"text-[#151515]",
                    "3"
                }
            }
            div {
                class:"w-full flex justify-between items-center gap-2 text-xs font-normal",
                span {
                    class:"text-[#555555]",
                    "Timings :"
                }
                span {
                    class:"text-[#151515]",
                    "Off"
                }
            }
            div {
                class:"w-full flex justify-between items-center gap-2 text-xs font-normal",
                span {
                    class:"text-[#555555]",
                    "Learned Images :"
                }
                span {
                    class:"text-[#151515]",
                    "2/2"
                }
            }
        }

        hr { class: "border-t-[0.5px] border-[#BEBEBE]" }

        div {
            class:"px-[13px] py-[14px] flex flex-col gap-2 items-start",

            span {
                class:"font-normal text-sm text-[#404040]",
                "Settings"
            }

            // span {
            //     class:"flex flex-col items-start gap-2",
            //     label {
            //         "class": "font-normal text-[10px] text-[#555555]",
            //         "Platform"
            //     }
            //     select {
            //         class: " px-[10px] py-1 border-[0.5px] border-[#DEDEDE] rounded-md font-normal text-xs text-[#313131] appearance-none pr-8",
            //         style: r#"
            //         color: #555555;
            //         background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
            //         background-repeat: no-repeat;
            //         background-position: right 0.75rem center;
            //         background-size: 10px 6px;
            //     "#,
            //         option { value: "", disabled: true, selected: true, }
            //         option { value: "web", "Web" }
            //         option { value: "mobile", "Mobile" }
            //         option { value: "desktop", "Desktop" }
            //     }
            // }
            span {  
               class: "flex items-center justify-between w-full gap-2",
                span {
                    class: "flex flex-col items-start gap-1",
                    label {
                        class: "font-normal text-[10px] text-[#555555]",
                        "Image file ROI"
                    }
                } 
            }
            span {
                class: "flex items-center justify-between w-full gap-2",
                span {
                    class: "flex flex-col items-start gap-1",
                    label {
                        class: "font-normal text-[10px] text-[#555555]",
                        "Width"
                    }
                    input {
                        r#type: "number",
                        class: "w-1/2 px-[10px] py-1 border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131]",
                        value: "{feature().roi_width}"
                    }
                }
                span {
                    class: "flex flex-col gap-1",
                    label {
                        class: "font-normal text-[10px] text-[#555555]",
                        "Height"
                    }
                    input {
                        r#type: "number",
                        class: "w-1/2 px-[10px] py-1 border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131]",
                        value: "{feature().roi_height}"
                    }
                }
            }
            
            span {
                class:"flex flex-col items-start gap-2 mt-2",
                label {
                    "class": "font-normal text-[10px] text-[#555555]",
                    "Algorithm "
                }
                select {
                // ensure the dropdown shows the current value
                value: "{feature().algorithm}",
                class: "min-w-full px-[10px] py-1 border-[0.5px] border-[#DEDEDE] rounded-md font-normal text-xs text-[#313131] appearance-none pr-8",
                style: r#"
                    color: #555555;
                    background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                    background-repeat: no-repeat;
                    background-position: right 0.75rem center;
                    background-size: 10px 6px;
                "#,

                onchange: move |evt| {
                     let new_algo = evt.value().to_string();

                    // update just the algorithm field
                    feature.with_mut(|f| {
                        f.algorithm = new_algo.clone();
                    });

                    // keep shared context in sync
                    shared.set(SharedData {
                        algorithm:    new_algo,
                        clear_clicked: clear_clicked.clone(),
                    });
                },

                option { value: "grayscale", style:"font-weight: 400; font-size: 10px; color: #555555;", "Subsample" }
                option { value: "rgb",                     "Subsample RGB" }
                option { value: "Histogram",               "Histogram" }
                option { value: "Histogram Cumulative",    "Histogram Cumulative" }
                option { value: "Histogram RGB",           "Histogram RGB" }
                option { value: "Histogram Cumulative RGB","Histogram Cumulative RGB" }
                option { value: "Composite Profile",       "Composite Profile" }
                option { value: "Horizontal Profile",      "Horizontal Profile" }
                option { value: "Vertical Profile",        "Vertical Profile" }
                }
            }

            span {
                class: "flex items-center justify-between w-full gap-2 mt-2",
                span {
                    class: "flex flex-col items-start gap-1",
                    label {
                        class: "font-normal text-[10px] text-[#555555]",
                        "Block width"
                    }
                    input {
                        r#type: "number",
                        class: "w-1/2 px-[10px] py-1 border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131]",
                        value: "{feature().block_width}"
                    }
                }
                span {
                    class: "flex flex-col gap-1",
                    label {
                        class: "font-normal text-[10px] text-[#555555]",
                        "Block height"
                    }
                    input {
                        r#type: "number",
                        class: "w-1/2 px-[10px] py-1 border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131]",
                        value: "{feature().block_height}"
                    }
                }
            }
            
            span {
                "class": "font-normal text-[10px] text-[#555555] mt-2",
                "Influence Field Range "
            }

            span {
                class: "flex items-center justify-between w-full gap-2",
                span {
                    class: "flex flex-col gap-1",
                    label {
                        class: "font-normal text-[10px] text-[#555555]",
                        "Min"
                    }
                    input {
                        r#type: "number",
                        class: "w-1/2 px-[10px] py-1 border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131]",
                        value: "{feature().if_field_range.min}"
                    }
                }
                span {
                    class: "flex flex-col items-start gap-1",
                    label {
                        class: "font-normal text-[10px] text-[#555555]",
                        "Max"
                    }
                    input {
                        r#type: "number",
                        class: "w-1/2 px-[10px] py-1 border bg-[#EAEAEA] rounded font-normal text-xs text-[#313131]",
                        value: "{feature().if_field_range.max}"
                    }
                }
            }
            
            span {
                class:"w-full mt-2 flex flex justify-between items-center gap-2",
                button {
                    class: "px-4 py-1 rounded-[50px] border-[0.5px] border-[#DEDEDE] bg-[#FFFFFF] font-normal text-[11px] text-[#151515]",
                    "Suggest"
                }

                button {
                    class: "px-4 py-1 rounded-[50px] border-[0.5px] border-[#DEDEDE] bg-[#FFFFFF] font-normal text-[11px] text-[#151515]",
                    "Validate"
                }
            }
            // div {
            //     h3 { "Feature Extraction Details" }
            //     p { "Algorithm: {feature.algorithm}" }
            //     p { "Normalized: {feature.normalized}" }
            //     p { "ROI Width: {feature.roi_width}" }
            //     p { "ROI Height: {feature.roi_height}" }
            //     p { "Block Width: {feature.block_width}" }
            //     p { "Block Height: {feature.block_height}" }
            //     p { "Range Min: {feature.if_field_range.min}" }
            //     p { "Range Max: {feature.if_field_range.max}" }
            // }
        }
    }
}

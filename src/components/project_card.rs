use crate::{components::project_form::ProjectForm, store::project_schema::NeuronConfig};
use dioxus::prelude::*;
<<<<<<< HEAD

const PLATFORM: Asset = asset!("/assets/icons/platform.svg");
const SIMULATION: Asset = asset!("/assets/icons/simulation.svg");
// const NUEROSHIELD: Asset = asset!("/assets/icons/nueroshield.svg");
const BRILLANT: Asset = asset!("/assets/icons/brillant.svg");
const COMPLETE_BRAIN: Asset = asset!("/assets/icons/complete_brain.svg");
const INCOMPLETE_BRAIN: Asset = asset!("/assets/icons/incomplete_brain.svg");

=======
>>>>>>> main
use crate::date_format::format_date_mmddyyyy;
use crate::store::project::get_project_by_id;
use crate::components::delete_alert_modal::DeleteModal;
use crate::components::delete_alert_modal::ToastType;
use std::time::Duration;
use dioxus::prelude::spawn;
use tokio::time::sleep;

#[component]
pub fn ProjectCard(
    id: String,
    name: String,
    platform: String,
    interface: String,
    description: String,
    created_at: String,
    updated_at: String,
    neurons: Option<NeuronConfig>,
    is_updating:Option<Signal<bool>>
) -> Element {
    let mut delete_modal_visible = use_signal(|| false);

    let project = get_project_by_id(&id).unwrap();
    let project = project.clone();
    // println!("Project: {:#?}", project);
    let toast: Signal<Option<ToastType>> = use_signal(|| None);
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

    let created_at = format_date_mmddyyyy(created_at.as_str());
    let updated_at = format_date_mmddyyyy(updated_at.as_str());

    let mut hovered = use_signal(|| false);

    let mut show_edit_modal = use_signal(|| false);

    //let mut modal = use_delete_modal();

    let toast_clone = toast.clone();
    use_effect(move || {
        if toast_clone().is_some() {
            let mut toast_clone = toast_clone.clone();
            spawn(async move {
                sleep(Duration::from_millis(2500)).await;
                toast_clone.set(None);
            });
        }
    });

    
    let icon_svg = match interface.as_str() {
        "Image" => rsx! {
            svg {
                width: "15",
                height: "15",
                view_box: "0 0 15 15",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
    
                g {
                    clip_path: "url(#clip0_366_3577)",
    
                    path {
                        d: "M13.0833 0H1.91675C0.859834 0 0 0.859863 0 1.91678V13.0833C0 14.1401 0.859834 15 1.91675 15H13.0833C14.1402 15 15 14.1401 15 13.0832V1.91678C15 0.859863 14.1402 0 13.0833 0ZM14.1165 13.0833C14.1165 13.653 13.653 14.1165 13.0833 14.1165H1.91675C1.34701 14.1165 0.883506 13.653 0.883506 13.0833V11.4717L3.79137 8.99757C3.89748 8.90728 4.05229 8.90643 4.15939 8.99537L5.9809 10.5079C6.15653 10.6537 6.41435 10.6418 6.57574 10.4802L10.9037 6.14561C10.982 6.06724 11.0732 6.05962 11.1208 6.06205C11.1682 6.06448 11.2583 6.08142 11.3281 6.1674L14.1165 9.6007L14.1165 13.0833ZM14.1165 8.19917L12.0139 5.61032C11.8054 5.35356 11.4964 5.19659 11.166 5.17963C10.8359 5.16293 10.5122 5.28721 10.2785 5.52129L6.23531 9.57073L4.72389 8.31568C4.28563 7.95176 3.65271 7.95554 3.21882 8.32471L0.883506 10.3117V1.91678C0.883506 1.34704 1.34701 0.883535 1.91675 0.883535H13.0833C13.653 0.883535 14.1165 1.34704 14.1165 1.91678V8.19917Z",
                        fill: "#0387D9"
                    }
    
                    path {
                        d: "M4.72182 1.8457C3.54716 1.8457 2.59155 2.80137 2.59155 3.97597C2.59155 5.1506 3.54719 6.10623 4.72182 6.10623C5.89645 6.10623 6.85208 5.1506 6.85208 3.97597C6.85208 2.80134 5.89648 1.8457 4.72182 1.8457ZM4.72182 5.22272C4.03434 5.22272 3.47506 4.66342 3.47506 3.97597C3.47506 3.28849 4.03434 2.72921 4.72182 2.72921C5.4093 2.72921 5.96857 3.28852 5.96857 3.97597C5.96857 4.66342 5.4093 5.22272 4.72182 5.22272Z",
                        fill: "#0387D9"
                    }
                }
    
                defs {
                    clipPath {
                        id: "clip0_366_3577",
                        rect {
                            width: "15",
                            height: "15",
                            fill: "white"
                        }
                    }
                }
            }
        },
        "Video" => rsx! {
            svg {
                width: "19",
                height: "13",
                view_box: "0 0 19 13",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M17.75 1.67385C17.5381 1.57067 17.3011 1.53006 17.0668 1.55678C16.8326 1.58349 16.6108 1.67643 16.4275 1.82469L14.2917 3.53302V3.16719C14.3406 2.74497 14.2935 2.31715 14.1537 1.91574C14.014 1.51432 13.7853 1.1497 13.4847 0.849153C13.1842 0.548602 12.8196 0.319901 12.4181 0.180159C12.0167 0.0404158 11.5889 -0.00675141 11.1667 0.0421853H3.6667C3.24448 -0.00675141 2.81666 0.0404158 2.41525 0.180159C2.01384 0.319901 1.64922 0.548602 1.34866 0.849153C1.04811 1.1497 0.819413 1.51432 0.67967 1.91574C0.539928 2.31715 0.49276 2.74497 0.541697 3.16719V9.83385C0.49276 10.2561 0.539928 10.6839 0.67967 11.0853C0.819413 11.4867 1.04811 11.8513 1.34866 12.1519C1.64922 12.4524 2.01384 12.6811 2.41525 12.8209C2.81666 12.9606 3.24448 13.0078 3.6667 12.9589H11.1667C11.5889 13.0078 12.0167 12.9606 12.4181 12.8209C12.8196 12.6811 13.1842 12.4524 13.4847 12.1519C13.7853 11.8513 14.014 11.4867 14.1537 11.0853C14.2935 10.6839 14.3406 10.2561 14.2917 9.83385V9.46802L16.4275 11.1764C16.6479 11.3543 16.9226 11.4514 17.2059 11.4514C17.3942 11.4511 17.5802 11.4087 17.75 11.3272C17.9629 11.2258 18.1426 11.066 18.268 10.8663C18.3934 10.6667 18.4594 10.4355 18.4584 10.1997V2.80135C18.4594 2.56557 18.3934 2.33436 18.268 2.13471C18.1426 1.93506 17.9629 1.77521 17.75 1.67385ZM13.0417 9.83385C13.0417 11.148 12.4809 11.7089 11.1667 11.7089H3.6667C2.35253 11.7089 1.7917 11.148 1.7917 9.83385V3.16719C1.7917 1.85302 2.35253 1.29219 3.6667 1.29219H11.1667C12.4809 1.29219 13.0417 1.85302 13.0417 3.16719V9.83385ZM17.2084 10.2005L14.2917 7.86719V5.13385L17.2084 2.80052V10.2005Z",
                    fill: "#0387D9"
                }
            }
        },
        "Audio" => rsx! {
            svg {
                width: "19",
                height: "19",
                view_box: "0 0 19 19",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
    
                g {
                    clip_path: "url(#clip0_366_3588)",
    
                    path {
                        d: "M10.9604 17.8409C10.7516 17.9405 10.5056 17.9141 10.3256 17.7689L4.4888 13.0997H1.7C1.0388 13.0997 0.5 12.5609 0.5 11.8997V7.09969C0.5 6.43849 1.0388 5.89969 1.7 5.89969H4.4888L10.3244 1.23049C10.4336 1.14409 10.5668 1.09969 10.7 1.09969C10.7888 1.09969 10.8776 1.11889 10.9604 1.15849C11.168 1.25929 11.3 1.46929 11.3 1.69969V17.2997C11.3 17.5301 11.168 17.7401 10.9604 17.8409ZM4.1 7.09969H1.7V11.8997H4.1V7.09969ZM10.1 2.94889L5.3 6.78889V12.2105L10.1 16.0505V2.94889Z",
                        fill: "#0387D9"
                    }
    
                    path {
                        d: "M14.3383 13.7425C14.1019 13.9765 13.7227 13.9729 13.4899 13.7377C13.2571 13.5013 13.2595 13.1221 13.4947 12.8893C14.4007 11.9929 14.8999 10.7893 14.8999 9.49925C14.8999 8.20925 14.4007 7.00565 13.4947 6.10925C13.2583 5.87645 13.2559 5.49725 13.4899 5.26085C13.6075 5.14205 13.7623 5.08325 13.9159 5.08325C14.0683 5.08325 14.2207 5.14085 14.3383 5.25725C15.4747 6.37925 16.0999 7.88525 16.0999 9.49925C16.0999 11.1133 15.4747 12.6193 14.3383 13.7425Z",
                        fill: "#0387D9"
                    }
    
                    path {
                        d: "M16.0304 15.4337C15.7952 15.6665 15.4148 15.6653 15.182 15.4301C14.9492 15.1949 14.9504 14.8145 15.1856 14.5817C16.5488 13.2293 17.3 11.4245 17.3 9.49972C17.3 7.57492 16.5488 5.77012 15.1856 4.41772C14.9504 4.18492 14.9492 3.80452 15.182 3.56932C15.2996 3.45052 15.4532 3.39172 15.608 3.39172C15.7604 3.39172 15.9128 3.44932 16.0304 3.56572C17.6228 5.14492 18.5 7.25212 18.5 9.49972C18.5 11.7473 17.6228 13.8545 16.0304 15.4337Z",
                        fill: "#0387D9"
                    }
                }
    
                defs {
                    clipPath {
                        id: "clip0_366_3588",
                        rect {
                            width: "18",
                            height: "18",
                            fill: "white",
                            transform: "matrix(1 0 0 -1 0.5 18.5)"
                        }
                    }
                }
            }
        },
        _ => rsx! {
            svg {
                width: "18",
                height: "14",
                view_box: "0 0 18 14",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M17.25 2.17288C17.038 2.0697 16.801 2.02909 16.5668 2.0558C16.3326 2.08252 16.1108 2.17545 15.9275 2.32371L13.7917 4.03204V3.66621C13.8406 3.24399 13.7934 2.81618 13.6537 2.41476C13.514 2.01335 13.2852 1.64873 12.9847 1.34818C12.6841 1.04763 12.3195 0.818925 11.9181 0.679182C11.5167 0.539439 11.0889 0.492272 10.6667 0.541209H3.16667C2.74445 0.492272 2.31663 0.539439 1.91522 0.679182C1.5138 0.818925 1.14918 1.04763 0.848634 1.34818C0.548083 1.64873 0.319382 2.01335 0.17964 2.41476C0.039897 2.81618 -0.0072702 3.24399 0.0416665 3.66621V10.3329C-0.0072702 10.7551 0.039897 11.1829 0.17964 11.5843C0.319382 11.9857 0.548083 12.3504 0.848634 12.6509C1.14918 12.9515 1.5138 13.1802 1.91522 13.3199C2.31663 13.4596 2.74445 13.5068 3.16667 13.4579H10.6667C11.0889 13.5068 11.5167 13.4596 11.9181 13.3199C12.3195 13.1802 12.6841 12.9515 12.9847 12.6509C13.2852 12.3504 13.514 11.9857 13.6537 11.5843C13.7934 11.1829 13.8406 10.7551 13.7917 10.3329V9.96704L15.9275 11.6754C16.1479 11.8534 16.4226 11.9504 16.7058 11.9504C16.8942 11.9501 17.0801 11.9077 17.25 11.8262C17.4629 11.7248 17.6425 11.565 17.768 11.3654C17.8934 11.1657 17.9594 10.9345 17.9583 10.6987V3.30038C17.9594 3.0646 17.8934 2.83338 17.768 2.63373C17.6425 2.43408 17.4629 2.27424 17.25 2.17288Z",
                    fill: "#0387D9"
                }
            }
        },
    };

    rsx! {
        DeleteModal {
            visible: delete_modal_visible.clone(),
            title: "Delete Project".to_string(),
            message: "Are you sure you want to delete this project? This action can't be undo".to_string(),
            item_type: "Project".to_string(),
            toast: toast.clone(),
        } 
        
            div {
            // onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            class: "transition-all duration-300",
            if hovered() {
                // HOVERED VIEW (Styled like the attached image)
                div {
                    class: "w-[270px] h-[203px] relative group min-h-full bg-[#D9D9D9] border border-[#BEBEBE] hover:border-[#A0A0A0] rounded-xl pt-3 px-3 shadow-sm hover:shadow-md hover:scale-[1.01] cursor-pointer transition-all duration-200 flex flex-col gap-1",
                    // Header with icon and name
                    div {
                        class: "flex items-center gap-3",
                        div {
                            class: "w-8 h-8 border-[0.5px] border-[#BEBEBE] rounded-full bg-[#FFFFFF] flex items-center justify-center",
                            span {
                                class:"h-[15px] w-[15px] flex items-center justify-center",
                                { icon_svg }
                            }
                        }
                        span {  
                            class: "flex flex-col",

                            h2 {
                                class: "max-w-[200px] font-normal text-sm text-[#151515] truncate",
                                title: "{name}",
                                "{name}"
                            }
                            p {
                                class: "h-[14px] text-[9px] text-[#787878]",
                                "Created : {created_at}    Last edited : {updated_at}"
                            }
                        }  
                    }

                    // p {
                    //     class: "h-[14px] text-[9px] text-[#787878]",
                    //     // "Created : {created_at}    Last edited : {updated_at}"
                    // }

                    // hr { class: "border-t border-gray-200 my-1" }

                    // Description Card
                    div {
                        class: "w-[240px] min-h-[130px] bg-[#FFFFFF] rounded-t-[14px] mt-4 p-3 -mb-[15px] shadow-[0_4px_6px_0_#00000040]",

                        div {
                            class: "font-normal text-[10px] text-[#404040] mb-1",
                            "Description"
                        }
                        div {
                            class: "text-xs text-[#313131] font-normal leading-tight overflow-hidden text-ellipsis line-clamp-[6] flex-1",
                            "{description}"
                        }
                    }
                }

            } else {
                // ORIGINAL VIEW (what you already had)
                div {
                    class: "w-[270px] h-[203px] relative group border border-[#BEBEBE] hover:border-[#A0A0A0] rounded-xl p-3 shadow-sm hover:shadow-md hover:scale-[1.01] cursor-pointer transition-all duration-200 flex flex-col gap-1",
                    div {  
                        class: "w-full h-full flex flex-col gap-2",
                            onmouseenter: move |_| hovered.set(true),
                            onmouseleave: move |_| hovered.set(false),
                        // Header with icon and name
                        div {
                            class: "flex items-center gap-3",
                            div {
                                class: "w-8 h-8 border-[0.5px] border-[#BEBEBE] rounded-full bg-[#FFFFFF] flex items-center justify-center",
                                span {
                                    class:"h-[15px] w-[15px] flex items-center justify-center",
                                    { icon_svg }
                                }
                            }
                            span {  
                                class: "flex flex-col",

                                h2 {
                                    class: "max-w-[200px] font-normal text-sm text-[#151515] truncate",
                                    title: "{name}",
                                    "{name}"
                                }
                                p {
                                    class: "h-[14px] text-[9px] text-[#787878]",
                                    "Created : {created_at}    Last edited : {updated_at}"
                                }
                            }  
                        }

                        hr { class: "border-t border-gray-200" }

                        // Platform info
                        div {
                            class: "flex items-center gap-2",
                            img {  
                                src:"{PLATFORM}",
                                alt: "Platform",
                            }
                            span { class: "text-[10px] text-[#787878] font-medium", "Platform :" }
                            span { class: "text-[10px] text-[#158826] font-medium", "{platform}" }
                            img {  
                                src:"{SIMULATION}",
                                alt: "Simulation",
                            }
                        }

                        // Metrics
                        div {
                            class: "flex flex-col gap-1 text-[13px]",
                            // First row: min_if, max_if, search_area
                            div {
                                class: "flex flex-wrap gap-1",
                                if let Some((key, val)) = min_parts {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "{key}:" }
                                        span { class: "text-[#151515] ", "{val}" }
                                    }
                                }else {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "min_if:" }
                                        span { class: "text-[#151515] ", "0" }
                                    }
                                }
                                if let Some((key, val)) = max_parts {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "{key}:" }
                                        span { class: "text-black ", "{val}" }
                                    }
                                }else {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "max_if:" }
                                        span { class: "text-black ", "0" }
                                    }
                                }
                                if let Some((key, val)) = search_parts {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "{key}:" }
                                        span { class: "text-black ", "{val}" }
                                    }
                                }else {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "search_area:" }
                                        span { class: "text-black ", "0" }
                                    }
                                }
                            }

                            // Second row: nn_capacity, committed_neurons
                            div {
                                class: "flex flex-wrap gap-1",
                                if let Some((key, val)) = nn_parts {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "{key}:" }
                                        span { class: "text-black ", "{val}" }
                                    }
                                }else {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "nn_capacity:" }
                                        span { class: "text-black ", "0" }
                                    }
                                }
                                if let Some((key, val)) = committed_parts {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "{key}:" }
                                        span { class: "text-black", "{val}" }
                                    }
                                }else {
                                    span {
                                        class: "bg-[#F0F0F0] h-[19px] px-2 py-1 justify-center items-center font-normal rounded text-[10px] flex gap-1",
                                        span { class: "text-[#555555]", "committed_neurons:" }
                                        span { class: "text-black", "0" }
                                    }
                                }
                            }
                        }
                        
                        hr { class: "border-t border-gray-200" }

                        } 
                    // Footer actions
                    div {
                        class: "h-[26px] flex justify-between items-center",
                        div {
                            class: "justify-end items-end flex gap-2",
                            svg {
                                    width: "18",
                                    height: "18",
                                    view_box: "0 0 18 18",
                                    fill: "none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        d: "M18 8.39686C18 9.60162 17.2254 10.6281 16.1311 11.0069C16.2644 11.3336 16.3346 11.6849 16.3346 12.0406C16.3346 13.3836 15.3596 14.5062 14.0712 14.7557C13.9759 16.1965 12.7586 17.3398 11.2759 17.3398C10.3391 17.3398 9.50889 16.883 9.00002 16.1836C8.49115 16.8831 7.66088 17.3398 6.72414 17.3398C5.24144 17.3398 4.02409 16.1965 3.92884 14.7557C2.64041 14.5062 1.66535 13.3836 1.66535 12.0406C1.66535 11.6849 1.73562 11.3336 1.86887 11.0069C0.7746 10.6282 0 9.60162 0 8.39686C0 7.26269 0.696943 6.27183 1.70972 5.84889C1.55965 5.50508 1.4803 5.13186 1.4803 4.75307C1.4803 4.0126 1.78154 3.31481 2.32848 2.78831C2.76353 2.36956 3.30857 2.10256 3.89513 2.01505C4.05118 1.50872 4.34936 1.05164 4.75904 0.699104C5.28275 0.248468 5.95412 0.000297546 6.64948 0.000297546C7.6194 0.000297546 8.47816 0.476902 8.99995 1.20488C9.52173 0.476902 10.3805 0.000297546 11.3504 0.000297546C12.0457 0.000297546 12.7171 0.248468 13.2408 0.699104C13.6505 1.05164 13.9487 1.50875 14.1048 2.01505C14.6913 2.10256 15.2364 2.36956 15.6714 2.78831C16.2184 3.31481 16.5196 4.0126 16.5196 4.75307C16.5196 5.13189 16.4402 5.50508 16.2902 5.84889C17.3031 6.27187 18 7.26273 18 8.39686ZM6.64951 1.0559C5.74796 1.0559 4.97223 1.71417 4.845 2.58712C4.845 2.58719 4.845 2.58726 4.84496 2.58729C4.83275 2.67111 4.82656 2.75703 4.82656 2.84264C4.82656 3.82784 5.64433 4.62935 6.64951 4.62935C6.941 4.62935 7.17731 4.86567 7.17731 5.15715C7.17731 5.44864 6.941 5.68495 6.64951 5.68495C5.15774 5.68495 3.92761 4.55859 3.78503 3.12196C3.06962 3.3468 2.5359 4.00278 2.5359 4.75307C2.5359 5.15318 2.67243 5.52788 2.93077 5.83665C3.05505 5.98524 3.08798 6.18999 3.01652 6.37004C2.94506 6.55009 2.78066 6.67652 2.5883 6.69942C1.71451 6.80336 1.0556 7.5331 1.0556 8.39686C1.0556 9.31439 1.78777 10.0649 2.72239 10.1055C2.92229 10.1142 3.10005 10.2351 3.18155 10.4178C3.263 10.6006 3.23418 10.8137 3.10706 10.9682C2.85445 11.2752 2.72092 11.646 2.72092 12.0406C2.72092 12.8481 3.29576 13.5263 4.06603 13.7052C4.43162 12.6375 5.45848 11.8826 6.63684 11.8826C6.92833 11.8826 7.16464 12.1189 7.16464 12.4104C7.16464 12.7019 6.92833 12.9382 6.63684 12.9382C5.807 12.9382 5.10042 13.5449 4.9932 14.3493C4.98339 14.423 4.97839 14.4985 4.97839 14.5736C4.97839 15.5169 5.7615 16.2842 6.72407 16.2842C7.70979 16.2854 8.49291 15.4836 8.47243 14.521V2.85419C8.47236 2.85028 8.47183 2.84651 8.47183 2.84261C8.47183 2.83881 8.47211 2.83508 8.47211 2.83128C8.46592 1.85129 7.65082 1.0559 6.64951 1.0559ZM15.4117 6.69942C15.2193 6.67655 15.0549 6.55009 14.9834 6.37004C14.912 6.18999 14.9449 5.98524 15.0692 5.83665C15.3275 5.52788 15.4641 5.15318 15.4641 4.75307C15.4641 4.00278 14.9303 3.34676 14.215 3.12196C14.0724 4.55859 12.8422 5.68495 11.3505 5.68495C11.059 5.68495 10.8227 5.44864 10.8227 5.15715C10.8227 4.86567 11.059 4.62935 11.3505 4.62935C12.3556 4.62935 13.1734 3.82784 13.1734 2.84264C13.1734 2.7571 13.1672 2.67121 13.155 2.58729C13.155 2.58722 13.155 2.58715 13.155 2.58708C13.0277 1.71417 12.252 1.0559 11.3505 1.0559C10.3491 1.0559 9.53408 1.85133 9.52782 2.83131C9.52782 2.83511 9.5281 2.83884 9.5281 2.84264V14.5331C9.51209 15.4913 10.2956 16.2852 11.2759 16.2843C12.2385 16.2843 13.0215 15.5169 13.0215 14.5736C13.0215 14.4986 13.0165 14.4231 13.0067 14.3494C12.8995 13.5449 12.1929 12.9382 11.3631 12.9382C11.0716 12.9382 10.8353 12.7019 10.8353 12.4104C10.8353 12.1189 11.0716 11.8826 11.3631 11.8826C12.5415 11.8826 13.5683 12.6375 13.9339 13.7052C14.7041 13.5263 15.279 12.8482 15.279 12.0407C15.279 11.646 15.1455 11.2752 14.8929 10.9682C14.7658 10.8137 14.737 10.6006 14.8184 10.4179C14.8999 10.2352 15.0777 10.1142 15.2776 10.1055C16.2122 10.065 16.9444 9.31443 16.9444 8.3969C16.9444 7.5331 16.2855 6.80336 15.4117 6.69942Z",
                                        fill: "#158826",
                                    }
                                }
                            }
                            div {
                                class: "flex gap-[10px]",
                                button {
                                    class: "bg-[#F0F0F0] px-[10px] py-1 rounded-[3px] text-xs font-medium text-[#101010]",
                                    onclick: move |_| {
                                        delete_modal_visible.set(true);
                                    },
                                    "Delete"
                                }
                            button {
                                    onclick: move |_| show_edit_modal.set(true),
                                    class: "bg-[#101010] px-[10px] py-1 rounded-[3px] text-xs font-medium text-[#FFFFFF] ",
                                    "Edit"
                                }
                            }
                        }

                    }

                    if *show_edit_modal.read() {
                        ProjectForm {
                            show_modal: show_edit_modal,
                            project: project.clone(),
                            is_updating : is_updating,
                        }
                    }

            }
        }
        if let Some(toast_val) = toast() {
            {
                let (bg_class, msg) = match toast_val {
                    ToastType::Success(msg) => ("bg-green-600", msg),
                    ToastType::Error(msg) => ("bg-red-500", msg),
                };
                rsx!(
                    div {
                        class: format!("fixed top-6 right-6 z-50 px-4 py-2 rounded shadow-lg text-white font-semibold transition-all duration-300 {}", bg_class),
                        "{msg}"
                    }
                )
            }
        }
    }

}

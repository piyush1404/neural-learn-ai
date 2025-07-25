use dioxus::{html::tr, prelude::*};

use crate::components::{knowledge_modal::KnowledgeModal, option_annotations::OptionForAnnotation};
use crate::views::project_details::{AppState, SharedData};
use crate::store::project::ProjectState;

#[component]
pub fn MenuBar(app_state: AppState) -> Element {
    let mut shared = app_state.shared;
    let shared_props = shared().clone();
    // println!("MenuBar entered with algorithm: ");
    let project_state = use_context::<ProjectState>();
    // println!("Project ID: {}", (project_state.project_id)());
    // println!("Project Name: {}", (project_state.project_name)());
    let project_name = project_state.project_name.clone();
    let mut selected_annotation = use_signal(|| "".to_string()); 
    let mut show_modal_options = use_signal(|| false);
    let mut selected_knowledge = use_signal(|| "".to_string());
    let mut show_modal_knowledge_view = use_signal(|| false);
    rsx! {
        div {
            class:"h-[40px] flex items-center gap-2 mx-6",
            div {
                class:"w-full flex items-center gap-2",
                span {
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
                span { class: "text-[#404040] font-nomal text-sm", {project_name} }
                span {
                    svg {
                        width: "18",
                        height: "18",
                        view_box: "0 0 18 18",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M15 6.45023L8.715 12.7352C8.4975 12.9527 8.2125 13.0877 7.905 13.1177L5.7075 13.3202C5.7075 13.3202 5.6475 13.3202 5.625 13.3202C5.3775 13.3202 5.1375 13.2227 4.965 13.0427C4.77 12.8477 4.6725 12.5702 4.695 12.2927L4.8975 10.0952C4.9275 9.78773 5.0625 9.50274 5.28 9.28523L11.55 3.00023L15 6.45023ZM16.455 2.07773L15.9225 1.54523C15.12 0.742734 13.8075 0.742734 13.005 1.54523L12.345 2.20523L15.795 5.65523L16.455 4.99523C16.845 4.60523 17.0625 4.08773 17.0625 3.54023C17.0625 2.99273 16.845 2.46773 16.455 2.08523V2.07773ZM15.5625 14.2577V9.90023C15.5625 9.59273 15.3075 9.33773 15 9.33773C14.6925 9.33773 14.4375 9.59273 14.4375 9.90023V14.2577C14.4375 15.1877 13.68 15.9452 12.75 15.9452H3.75C2.82 15.9452 2.0625 15.1877 2.0625 14.2577V5.25023C2.0625 4.32023 2.82 3.56273 3.75 3.56273H8.1075C8.415 3.56273 8.67 3.30773 8.67 3.00023C8.67 2.69273 8.415 2.43773 8.1075 2.43773H3.75C2.1975 2.43773 0.9375 3.69773 0.9375 5.25023V14.2502C0.9375 15.8027 2.1975 17.0627 3.75 17.0627H12.75C14.3025 17.0627 15.5625 15.8027 15.5625 14.2502V14.2577Z",
                            fill: "#0387D9",
                        }
                    }

                }
            }
            div {
                class:"w-full flex items-center justify-end gap-2",
                div {select {
                    class: "px-3 py-1 font-normal text-[11px] w-[105px] text-[#555555] appearance-none",
                    value: "{selected_annotation}",

                    style: r#"
                    color: #555555;
                    background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                    background-repeat: no-repeat;
                    background-position: right 0.75rem center;
                    background-size: 10px 6px;
                "#,
                onchange: move |e| {
                    let value = e.value();
                    selected_annotation.set(value.clone());
                    if value == "options" {
                        show_modal_options.set(true);
                    }
                },

                option { value: "", disabled: true, hidden: true, selected: selected_annotation() == "", "Annotations" }
                option { value: "options", "Options" }
                option { value: "clear_image", "Clear in current image" }
                option { value: "clear_all", "Clear all" }
                option { value: "load", "Load" }
                option { value: "review_all", "Review all" }
                option { value: "save_all", "Save all" }

                }

                if *show_modal_options.read() {
        
                    OptionForAnnotation { show_modal_options: show_modal_options, selected_annotation: selected_annotation }
                }}
                
                div {select {
                    class: "px-3 py-1 font-normal text-[11px] w-[100px] text-[#555555] appearance-none",
                    style: r#"
                    color: #555555;
                    background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                    background-repeat: no-repeat;
                    background-position: right 0.75rem center;
                    background-size: 10px 6px;
                "#,
                onchange: move |e| {
                    let value = e.value();
                    selected_knowledge.set(value.clone());
                    if value == "view" {
                        show_modal_knowledge_view.set(true);
                    } else if value == "clear" {
                        // logic to clear
                    } else if value == "undo_last_learning" {
                        // logic to undo
                    }
                },
                option { value: "", disabled: true,hidden: true, selected: selected_knowledge() == "", "Knowledge" }
                option { value: "view", "View" }
                option { value: "clear", "Clear" }
                option { value: "undo_last_learning", "Undo last learning" }
                }

                if show_modal_knowledge_view(){
                    KnowledgeModal { show_modal_knowledge_view: show_modal_knowledge_view, selected_knowledge: selected_knowledge }
                }}

                div {select {
                    class: "px-3 py-1 font-normal text-[11px] w-20 text-[#555555] appearance-none",
                    style: r#"
                    color: #555555;
                    background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                    background-repeat: no-repeat;
                    background-position: right 0.75rem center;
                    background-size: 10px 6px;
                "#,
                    option { value: "", disabled: true,hidden: true, selected: true, "Results" }
                    option { value: "clear_in_current_image", "Clear in current image" }
                    option { value: "clear_all", "Clear all" }
                    option { value: "review_all", "Review all" }
                    option { value: "save_all", "Save all" }
                    option { value: "details_at_cursor", "Details at cursor" }
                }}

                div{select {
                    class: "px-3 py-1 font-normal text-[11px] w-[80px] text-[#555555] appearance-none",
                    style: r#"
                    color: #555555;
                    background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                    background-repeat: no-repeat;
                    background-position: right 0.75rem center;
                    background-size: 10px 6px;
                "#,
                    option { value: "", disabled: true, hidden: true, selected: true, "Options" }
                    option { value: "cursor_color", "Cursor Color" }
                    option { value: "cursor_info", "Cursor info" }
                    option { value: "timing_info", "Timing info" }
                    option { value: "save_transform_image", "Save transform image" }
                }}

            }
            div {
                class:"flex items-center space-x-[10px]",
                div {
                    class: "flex items-center border border-[#EDEDED] rounded-[50px] px-3 py-1 bg-[#F7F7F7] shadow-sm",

                    div {
                        class: "w-4 h-4 bg-[#CACACA] flex items-center justify-center",
                        span {
                            class:"h-[15px] w-[15px] flex items-center justify-center",
                            svg {
                                width: "12",
                                height: "12",
                                view_box: "0 0 12 12",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M11.5313 5.53115C11.8272 5.23147 11.9931 4.82728 11.9931 4.40615C11.9931 3.98501 11.8272 3.58083 11.5313 3.28115L8.7301 0.468649C8.42592 0.181466 8.02343 0.0214844 7.6051 0.0214844C7.18677 0.0214844 6.78428 0.181466 6.4801 0.468649L0.480097 6.46865C0.184218 6.76833 0.0183105 7.17251 0.0183105 7.59365C0.0183105 8.01478 0.184218 8.41897 0.480097 8.71865L3.28135 11.5199C3.58103 11.8158 3.98521 11.9817 4.40635 11.9817C4.82748 11.9817 5.23166 11.8158 5.53135 11.5199L11.5313 5.53115ZM7.03135 1.03115C7.18095 0.882757 7.38313 0.799492 7.59385 0.799492C7.80456 0.799492 8.00674 0.882757 8.15635 1.03115L10.9576 3.8324C11.106 3.982 11.1893 4.18418 11.1893 4.3949C11.1893 4.60562 11.106 4.8078 10.9576 4.9574L6.8326 9.0824L2.9026 5.16365L7.03135 1.03115ZM4.9651 10.9649C4.81197 11.1064 4.61111 11.185 4.4026 11.185C4.19408 11.185 3.99323 11.1064 3.8401 10.9649L1.0351 8.16365C0.886706 8.01405 0.803441 7.81187 0.803441 7.60115C0.803441 7.39043 0.886706 7.18825 1.0351 7.03865L2.33635 5.7374L6.2701 9.66365L4.9651 10.9649Z",
                                    fill: "white"
                                }
                                path {
                                    d: "M11.5988 11.2011H7.20001C7.14431 11.1957 7.08809 11.202 7.03498 11.2196C6.98186 11.2372 6.93302 11.2657 6.89158 11.3034C6.85014 11.341 6.81703 11.3868 6.79437 11.438C6.77171 11.4892 6.76001 11.5445 6.76001 11.6005C6.76001 11.6564 6.77171 11.7118 6.79437 11.7629C6.81703 11.8141 6.85014 11.86 6.89158 11.8976C6.93302 11.9352 6.98186 11.9637 7.03498 11.9813C7.08809 11.9989 7.14431 12.0052 7.20001 11.9998H11.5988C11.6545 12.0052 11.7107 11.9989 11.7638 11.9813C11.8169 11.9637 11.8657 11.9352 11.9072 11.8976C11.9486 11.86 11.9817 11.8141 12.0044 11.7629C12.027 11.7118 12.0388 11.6564 12.0388 11.6005C12.0388 11.5445 12.027 11.4892 12.0044 11.438C11.9817 11.3868 11.9486 11.341 11.9072 11.3034C11.8657 11.2657 11.8169 11.2372 11.7638 11.2196C11.7107 11.202 11.6545 11.1957 11.5988 11.2011Z",
                                    fill: "white"
                                }
                            }

                        }
                    }

                    button {
                        onclick: move |evt| {
                            evt.prevent_default();
                            shared.set(SharedData {
                                algorithm: shared_props.algorithm.clone(),
                                clear_clicked: true
                            });
                        },
                        class: "ml-2 outline-none border-none bg-transparent font-normal text-xs text-[#555555]",
                        "Clear"
                    }
                }

                div {
                    class: "flex items-center border border-[#EDEDED] rounded-[50px] px-3 py-1 bg-[#F7F7F7] shadow-sm",

                    div {
                        class: "w-4 h-4 bg-[#CACACA] flex items-center justify-center",
                        span {
                            class:"h-[15px] w-[15px] flex items-center justify-center",
                            svg {
                                width: "12",
                                height: "12",
                                view_box: "0 0 12 12",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M11.5162 2.73376L9.26625 0.483756C9.23121 0.449001 9.18965 0.421504 9.14396 0.402842C9.09828 0.38418 9.04935 0.374721 9 0.375006H1.5C1.20163 0.375006 0.915483 0.493533 0.704505 0.704511C0.493526 0.915489 0.375 1.20164 0.375 1.50001V10.5C0.375 10.7984 0.493526 11.0845 0.704505 11.2955C0.915483 11.5065 1.20163 11.625 1.5 11.625H10.5C10.7984 11.625 11.0845 11.5065 11.2955 11.2955C11.5065 11.0845 11.625 10.7984 11.625 10.5V3.00001C11.6253 2.95065 11.6158 2.90173 11.5972 2.85604C11.5785 2.81035 11.551 2.7688 11.5162 2.73376ZM7.5 1.12501V3.37501H4.5V1.12501H7.5ZM3 10.875V8.25001C3 8.15055 3.03951 8.05517 3.10983 7.98484C3.18016 7.91451 3.27554 7.87501 3.375 7.87501H8.625C8.72446 7.87501 8.81984 7.91451 8.89016 7.98484C8.96049 8.05517 9 8.15055 9 8.25001V10.875H3ZM10.875 10.5C10.875 10.5995 10.8355 10.6948 10.7652 10.7652C10.6948 10.8355 10.5995 10.875 10.5 10.875H9.75V8.25001C9.75 7.95164 9.63147 7.66549 9.42049 7.45451C9.20952 7.24353 8.92337 7.12501 8.625 7.12501H3.375C3.07663 7.12501 2.79048 7.24353 2.5795 7.45451C2.36853 7.66549 2.25 7.95164 2.25 8.25001V10.875H1.5C1.40054 10.875 1.30516 10.8355 1.23483 10.7652C1.16451 10.6948 1.125 10.5995 1.125 10.5V1.50001C1.125 1.40055 1.16451 1.30517 1.23483 1.23484C1.30516 1.16452 1.40054 1.12501 1.5 1.12501H3.75V3.37501C3.75 3.57392 3.82902 3.76468 3.96967 3.90534C4.11032 4.04599 4.30109 4.12501 4.5 4.12501H7.5C7.69891 4.12501 7.88968 4.04599 8.03033 3.90534C8.17098 3.76468 8.25 3.57392 8.25 3.37501V1.12501H8.84625L10.875 3.15376V10.5Z",
                                    fill: "white",
                                }
                            }

                        }
                    }

                    button {
                        class: "ml-2 outline-none border-none bg-transparent font-normal text-xs text-[#555555]",
                        "Save"
                    }
                }
            }
        }
    }
}

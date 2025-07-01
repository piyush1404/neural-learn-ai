use dioxus::prelude::*;
use uuid::Uuid;
use crate::store::project_schema::{ Project, Category};
use crate::store::project::{ add_project };
use crate::date_format::{get_local_and_utc_iso};
#[derive(PartialEq, Props, Clone)]
pub struct ProjectFormProps {
    show_modal: Signal<bool>,
    project: Option<Project>,
}

#[component]
pub fn ProjectForm(props: ProjectFormProps) -> Element {
    let (_, utc_iso) = get_local_and_utc_iso();

    let mut show_modal = props.show_modal;
    let mut show_advanced = use_signal(|| false);

    let mut categories = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().categories.clone() } else { vec![
        Category { id: 0, name: "Background".to_string(), color: "#4C4C4C".to_string(), context_id: 0 },
    ] });


    let mut project_name = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().name.clone() } else { "".to_string() });
    let mut project_name_error=use_signal(|| false);
    let mut platform = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().platform.clone() } else { "Simulation".to_string() });
    let project_type = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().interface.clone() } else { "image".to_string() });

    let mut description = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().description.clone() } else { "".to_string() });
    let mut description_error=use_signal(|| false);
    let mut normalized = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().normalized } else { false });
    let mut algorithm = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().algorithm.clone() } else { "Subsample".to_string() });

    let mut roi_width = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().roi_width } else { 64 });
    let mut roi_height = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().roi_height } else { 64 });
    let mut block_width = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().block_width } else { 16 });
    let mut block_height = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().block_height } else { 16 });
    let mut range_min = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().if_field_range.min } else { 0 });
    let mut range_max = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().if_field_range.max } else { 255 });

    let selected_label = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().interface.clone() } else { "Image".to_string() });
    let mut selected_icon = use_signal(|| rsx!(svg {
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
    }));
    let mut show_options = use_signal(|| false);
    let mut show_modal_feature_extraction_panel = use_signal(|| false);


    let options = vec![
        ("Image", rsx!(
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
        )),
        ("Video", rsx!(
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
        )),
        ("Audio", rsx!(
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
        )),
    ];
    
    let option_list: Vec<_> = options
    .iter()
    .map(|(label, icon)| {
        let label = label.to_string();
        // let icon_element = icon.clone(); // icon is already an Element
        let mut selected_label = selected_label.clone();
        let icon_element = icon.clone().unwrap();          // Original
        let icon_element_for_display = icon_element.clone(); // For use outside closure        
        let mut show_options = show_options.clone();

        rsx!(
            li {
                class: "flex items-center gap-2 px-4 py-2 text-[#FFFFFF] rounded hover:bg-[#555555] cursor-pointer",
                onclick: move |_| {
                    selected_label.set(label.clone());
                    selected_icon.set(Ok(icon_element.clone()));
                    show_options.set(false);
                     if label == "Audio" {
                        show_modal_feature_extraction_panel.set(true);
                    }else {
                        show_modal_feature_extraction_panel.set(false);
                    }
                },
                {icon_element_for_display} // use the clone here
                span { "{label}" }
            }
        )
    })
    .collect();

    let  feature_extraction_panel_class = if show_modal_feature_extraction_panel() {
        "pt-3 w-[56%] opacity-50 pointer-events-none"
    } else {
        "pt-3 w-[56%]"
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div {
                class: "bg-[#FFFFFF] rounded-[10px] overflow-hidden shadow-lg max-h-[580px] flex flex-col",
                div {
                    class: "flex justify-between items-center px-5 py-2 border-b h-[42px]",
                    span { class: "text-sm font-normal text-[#404040]", "Project Details" }
                    button {    
                        class: " hover:text-black",
                        onclick: move |_| show_modal.set(false),
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
                    class:"bg-[#EFEFEF] px-5 pb-5 pt-[10px] ",
                    div {
                        class: "grid grid-cols-3 gap-4",
                    div {
                        label { class: "block mb-1 font-normal text-xs text-[#404040]", "Project Name" }
                        input {
                            class: "w-full border-[0.5px] border-[#8F8F8F] rounded px-4 py-1 font-normal text-xs text-[#313131",
                            maxlength: "50",
                            value: "{project_name}",
                            oninput: move |e| {
                                let value = e.value().to_string();
                                project_name_error.set(value.len() > 50);
                                project_name.set(value);
                            }
                        }
                        if  project_name_error() {
                            p {
                                class: "text-xs text-red-500 mt-1",
                                "Project name cannot exceed 50 characters."
                            }
                        }
                    }
                    div {
                        label {
                            class: "block mb-1 text-xs font-normal text-[#4D4D4D]",
                            "Select Platform"
                        }
                        select {
                            class: "w-full border-[0.5px] border-[#8F8F8F] rounded px-4 py-1 font-normal text-xs text-[#313131] appearance-none pr-7",
                            value: "{platform}",
                            style: r#"
                                color: #555555;
                                background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                                background-repeat: no-repeat;
                                background-position: right 0.75rem center;
                                background-size: 10px 6px;
                            "#,
                            onchange: move |e| {
                                let value = e.value().to_string();
                                platform.set(value);
                            },
                            option { value: "Simulation", "Simulation" }
                            option { value: "Brilliant", "Brilliant" }
                            option { value: "Neuro Shield", "Neuro Shield" }
                        }
                    }                    
                    div {
                        label { class: "block mb-1 text-xs font-normal text-[#4D4D4D]", "Project Type" }
                        div {
                            class: "relative w-full text-xs ",
                
                            // Trigger button
                            div {
                                class: "w-full bg-white border border-[#8F8F8F] rounded px-4 py-1 pr-5 flex justify-between items-center text-[#313131] cursor-pointer",
                                onclick: move |_| show_options.set(!show_options()),
                
                                div {
                                    class: "flex items-center gap-2",
                                    {selected_icon()}  // render the Element directly
                                    span { "{selected_label()}" }
                                }
                                
                
                                svg {
                                    width: "12",
                                    height: "6",
                                    view_box: "0 0 12 6",
                                    fill: "none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        d: "M6.04749 5.95753C5.86584 5.95753 5.6842 5.88817 5.5457 5.74973L1.18742 1.39141C0.910181 1.11417 0.910181 0.664667 1.18742 0.387536C1.46456 0.110405 1.91397 0.110405 2.19123 0.387536L6.04749 4.24402L9.90378 0.387671C10.181 0.110539 10.6304 0.110539 10.9075 0.387671C11.1849 0.664802 11.1849 1.1143 10.9075 1.39155L6.54929 5.74987C6.41072 5.88832 6.22909 5.95753 6.04749 5.95753Z",
                                        fill: "#313131"
                                    }
                                }
                            }
                
                            // Dropdown options
                            if show_options() {
                                ul {
                                    class: "absolute w-full bg-white border mt-1 rounded shadow z-10",
                                    {option_list.into_iter()}
                                }
                            }
                        }
                       
                    }
                }
                div {
                    class: "col-span-3 mt-[10pxs]",
                    div {
                        class: "flex justify-between items-center bg-[#EFEFEF] mb-1 text-xs text-[#404040]",
                        span { "Description" }
                        span { "{description.read().len()}/100 alphabets" }
                    }
                    textarea {
                        class: "w-full border border-[#8F8F8F] rounded font-poppins font-normal px-4 py-1 text-xs text-[#313131] resize-none bg-[#FFFFFF] appearance-none outline-none",
                        maxlength: "100",
                        value: "{description}",
                        oninput: move |e| {
                            let value = e.value().to_string();
                            description_error.set(value.len() > 100);
                            description.set(value);
                        }
                    }
                    if  project_name_error() {
                        p {
                            class: "text-xs text-red-500 mt-1",
                            "Description cannot exceed 50 characters."
                        }
                    }
                } 
               } 
                div {
                    class: "text-center bg-[#FAFAFA] py-[13px]",
                    button {
                        class: "px-[10px] py-1 bg-[#0387D9] text-[#FFFFFF] font-normal text-xs rounded-[13px] text-center",
                        onclick: move |_| {
                            let current = *show_advanced.read();
                            show_advanced.set(!current);
                        },
                        "Advance " { if *show_advanced.read() { "-" } else { "+" } }
                    }
                }
                if *show_advanced.read() {
                    div {
                        class: "flex px-5 gap-4 border border-[#DDDDDD] bg-[#FAFAFA]",
                
                        // Categories Panel
                        div {
                            class: "pt-3 border-r w-[22%] flex flex-col justify-between h-[266px]",
                        
                            div {
                                class: "relative mb-2 overflow-y-auto space-y-2 h-[266px]",
                        
                                span { class: "block mb-2 text-xs text-[#404040] font-normal", "Categories" }
                        
                                div {
                                    class: "flex items-center gap-2 text-xs text-[#404040] mb-1",
                                    span { class: "w-[91px] text-[10px] h-[15px] rounded", "Name" }
                                    span { class: "w-[28px] text-[10px] h-[15px]", "Color" }
                                }
                        
                                for (index, category) in categories.read().iter().cloned().enumerate() {
                                    div {
                                        class: "flex items-center gap-2",
                        
                                        input {
                                            class: "border p-1 w-[91px] rounded text-[11px] font-normal text-[#404040] h-5",
                                            value: "{category.name}",
                                            oninput: move |e| {
                                                let mut updated = categories.write().clone();
                                                updated[index].name = e.value().clone();
                                                categories.set(updated);
                                            }
                                        }
                        
                                        input {
                                            r#type: "color",
                                            class: "appearance-none border border-gray-300 w-[28px] h-[20px] p-0 rounded cursor-pointer",
                                            value: category.color.clone(),
                                            onchange: move |e| {
                                                let mut updated = categories.write().clone();
                                                updated[index].color = e.value().clone();
                                                categories.set(updated);
                                            }
                                        }
                        
                                        div {
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
                                            if index == categories.read().len() - 1 {
                                                button {
                                                    class: "w-[19px] h-[19px] rounded-sm border border-blue-300 text-blue-500 text-sm flex items-center justify-center",
                                                    onclick: move |_| {
                                                        let mut updated = categories.write().clone();
                                                        let new_id = updated.last().map_or(1, |c| c.id + 1);
                                                        updated.push(Category {
                                                            id: new_id,
                                                            name: "".to_string(),
                                                            color: "#000000".to_string(),
                                                            context_id: 101, // Adjust as per your app context
                                                        });
                                                        categories.set(updated);
                                                    },
                                                    "+"
                                                }
                                            }
                                        }
                                    }
                                }
                        
                                div {
                                    class: "absolute bottom-[20px] right-[10px]",
                                    button {
                                        class: "bg-[#0387D9] text-[#FFFFFF] px-4 py-1 rounded-[13px] text-sm",
                                        onclick: move |_| {
                                            categories.set(vec![
                                                Category {
                                                    id: 1,
                                                    name: "Background".to_string(),
                                                    color: "#4C4C4C".to_string(),
                                                    context_id: 101,
                                                },
                                                Category {
                                                    id: 2,
                                                    name: "Object".to_string(),
                                                    color: "#F85858".to_string(),
                                                    context_id: 101,
                                                },
                                            ]);
                                        },
                                        "Reset"
                                    }
                                }
                            }
                        }
                
                        // Context Panel
                        div {
                            class: "pt-3 border-r w-[22%] opacity-50 cursor-pointer-none",
                            span { class: "block mb-2 text-xs text-[#404040] font-normal", "Context" }
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
                                        class: "px-2 py-1 text-[11px] text-[#40404080] bg-[#FFFFFF] border-[0.5px] border-[#8F8F8F] rounded",
                                        "Enter Context"
                                    }
                                }
                                div {
                                    class: "flex",
                                    div {
                                        class: "flex flex-col items-center w-5 mr-1",
                                        div { class: "w-px h-3 bg-gray-300" }
                                        div { class: "w-full h-px bg-gray-300" }
                                        div { class: "w-px h-full grow" }
                                    }
                                    div {
                                        class: "flex items-center gap-1",
                                        button {
                                            class: "flex items-center justify-center w-5 h-5 text-sm border rounded text-blue-500",
                                            "+"
                                        }
                                        div {
                                            class: "px-2 py-1 text-[11px] text-[#40404080] bg-[#FFFFFF] border-[0.5px] border-[#8F8F8F] rounded",
                                            "Sub Context"
                                        }
                                    }
                                }
                            }
                        }
                
                        // Feature Extraction Panel
                        div {
                            class: feature_extraction_panel_class,
                            span { class: "block mb-2 text-xs text-[#404040] font-normal", "Feature Extraction" }
                
                            div {
                                class: "mb-4",
                                label { class: "mb-1 font-normal text-[10px] text-[#404040]", "Algorithm" }
                                div {
                                    class: "flex gap-4 items-center",
                
                                    div {
                                        class: "flex flex-col text-xs",
                                        select {
                                            class: "border rounded px-2 py-1 text-xs appearance-none pr-7",
                                            value: "{algorithm}",
                                            style: r#"
                                            color: #555555;
                                            background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                                            background-repeat: no-repeat;
                                            background-position: right 0.75rem center;
                                            background-size: 10px 6px;
                                        "#,
                                        onchange: move |e| algorithm.set(e.value()),
                                        option { "Subsample" }
                                        option {  "Subsample RGB" }
                                        option { "Histogram" }
                                        option {  "Histogram Cumulative" }
                                        option {  "Histogram RGB" }
                                        option {  "Histogram Cumulative RGB" }
                                        option {  "Composite Profile" }
                                        option {  "Horizontal Profile" }
                                        option {  "Vertical Profile" }
                                        }
                                    }
                
                                    div {
                                        class: "flex items-center mb-2 gap-2 mt-2",
                                        input { r#type: "checkbox", class: "w-4 h-4 accent-[#0387D9] border border-[#0387D9] rounded", checked: *normalized.read(),
                                        onchange: move |e| normalized.set(e.value().parse::<bool>().unwrap_or(false)) }
                                        label { class: "text-[10px] mb-1 text-[#404040] font-normal", "Normalize" }
                                    }
                                }
                            }
                
                            div { class: "flex items-center justify-end pr-10", label { class: "font-normal text-[10px] text-[#404040]", "Influence field range " } }
                
                            div {
                                class: "grid grid-cols-6 gap-4 text-xs mb-4",
                                div {
                                    class: "flex flex-col w-[62px]",
                                    label {
                                        class: "mb-1 font-normal text-[10px] text-[#404040]",
                                        "Width"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "{roi_width}",
                                        class: "border rounded px-2 py-1 text-xs",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse() {
                                                roi_width.set(v);
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col w-[62px]",
                                    label {
                                        class: "mb-1 font-normal text-[10px] text-[#404040]",
                                        "Height"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "{roi_height}",
                                        class: "border rounded px-2 py-1 text-xs",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse() {
                                                roi_height.set(v);
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col w-[62px]",
                                    label {
                                        class: "mb-1 font-normal text-[10px] text-[#404040]",
                                        "Block Width"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "{block_width}",
                                        class: "border rounded px-2 py-1 text-xs",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse() {
                                                block_width.set(v);
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col w-[62px]",
                                    label {
                                        class: "mb-1 font-normal text-[10px] text-[#404040]",
                                        "Block Height"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "{block_height}",
                                        class: "border rounded px-2 py-1 text-xs",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse() {
                                                block_height.set(v);
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col w-[62px]",
                                    label {
                                        class: "mb-1 font-normal text-[10px] text-[#404040]",
                                        "Max"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "{range_max}",
                                        class: "border rounded px-2 py-1 text-xs",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse() {
                                                range_max.set(v);
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col w-[62px]",
                                    label {
                                        class: "mb-1 font-normal text-[10px] text-[#404040]",
                                        "Min"
                                    }
                                    input {
                                        r#type: "number",
                                        value: "{range_min}",
                                        class: "border rounded px-2 py-1 text-xs",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse() {
                                                range_min.set(v);
                                            }
                                        }
                                    }
                                }
                            }

                            div {
                                class: "flex justify-end gap-2",
                                button {
                                    class: "bg-[#0387D9] hover:bg-blue-600 text-[#FFFFFF] text-xs font-normal py-1 px-4 rounded-[13px]",
                                    "Suggest"
                                }
                                button {
                                    class: "bg-[#0387D9] hover:bg-blue-600 text-[#FFFFFF] text-xs font-normal py-1 px-4 rounded-[13px]",
                                    "Validate"
                                }
                            }
                        }
                    }
                    div {
                        class: "flex justify-between items-center px-5 py-[13px] bg-[#FAFAFA] border-t",
                        div {
                            class: "w-full flex justify-end items-center gap-3",
                            button {
                                class: "text-xs rounded-[3px] px-4 py-1 bg-[#1010101A] text-[#101010]",
                                onclick: move |_| show_modal.set(false),
                                "Cancel"
                            }
                            button {
                                onclick: move |_| {
                                    let id = Uuid::new_v4().to_string();
                                    if props.project.is_none(){
                                        let project_form_data = serde_json::json!({
                                            "id": id,
                                            "name": project_name.read().to_string(),
                                            "platform": platform.read().to_string(),
                                            "interface": project_type.read().to_string(),
                                            "type": "vision".to_string(),
                                            "description": description.read().to_string(),
                                            "created_at": utc_iso,
                                            "updated_at": utc_iso,
                                            "categories": categories.read().iter().map(|cat| {
                                                serde_json::json!({
                                                    "id": cat.id,
                                                    "name": cat.name,
                                                    "color": cat.color,
                                                    "context_id": cat.context_id
                                                })
                                            }).collect::<Vec<_>>(),
                                            "feature_extraction": {
                                                "algorithm": algorithm.read().to_string(),
                                                "normalized": *normalized.read(),
                                                "roi_width": *roi_width.read(),
                                                "roi_height": *roi_height.read(),
                                                "block_width": *block_width.read(),
                                                "block_height": *block_height.read(),
                                                "if_field_range": {
                                                    "min": *range_min.read(),
                                                    "max": *range_max.read()
                                                }
                                            }
                                        });
                                        let deserialized: Result<Project, _> = serde_json::from_value(project_form_data);
                                        // println!("{:#?}", deserialized);
                                        match deserialized {
                                            Ok(project) => {
                                                match add_project(project) {
                                                    Ok(_) => println!("Project added successfully."),
                                                    Err(e) => eprintln!("Error adding project: {}", e),
                                                }
                                            },
                                            Err(e) => {
                                                eprintln!("Failed to deserialize project_form_data: {}", e);
                                            }
                                        }    
                                    }else {}
                                   
                                    show_modal.set(false);
                                },
                                class: "font-medium text-xs bg-[#101010] text-[#FFFFFF] rounded-[3px] px-4 py-1",
                                "Start",
                            }
                        }
                    }
                }       
            }
        }   
    }
}

fn extract_hex(color: &str) -> String {
    match color {
        "bg-red-500" => "#ef4444".to_string(),
        "bg-black" => "#000000".to_string(),
        "bg-zinc-800" => "#27272a".to_string(),
        hex if hex.starts_with('#') => hex.to_string(),
        _ => "#000000".to_string(), // fallback
    }
}

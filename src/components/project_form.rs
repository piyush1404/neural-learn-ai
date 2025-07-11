use chrono::Utc;
use dioxus::prelude::*;
use uuid::Uuid;
use crate::store::project_schema::{ Project, Category};
use crate::store::project::{ add_project, update_project };
use crate::date_format::{get_local_and_utc_iso};
use crate::views::project_details::{AppState, SharedData};
use crate::store::project::ProjectState;

#[derive(PartialEq, Props, Clone)]
pub struct ProjectFormProps {
    show_modal: Signal<bool>,
    project: Option<Project>,
    is_updating: Option<Signal<bool>>
}
use crate::state::tabs::TabContext;

#[component]
pub fn ProjectForm(props: ProjectFormProps) -> Element {
    let mut tab_context: Signal<TabContext> = use_context::<Signal<TabContext>>();
    let mut project_state = use_context::<ProjectState>();
    println!("ProjectForm entered");
    let (_, utc_iso) = get_local_and_utc_iso();

    let mut show_modal = props.show_modal;
    let mut is_updating = props.is_updating;
    let mut show_advanced = use_signal(|| false);

    let mut categories = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().categories.clone() } else { vec![
        Category { id: 0, name: "Background".to_string(), color: "#4C4C4C".to_string(), context_id: 0 },
    ] });


    let mut project_name = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().name.clone() } else { "".to_string() });
    let mut project_name_error=use_signal(|| false);
    let mut platform = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().platform.clone() } else { "Simulation".to_string() });
    // let mut project_type = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().interface.clone() } else { "Image".to_string() });

    let mut description = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().description.clone() } else { "".to_string() });
    let mut description_error=use_signal(|| false);
    let mut normalized = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().normalized } else { false });
    let mut algorithm = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().algorithm.clone() } else { "grayscale".to_string() });

    let mut roi_width = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().roi_width } else { 16 });
    let mut roi_height = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().roi_height } else { 16 });
    let mut block_width = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().block_width } else { 1 });
    let mut block_height = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().block_height } else { 1 });
    let mut range_min = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().if_field_range.min } else { 1 });
    let mut range_max = use_signal(|| if props.project.is_some() { props.project.as_ref().unwrap().feature_extraction.as_ref().unwrap().if_field_range.max } else { 16 });

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
                class: "flex items-center gap-2 px-4 py-2 rounded cursor-pointer hover:bg-[#555555] hover:text-[#FFFFFF] text-[#555555]",
                onclick: move |_| {
                    selected_label.set(label.clone());
                    selected_icon.set(Ok(icon_element.clone()));
                    show_options.set(false);
                    if label == "Audio" {
                        show_modal_feature_extraction_panel.set(true);
                    } else {
                        show_modal_feature_extraction_panel.set(false);
                    }
                },
                {icon_element_for_display}
                span { 
                    class: "font-normal text-xs pl-1",
                    "{label}"
                }
            }
        )
        
        
    })
    .collect();
    let  feature_extraction_panel_class = if show_modal_feature_extraction_panel() {
        "pt-3 w-[56%] opacity-50 pointer-events-none"
    } else {
        "pt-3 w-[56%]"
    };
    let mut open_image_roi = {
        move |id: &str, name: String| {
            println!("Opening image ROI...");
            println!("ID: {}, Project Name: {}", id, name);
            project_state.project_id.set(id.to_string());
            project_state.project_name.set(name.to_string());
            tab_context.write().add_tab(
                &name,
                rsx! { crate::views::project_details::ProjectDetails {} },
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

    let mut is_error=use_signal(|| false);
    let mut error = use_signal(|| "".to_string());

    let mut is_success=use_signal(|| false);
    let mut success = use_signal(|| "".to_string());
    
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
                            // maxlength: "50",
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
                                    class: "absolute w-full bg-white border mt-1 rounded-xl shadow z-10",
                                    {option_list.into_iter()}
                                }
                            }
                        }
                       
                    }

                }
                div {
                    class: "col-span-3 mt-4",
                    div {
                        class: "flex justify-between items-center bg-[#EFEFEF] mb-1 text-xs text-[#404040]",
                        span { "Description" }
                        span { "{description.read().len()}/200 alphabets" }
                    }
                    textarea {
                        class: "w-full border border-[#8F8F8F] rounded font-poppins font-normal px-4 py-1 text-xs text-[#313131] resize-none bg-[#FFFFFF] appearance-none outline-none",
                        maxlength: "200",
                        value: "{description}",
                        oninput: move |e| {
                            let value = e.value().to_string();
                            description_error.set(value.len() > 200);
                            description.set(value);
                        }
                    }
                    if  description_error() {
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
                                class: "relative overflow-y-auto space-y-2 h-[266px] custom-scrollbar",

                                div {  
                                    class: "flex justify-between items-center pr-2",
                                    span { class: "text-xs text-[#404040] font-normal", "Categories" }
                                    button {
                                        class: "text-[#0387D9] font-medium text-sm ",
                                        onclick: move |_| {
                                            categories.set(vec![
                                                Category {
                                                    id: 1,
                                                    name: "Background".to_string(),
                                                    color: "#4C4C4C".to_string(),
                                                    context_id: 101,
                                                },
                                            ]);
                                        },
                                        "Reset"
                                    }

                                }
                        
                                div {
                                    class: "flex items-center gap-2 text-xs text-[#404040] mb-1",
                                    span { class: "w-[91px] text-[10px] h-[15px] rounded", "Name" }
                                    span { class: "w-[28px] text-[10px] h-[15px]", "Color" }
                                }
                        
                                div {  
                                    class: "flex flex-col gap-1",
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
                                            class: "appearance-none w-[28px] h-[20px] p-0 rounded cursor-pointer",
                                            value: category.color.clone(),
                                            disabled: category.name.trim().is_empty(), 
                                            onchange: move |e| {
                                                let mut updated = categories.write().clone();
                                                updated[index].color = e.value().clone();
                                                categories.set(updated);
                                            }
                                        }
                        
                                        div {
                                            if categories.read().len() > 1 && index != categories.read().len() - 1 {
                                                button {
                                                    class: "w-[19px] h-[19px] rounded-sm bordertext-sm flex items-center justify-center",
                                                    onclick: move |_| {
                                                        let mut updated = categories.write().clone();
                                                        updated.remove(index);
                                                        categories.set(updated);
                                                    },
                                                    svg {
                                                        width: "19",
                                                        height: "19",
                                                        view_box: "0 0 19 19",
                                                        fill: "none",
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                
                                                        rect {
                                                            x: "0.5",
                                                            y: "0.5",
                                                            width: "18",
                                                            height: "18",
                                                            rx: "3.5",
                                                            fill: "white",
                                                            stroke: "#E56750",
                                                        }
                                                
                                                        mask {
                                                            id: "path-2-inside-1_355_3774",
                                                            fill: "white",
                                                            path {
                                                                fill_rule: "evenodd",
                                                                clip_rule: "evenodd",
                                                                d: "M5.61046 5.61029C5.34205 5.8787 5.34205 6.31414 5.61046 6.58254L12.4164 13.3885C12.6849 13.657 13.1202 13.6569 13.3886 13.3885C13.657 13.1201 13.6571 12.6847 13.3886 12.4162L6.58281 5.6104C6.3143 5.34189 5.87886 5.34189 5.61046 5.61029Z",
                                                            }
                                                        }
                                                
                                                        path {
                                                            fill_rule: "evenodd",
                                                            clip_rule: "evenodd",
                                                            d: "M5.61046 5.61029C5.34205 5.8787 5.34205 6.31414 5.61046 6.58254L12.4164 13.3885C12.6849 13.657 13.1202 13.6569 13.3886 13.3885C13.657 13.1201 13.6571 12.6847 13.3886 12.4162L6.58281 5.6104C6.3143 5.34189 5.87886 5.34189 5.61046 5.61029Z",
                                                            fill: "#E56750",
                                                        }
                                                
                                                        path {
                                                            d: "M5.61046 5.61029L7.52071 7.52054C8.30731 6.73394 8.30731 5.45888 7.52071 4.67229L5.61046 6.58254L3.70021 8.49279C2.3768 7.16939 2.37679 5.02345 3.70021 3.70004L5.61046 5.61029ZM5.61046 6.58254L7.52071 4.67229L14.3266 11.4782L12.4164 13.3885L10.5061 15.2987L3.70021 8.49279L5.61046 6.58254ZM12.4164 13.3885L14.3266 11.4782C13.5399 10.6915 12.2647 10.6919 11.4784 11.4782L13.3886 13.3885L15.2989 15.2987C13.9757 16.6219 11.8299 16.6225 10.5061 15.2987L12.4164 13.3885ZM13.3886 13.3885L11.4784 11.4782C10.692 12.2646 10.6916 13.5397 11.4784 14.3265L13.3886 12.4162L15.2989 10.506C16.6227 11.8297 16.622 13.9756 15.2989 15.2987L13.3886 13.3885ZM13.3886 12.4162L11.4784 14.3265L4.67256 7.52065L6.58281 5.6104L8.49306 3.70014L15.2989 10.506L13.3886 12.4162ZM6.58281 5.6104L4.67256 7.52065C5.45878 8.30688 6.73384 8.30741 7.52071 7.52054L5.61046 5.61029L3.70021 3.70004C5.02388 2.37636 7.16982 2.37691 8.49306 3.70014L6.58281 5.6104Z",
                                                            fill: "#E56750",
                                                            mask: "url(#path-2-inside-1_355_3774)",
                                                        }
                                                
                                                        mask {
                                                            id: "path-4-inside-2_355_3774",
                                                            fill: "white",
                                                            path {
                                                                fill_rule: "evenodd",
                                                                clip_rule: "evenodd",
                                                                d: "M5.61046 13.3878C5.87886 13.6562 6.3143 13.6562 6.58271 13.3878L13.3886 6.58183C13.6571 6.31332 13.657 5.87798 13.3886 5.60958C13.1202 5.34118 12.6849 5.34107 12.4164 5.60958L5.61056 12.4154C5.34206 12.6839 5.34206 13.1194 5.61046 13.3878Z",
                                                            }
                                                        }
                                                
                                                        path {
                                                            fill_rule: "evenodd",
                                                            clip_rule: "evenodd",
                                                            d: "M5.61046 13.3878C5.87886 13.6562 6.3143 13.6562 6.58271 13.3878L13.3886 6.58183C13.6571 6.31332 13.657 5.87798 13.3886 5.60958C13.1202 5.34118 12.6849 5.34107 12.4164 5.60958L5.61056 12.4154C5.34206 12.6839 5.34206 13.1194 5.61046 13.3878Z",
                                                            fill: "#E56750",
                                                        }
                                                
                                                        path {
                                                            d: "M5.61046 13.3878L7.52071 11.4775C6.73411 10.6909 5.45905 10.6909 4.67245 11.4775L6.58271 13.3878L8.49296 15.298C7.16956 16.6214 5.02362 16.6214 3.70021 15.298L5.61046 13.3878ZM6.58271 13.3878L4.67245 11.4775L11.4784 4.67157L13.3886 6.58183L15.2989 8.49208L8.49296 15.298L6.58271 13.3878ZM13.3886 6.58183L11.4784 4.67157C10.6916 5.45833 10.692 6.73349 11.4784 7.51983L13.3886 5.60958L15.2989 3.69933C16.622 5.02247 16.6227 7.16831 15.2989 8.49208L13.3886 6.58183ZM13.3886 5.60958L11.4784 7.51983C12.2647 8.30617 13.5399 8.30659 14.3266 7.51983L12.4164 5.60958L10.5061 3.69933C11.8299 2.37556 13.9757 2.37618 15.2989 3.69933L13.3886 5.60958ZM12.4164 5.60958L14.3266 7.51983L7.52082 14.3257L5.61056 12.4154L3.70031 10.5052L10.5061 3.69933L12.4164 5.60958ZM5.61056 12.4154L7.52082 14.3257C8.30704 13.5394 8.30758 12.2644 7.52071 11.4775L5.61046 13.3878L3.70021 15.298C2.37653 13.9743 2.37707 11.8284 3.70031 10.5052L5.61056 12.4154Z",
                                                            fill: "#E56750",
                                                            mask: "url(#path-4-inside-2_355_3774)",
                                                        }
                                                    }
                                                
                                                }
                                            }
                                            if index == categories.read().len() - 1 {
                                                button {
                                                    class: "w-[19px] h-[19px] rounded border border-[#0387D9] text-sm flex items-center justify-center",
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
                                                    svg {
                                                        width: "11",
                                                        height: "11",
                                                        view_box: "0 0 11 11",
                                                        fill: "none",
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                
                                                        path {
                                                            d: "M0 5.49998C0 5.87956 0.307902 6.18746 0.687483 6.18746H10.3125C10.6922 6.18746 11 5.87956 11 5.49998C11 5.1204 10.6922 4.8125 10.3125 4.8125H0.687629C0.307903 4.8125 0 5.1204 0 5.49998Z",
                                                            fill: "#0387D9",
                                                            fill_rule: "evenodd",
                                                            clip_rule: "evenodd"
                                                        }
                                                
                                                        path {
                                                            d: "M5.50002 11C5.8796 11 6.1875 10.6921 6.1875 10.3125V0.687482C6.1875 0.307757 5.8796 0 5.50002 0C5.12044 0 4.81254 0.307757 4.81254 0.687482V10.3124C4.81254 10.6921 5.12044 11 5.50002 11Z",
                                                            fill: "#0387D9",
                                                            fill_rule: "evenodd",
                                                            clip_rule: "evenodd"
                                                        }
                                                    }
                                                          
                                                }
                                            }
                                        }
                                    }
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
                                        option { value: "grayscale", "Subsample" }
                                        option { value: "rgb", "Subsample RGB" }
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
                                        max: "16",
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
                                        min: "1",
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
                                    if props.project.is_none(){

                                        // Collect validation errors
                                        if project_name.read().trim().is_empty() {
                                            error.set("Project name cannot be empty".to_string());
                                            println!("error: {}", error.read());
                                            is_error.set(true);
                                            return; // ✅ Prevents further execution
                                        }

                                        if platform.read().trim().is_empty() {
                                            error.set("Platform cannot be empty".to_string());
                                            println!("error: {}", error.read());
                                            is_error.set(true);
                                            return;
                                        }

                                        if selected_label.read().trim().is_empty() {
                                            error.set("Interface cannot be empty".to_string());
                                            println!("error: {}", error.read());
                                            is_error.set(true);
                                            return;
                                        }

                                        if description.read().trim().is_empty() {
                                            error.set("Description cannot be empty".to_string());
                                            println!("error: {}", error.read());
                                            is_error.set(true);
                                            return;
                                        }

                                        let id = Uuid::new_v4().to_string();
                                        println!("Creating in IFnew project with ID: {}", id);
                                        
                                        let project_form_data = serde_json::json!({
                                            "id": id,
                                            "name": project_name.read().to_string(),
                                            "platform": platform.read().to_string(),
                                            "interface": selected_label.read().to_string(),
                                            "type": "vision".to_string(),
                                            "description": description.read().to_string(),
                                            "created_at": utc_iso,
                                            "updated_at": utc_iso,
                                            "categories": categories.read().iter()
                                            .filter(|cat| !cat.name.trim().is_empty()) 
                                            .map(|cat| {
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
                                    // open_image_roi(&id, project_name.read().clone());

                                    }else {
                                        // Collect validation errors
                                        if project_name.read().trim().is_empty() {
                                            error.set("Project name cannot be empty".to_string());
                                            is_error.set(true);
                                            println!("error : {}",error());
                                            return; // ✅ Prevents further execution
                                        }

                                        if platform.read().trim().is_empty() {
                                            error.set("Platform cannot be empty".to_string());
                                            is_error.set(true);
                                            println!("error : {}",error());
                                            return;
                                        }

                                        if selected_label.read().trim().is_empty() {
                                            error.set("Interface cannot be empty".to_string());
                                            is_error.set(true);
                                            println!("error : {}",error());
                                            return;
                                        }

                                        if description.read().trim().is_empty() {
                                            error.set("Description cannot be empty".to_string());
                                            is_error.set(true);
                                            println!("error : {}",error());
                                            return;
                                        }

                                        let id = props.project.as_ref().unwrap().id.clone();
                                        println!("Creating in ELSE new project with ID: {}", id);

                                        use serde_json::{json, Map, Value};
                                        let mut patch_payload = Map::new();

                                        let id_clone = id.clone();
                                        patch_payload.insert("id".to_string(), Value::String(id_clone));

                                        let name = project_name.read().to_string();
                                        if !name.trim().is_empty() {
                                            patch_payload.insert("name".to_string(), Value::String(name));
                                        }

                                        let platform = platform.read().to_string();
                                        if !platform.trim().is_empty() {
                                            patch_payload.insert("platform".to_string(), Value::String(platform));
                                        }

                                        let interface = selected_label.read().to_string();
                                        if !interface.trim().is_empty() {
                                            patch_payload.insert("interface".to_string(), Value::String(interface));
                                        }

                                        let project_type = props.project.as_ref().unwrap().r#type.clone();
                                        if !project_type.trim().is_empty() {
                                            patch_payload.insert("type".to_string(), Value::String(project_type));
                                        }

                                        let description = description.read().to_string();
                                        if !description.trim().is_empty() {
                                            patch_payload.insert("description".to_string(), Value::String(description));
                                        }

                                        let neurons = props.project.as_ref().unwrap().neurons.clone();
                                        if !neurons.is_none() {
                                            patch_payload.insert("neurons".to_string(), Value::String(format!("{:?}", neurons.unwrap())));
                                        }

                                        let categories = categories.read().iter()
                                        .filter(|cat| !cat.name.trim().is_empty()) 
                                        .map(|cat| {
                                            json!({
                                                "id": cat.id,
                                                "name": cat.name,
                                                "color": cat.color,
                                                "context_id": cat.context_id
                                            })
                                        })
                                        .collect::<Vec<_>>();
                                        if !categories.is_empty() {
                                            patch_payload.insert("categories".to_string(), Value::Array(categories));
                                        }

                                        let feature_extraction = json!({
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
                                        });
                                        if !feature_extraction.is_null() {
                                            patch_payload.insert("feature_extraction".to_string(), feature_extraction);
                                        }
                                         if let Some(created_at) = props.project.as_ref().unwrap().created_at.clone().map(|s| s.trim().to_string()) {
                                            if !created_at.is_empty() {
                                                patch_payload.insert("created_at".to_string(), Value::String(created_at));
                                            }
                                        }
                                        let utc_iso = Utc::now().to_rfc3339();
                                        patch_payload.insert("updated_at".to_string(), Value::String(utc_iso));

                                        // println!("{:#?}", patch_payload);

                                        let deserialized: Result<Project, _> = serde_json::from_value(Value::Object(patch_payload));
                                        // println!("{:#?}", deserialized);
                                        match deserialized {
                                            Ok(project) => {
                                                match update_project(&id, project) {
                                                    Ok(_) => println!("Project updated successfully."),
                                                    Err(e) => eprintln!("Error updating project: {}", e),
                                                }
                                            },
                                            Err(e) => {
                                                eprintln!("Failed to deserialize project_form_data: {}", e);
                                            }
                                        }
                                        // open_image_roi(&id, project_name.read().clone());

                                    }
                                    show_modal.set(false);
                                    if let Some(mut is_updating_signal) = is_updating {
                                        is_updating_signal.set(true);
                                    }
                                    // println!("id: {}", id);
                                    // println!("props.project is: {:?}", props.project);
                                    // println!("name: {}", project_name.read());
                                    // open_image_roi("1", project_name.read().clone());

                                },
                                class: "font-medium text-xs bg-[#101010] text-[#FFFFFF] rounded-[3px] px-4 py-1",
                                {
                                    if props.project.is_none() {
                                        rsx!("Start")
                                    } else {
                                        rsx!("Update")
                                    }
                                }
                            }
                        }
                    }

                    if *is_error.read() {
                        // AlertCard {
                        //     title: "Error".to_string(),
                        //     icon_svg: rsx! {
                        //         svg {
                        //             width: "232",
                        //             height: "76",
                        //             view_box: "0 0 232 76",
                        //             fill: "none",
                        //             xmlns: "http://www.w3.org/2000/svg",
                        //             // Paste simplified or full path here
                        //             path { d:"M89.6747 39.3403C89.6747 41.1911 91.175 42.6914 93.0257 42.6914H115.94C117.791 42.6914 119.291 44.1917 119.291 46.0425V46.1714C119.291 48.0221 117.791 49.5225 115.94 49.5225H64.2308C62.3801 49.5225 60.8798 48.0221 60.8798 46.1714V46.0454C60.8798 44.1947 59.3794 42.6943 57.5287 42.6943H34.6136C32.7629 42.6943 31.2626 41.194 31.2626 39.3433V34.9448C31.2626 33.0941 32.7629 31.5938 34.6136 31.5938H86.3236C88.1744 31.5938 89.6747 33.0941 89.6747 34.9448V39.3403ZM183.462 35.9273C183.462 37.778 181.961 39.2783 180.111 39.2783H128.401C126.55 39.2783 125.05 37.778 125.05 35.9273V35.7983C125.05 33.9476 126.55 32.4473 128.401 32.4473H180.111C181.961 32.4473 183.462 33.9476 183.462 35.7983V35.9273ZM183.462 17.9946C183.462 19.8454 184.962 21.3457 186.813 21.3457H194.919C196.77 21.3457 198.27 22.846 198.27 24.6968V29.0952C198.27 30.946 196.77 32.4463 194.919 32.4463H143.209C141.359 32.4463 139.858 30.946 139.858 29.0952V24.6968C139.858 22.846 138.358 21.3457 136.507 21.3457H128.401C126.55 21.3457 125.05 22.846 125.05 24.6968V28.2417C125.05 30.0925 123.549 31.5928 121.699 31.5928H69.9896C68.1389 31.5928 66.6385 30.0925 66.6385 28.2417V23.8433C66.6385 21.9925 68.1389 20.4922 69.9896 20.4922H121.699C123.549 20.4922 125.05 18.9919 125.05 17.1411V13.5952C125.05 11.7445 126.55 10.2441 128.401 10.2441H180.111C181.961 10.2441 183.462 11.7445 183.462 13.5952V17.9946Z" , fill:"#FFF2D2"}
                        //             path { d:"M73.2197 65.8179C73.2197 67.6686 71.7194 69.1689 69.8687 69.1689H61.7622C59.9115 69.1689 58.4111 70.6693 58.4111 72.52V72.648C58.4111 74.4987 56.9108 75.999 55.0601 75.999H3.35106C1.50032 75.999 0 74.4987 0 72.648V72.519C0 70.6683 1.50032 69.168 3.35106 69.168H11.4575C13.3083 69.168 14.8086 67.6676 14.8086 65.8169V61.4194C14.8086 59.5687 16.3089 58.0684 18.1597 58.0684H69.8687C71.7194 58.0684 73.2197 59.5687 73.2197 61.4194V65.8179ZM58.4111 54.7163C58.4111 56.5671 56.9108 58.0674 55.0601 58.0674H3.35106C1.50032 58.0674 0 56.5671 0 54.7163V50.3179C0 48.4671 1.50032 46.9668 3.35106 46.9668H55.0601C56.9108 46.9668 58.4111 48.4671 58.4111 50.3179V54.7163Z" , fill:"#FFF2D2"}
                        //             path { d:"M184.833 63.1904C184.833 64.1337 185.597 64.8984 186.541 64.8984H220.816C221.995 64.8984 222.951 65.8542 222.951 67.0332C222.951 68.2122 221.995 69.168 220.816 69.168H183.578C183.514 69.168 183.462 69.1159 183.462 69.0518C183.462 68.9876 183.41 68.9355 183.345 68.9355H163.503C161.652 68.9355 160.152 70.4359 160.152 72.2866V72.4146C160.152 74.2653 158.652 75.7656 156.801 75.7656H124.287C122.436 75.7656 120.936 74.2653 120.936 72.4146V71.6636C120.936 69.8128 122.436 68.3125 124.287 68.3125H142.266C144.116 68.3125 145.617 66.8122 145.617 64.9614V64.8335C145.617 62.9827 147.117 61.4824 148.968 61.4824H183.125C184.068 61.4824 184.833 62.2471 184.833 63.1904Z" , fill:"#FFF2D2"}
                        //             path { d:"M161.149 50.3809C162.416 50.3809 163.442 51.4075 163.442 52.6738C163.442 53.9402 162.416 54.9668 161.149 54.9668H141.289C139.439 54.9668 137.938 56.4671 137.938 58.3179V58.7544C137.938 60.6051 136.438 62.1055 134.587 62.1055H102.075C100.224 62.1055 98.7235 60.6051 98.7235 58.7544V58.0034C98.7235 56.1527 100.224 54.6523 102.075 54.6523H122.092C123.271 54.6523 124.227 53.6961 124.227 52.5166C124.227 51.3371 125.184 50.3809 126.363 50.3809H161.149Z" , fill:"#FFF2D2"}
                        //             path { d:"M102.896 4.26758C104.075 4.26758 105.031 5.22335 105.031 6.40234C105.031 7.58134 105.986 8.53711 107.165 8.53711H117.311C119.162 8.53711 120.662 10.0374 120.662 11.8882V12.6392C120.662 14.4899 119.162 15.9902 117.311 15.9902H84.7986C82.9479 15.9902 81.4475 14.4899 81.4475 12.6392V12.2056C81.4475 10.3548 79.9472 8.85449 78.0965 8.85449H68.1092C66.8425 8.85449 65.8157 7.82768 65.8157 6.56104C65.8157 5.29439 66.8425 4.26758 68.1092 4.26758H102.896Z" , fill:"#FFF2D2"}
                        //             path { d:"M226.037 40.8228C226.037 42.6735 224.536 44.1738 222.686 44.1738H219.392C217.571 44.1738 216.094 45.6503 216.094 47.4717C216.094 49.293 217.571 50.7695 219.392 50.7695H223.744C225.01 50.7695 226.037 51.7961 226.037 53.0625C226.037 54.3289 225.01 55.3555 223.744 55.3555H189.112C187.847 55.3555 186.821 54.3297 186.821 53.0645C186.821 51.7992 185.795 50.7734 184.53 50.7734H180.231C178.38 50.7734 176.88 49.2731 176.88 47.4224V46.6714C176.88 44.8206 178.38 43.3203 180.231 43.3203H183.521C185.344 43.3203 186.821 41.8429 186.821 40.0205C186.821 38.1981 188.298 36.7207 190.121 36.7207H222.686C224.536 36.7207 226.037 38.221 226.037 40.0718V40.8228Z" , fill:"#FFF2D2"}
                        //             path { d:"M61.7024 23.3916C63.323 23.3917 64.6575 24.7595 64.6575 26.4727C64.6573 28.1857 63.3229 29.5526 61.7024 29.5527C60.0818 29.5527 58.7465 28.1858 58.7463 26.4727C58.7463 24.7594 60.0817 23.3916 61.7024 23.3916Z" , fill:"#FFF2D2" ,stroke:"#DFC685" ,stroke_width:"0.670213"}
                        //             path { d:"M228.71 43.8857C230.33 43.8859 231.665 45.2536 231.665 46.9668C231.665 48.6798 230.33 50.0467 228.71 50.0469C227.089 50.0469 225.754 48.6799 225.754 46.9668C225.754 45.2535 227.089 43.8857 228.71 43.8857Z" , fill:"#FFF2D2" ,stroke:"#DFC685" ,stroke_width:"0.670213"}
                        //             path { d:"M9.04993 60.9658C10.6705 60.966 12.005 62.3337 12.005 64.0469C12.0048 65.7599 10.6704 67.1268 9.04993 67.127C7.42931 67.127 6.09403 65.76 6.09387 64.0469C6.09387 62.3336 7.42921 60.9658 9.04993 60.9658Z" , fill:"#FFF2D2" ,stroke:"#DFC685" ,stroke_width:"0.670213"}
                        //             path { d:"M188.397 13.1436C190.018 13.1437 191.353 14.5114 191.353 16.2246C191.352 17.9376 190.018 19.3046 188.397 19.3047C186.777 19.3047 185.442 17.9377 185.441 16.2246C185.441 14.5114 186.777 13.1436 188.397 13.1436Z" , fill:"#FFF2D2" ,stroke:"#DFC685" ,stroke_width:"0.670213"}
                        //             path { d:"M114.355 68.6494C115.976 68.6495 117.31 70.0173 117.31 71.7305C117.31 73.4435 115.975 74.8104 114.355 74.8105C112.734 74.8105 111.399 73.4436 111.399 71.7305C111.399 70.0172 112.734 68.6494 114.355 68.6494Z" , fill:"#FFF2D2", stroke:"#DFC685" ,stroke_width:"0.670213"}
                        //             path { d:"M170.298 47.2998C171.919 47.2999 173.253 48.6677 173.253 50.3809C173.253 52.0939 171.919 53.4608 170.298 53.4609C168.678 53.4609 167.342 52.094 167.342 50.3809C167.342 48.6676 168.678 47.2998 170.298 47.2998Z" , fill:"#FFF2D2" ,stroke:"#DFC685" ,stroke_width:"0.670213"}
                        //             path { d:"M110.241 0.334961C111.862 0.335092 113.197 1.70285 113.197 3.41602C113.196 5.12904 111.862 6.49596 110.241 6.49609C108.621 6.49609 107.286 5.12912 107.285 3.41602C107.285 1.70277 108.621 0.334961 110.241 0.334961Z" , fill:"#FFF2D2" ,stroke:"#DFC685" ,stroke_width:"0.670213"}
                        //             path { d:"M116 45.6953C114.881 45.6953 113.944 46.6327 113.944 47.751C113.944 48.8692 114.881 49.8066 116 49.8066C117.077 49.8066 118.055 48.8692 118.006 47.8003C118.055 46.6245 117.126 45.6953 116 45.6953Z" , fill:"#DFB240"}
                        //             path { d:"M135.027 52.9809C136.318 50.7526 136.326 48.0967 135.043 45.8766L122.166 23.5768C120.892 21.3321 118.59 20 116.008 20C113.426 20 111.123 21.3403 109.849 23.5686L96.9559 45.893C95.6732 48.1378 95.6814 50.8102 96.9806 53.0385C98.2633 55.2421 100.557 56.566 103.123 56.566H128.843C131.417 56.566 133.727 55.2257 135.027 52.9809ZM132.231 51.3693C131.516 52.6027 130.249 53.3345 128.835 53.3345H103.115C101.717 53.3345 100.459 52.6191 99.7598 51.4104C99.0527 50.1852 99.0445 48.7216 99.7516 47.4882L112.645 25.172C113.344 23.9469 114.593 23.2233 116.008 23.2233C117.414 23.2233 118.672 23.9551 119.371 25.1803L132.256 47.4964C132.946 48.6969 132.938 50.1441 132.231 51.3693Z" , fill:"#DFB240"}
                        //             path { d:"M115.49 31.2651C114.511 31.5446 113.903 32.4327 113.903 33.5098C113.952 34.1594 113.993 34.8172 114.043 35.4668C114.182 37.9418 114.322 40.3675 114.462 42.8425C114.511 43.6812 115.161 44.2897 116 44.2897C116.838 44.2897 117.496 43.6401 117.537 42.7932C117.537 42.2834 117.537 41.8147 117.586 41.2967C117.677 39.7097 117.776 38.1227 117.866 36.5358C117.915 35.5079 118.006 34.4801 118.055 33.4523C118.055 33.0823 118.006 32.7534 117.866 32.4245C117.447 31.5035 116.468 31.0348 115.49 31.2651Z" ,fill:"#DFB240"}
        
                        //         }
                        //     },
                        //     message_title: "Something went wrong".to_string(),
                        //     message_body: error.read().clone(),
                        //     confirm_text: Some("OK".to_string()),
                        //     cancel_text: None,
                        //     color: "red".to_string(),
                        //     on_close: EventHandler::new(move |_| {
                        //         is_error.set(false);
                        //         error.set("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s.".to_string());
                        //     }),
                            
                        //     on_confirm: Some(EventHandler::new(move |_| {
                        //         is_error.set(false);
                        //         error.set("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum".to_string());
                        //     })),
                            
                        // }

                        // ErrorModal {  
                        //     visible: is_error,
                        //     title: "Something went wrong".to_string(),
                        //     message: error.read().clone(),
                        //     item_type: "red".to_string(),

                        // }
                    }                    
                   
                }       
            }
        }   
    }
}

use dioxus::prelude::*;
use std::fs;

use crate::components::new_project_card::NewProjectCard;
use crate::components::project_card::ProjectCard;

use crate::store::project::{
    add_project, delete_project, get_projects_by_name, load_projects, update_project, 
};
use crate::store::project_schema::Project;

const PAGE_SIZE: usize = 7;

#[derive(Clone, PartialEq)]
struct Tab {
    id: usize,
    title: String,
    active: bool,
}

#[component]
pub fn HomePage() -> Element {
    let projects = use_signal(|| load_projects());

    let mut current_page = use_signal(|| 0);

    let total_pages = (projects().len() + PAGE_SIZE - 1) / PAGE_SIZE;

    // Pagination handlers
    let go_next = {
        to_owned![current_page, total_pages];
        move |_| {
            if current_page() + 1 < total_pages {
                current_page.set(current_page() + 1);
            }
        }
    };

    let go_prev = {
        to_owned![current_page];
        move |_| {
            if current_page() > 0 {
                current_page.set(current_page() - 1);
            }
        }
    };

    // Calculate visible projects for current page
    let visible_projects = {
        let start = current_page() * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(projects().len());
        projects()[start..end].to_vec()
    };

    // println!("Visible projects: {:#?}", visible_projects);

    // Tab management logic (unchanged)
    // let mut tabs = use_signal(|| {
    //     vec![Tab {
    //         id: 1,
    //         title: "Home".to_string(),
    //         active: true,
    //     }]
    // });
    // let next_id = use_signal(|| 2);

    // let add_tab = {
    //     to_owned![tabs, next_id];
    //     move |_| {
    //         tabs.write().iter_mut().for_each(|t| t.active = false);
    //         tabs.write().push(Tab {
    //             id: next_id(),
    //             title: format!("Tab {}", next_id()),
    //             active: true,
    //         });
    //         next_id.set(next_id() + 1);
    //     }
    // };

    // let mut activate_tab = {
    //     to_owned![tabs];
    //     move |id| {
    //         tabs.write().iter_mut().for_each(|t| t.active = t.id == id);
    //     }
    // };

    // let mut close_tab = {
    //     to_owned![tabs];
    //     move |id| {
    //         let mut new_list: Vec<Tab> = tabs().into_iter().filter(|t| t.id != id).collect();
    //         if !new_list.iter().any(|t| t.active) {
    //             if let Some(first) = new_list.first_mut() {
    //                 first.active = true;
    //             }
    //         }
    //         tabs.set(new_list);
    //     }
    // };

    // let rendered_tabs = tabs()
    //     .clone()
    //     .into_iter()
    //     .map(|tab| {
    //         let (bg_class, border_class, text_class, z_class) = if tab.active {
    //             (
    //                 "bg-[#e0e0e0] hover:bg-[#d0d0d0]",
    //                 "border-x border-t border-gray-300",
    //                 "text-black",
    //                 "z-10",
    //             )
    //         } else {
    //             ("bg-white", "border border-transparent", "text-gray-600", "z-0")
    //         };

    //         let class_name = format!(
    //             "relative px-4 py-1 flex items-center gap-2 text-sm rounded-t-md cursor-pointer min-w-[100px] flex-shrink-0 {bg_class} {border_class} {text_class} {z_class} -mb-px select-none"
    //         );

    //         rsx! {
    //             div {
    //                 key: "{tab.id}",
    //                 class: "{class_name}",
    //                 onclick: move |_| activate_tab(tab.id),
    //                 "{tab.title}",
    //                 button {
    //                     class: "ml-2 text-gray-800 hover:text-red-500",
    //                     onclick: move |evt| {
    //                         evt.stop_propagation();
    //                         close_tab(tab.id);
    //                     },
    //                     "×"
    //                 }
    //             }
    //         }
    //     })
    //     .collect::<Vec<_>>();

    // test_add_project();
    // test_get_projects_by_name();
    // test_update_project();
    // test_delete_project();

    let page_info = format!("{:02}/{:02}", current_page() + 1, total_pages);

    rsx! {
        // div {
        //     class: "relative border-b border-gray-300 flex items-end space-x-1",
        //     div {
        //         class: "flex overflow-x-auto whitespace-nowrap no-scrollbar",
        //         {rendered_tabs.into_iter()}
        //     }
        //     button {
        //         class: "w-6 h-6 mb-[1px] bg-gray-200 hover:bg-gray-300 rounded-full text-gray-700 text-lg flex items-center justify-center flex-shrink-0",
        //         onclick: add_tab,
        //         "+"
        //     }
        // }
        div {
            class: "flex flex-col w-full border border-[#BEBEBE] rounded-lg mt-1",
            div {
                class: "h-[50px] flex items-center gap-4 p-4 border-b",
                span { class: "text-[#151515] font-light text-xs", "Projects" }

                div {
                    class:"w-full flex items-center justify-end gap-4",
                    div {
                        class: "flex items-center gap-4",
                        label {
                            class: "flex items-center gap-2 cursor-pointer",
                            input { r#type: "radio", name: "filter", value: "all", checked: true,
                            class: "w-[20px] h-[20px] border-2 rounded-full focus:outline-none" }
                            span { class: "text-[#151515] text-xs font-light", "All" }
                        }
                        label {
                            class: "flex items-center gap-2 cursor-pointer",
                            input { r#type: "radio", name: "filter", value: "complete", class: "w-[20px] h-[20px] border-2 rounded-full focus:outline-none"}
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

                            span { class: "text-[#151515] text-xs font-light", "Complete" }
                        }
                        label {
                            class: "flex items-center gap-2 cursor-pointer",
                            input { r#type: "radio", name: "filter", value: "incomplete", class: "w-[20px] h-[20px] border-2 rounded-full focus:outline-none" }
                            svg {
                                width: "18",
                                height: "18",
                                view_box: "0 0 18 18",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "M18 8.39686C18 9.60162 17.2254 10.6281 16.1311 11.0069C16.2644 11.3336 16.3346 11.6849 16.3346 12.0406C16.3346 13.3836 15.3596 14.5062 14.0712 14.7557C13.9759 16.1965 12.7586 17.3398 11.2759 17.3398C10.3391 17.3398 9.50889 16.883 9.00002 16.1836C8.49115 16.8831 7.66088 17.3398 6.72414 17.3398C5.24144 17.3398 4.02409 16.1965 3.92884 14.7557C2.64041 14.5062 1.66535 13.3836 1.66535 12.0406C1.66535 11.6849 1.73562 11.3336 1.86887 11.0069C0.7746 10.6282 0 9.60162 0 8.39686C0 7.26269 0.696943 6.27183 1.70972 5.84889C1.55965 5.50508 1.4803 5.13186 1.4803 4.75307C1.4803 4.0126 1.78154 3.31481 2.32848 2.78831C2.76353 2.36956 3.30857 2.10256 3.89513 2.01505C4.05118 1.50872 4.34936 1.05164 4.75904 0.699104C5.28275 0.248468 5.95412 0.000297546 6.64948 0.000297546C7.6194 0.000297546 8.47816 0.476902 8.99995 1.20488C9.52173 0.476902 10.3805 0.000297546 11.3504 0.000297546C12.0457 0.000297546 12.7171 0.248468 13.2408 0.699104C13.6505 1.05164 13.9487 1.50875 14.1048 2.01505C14.6913 2.10256 15.2364 2.36956 15.6714 2.78831C16.2184 3.31481 16.5196 4.0126 16.5196 4.75307C16.5196 5.13189 16.4402 5.50508 16.2902 5.84889C17.3031 6.27187 18 7.26273 18 8.39686ZM6.64951 1.0559C5.74796 1.0559 4.97223 1.71417 4.845 2.58712C4.845 2.58719 4.845 2.58726 4.84496 2.58729C4.83275 2.67111 4.82656 2.75703 4.82656 2.84264C4.82656 3.82784 5.64433 4.62935 6.64951 4.62935C6.941 4.62935 7.17731 4.86567 7.17731 5.15715C7.17731 5.44864 6.941 5.68495 6.64951 5.68495C5.15774 5.68495 3.92761 4.55859 3.78503 3.12196C3.06962 3.3468 2.5359 4.00278 2.5359 4.75307C2.5359 5.15318 2.67243 5.52788 2.93077 5.83665C3.05505 5.98524 3.08798 6.18999 3.01652 6.37004C2.94506 6.55009 2.78066 6.67652 2.5883 6.69942C1.71451 6.80336 1.0556 7.5331 1.0556 8.39686C1.0556 9.31439 1.78777 10.0649 2.72239 10.1055C2.92229 10.1142 3.10005 10.2351 3.18155 10.4178C3.263 10.6006 3.23418 10.8137 3.10706 10.9682C2.85445 11.2752 2.72092 11.646 2.72092 12.0406C2.72092 12.8481 3.29576 13.5263 4.06603 13.7052C4.43162 12.6375 5.45848 11.8826 6.63684 11.8826C6.92833 11.8826 7.16464 12.1189 7.16464 12.4104C7.16464 12.7019 6.92833 12.9382 6.63684 12.9382C5.807 12.9382 5.10042 13.5449 4.9932 14.3493C4.98339 14.423 4.97839 14.4985 4.97839 14.5736C4.97839 15.5169 5.7615 16.2842 6.72407 16.2842C7.70979 16.2854 8.49291 15.4836 8.47243 14.521V2.85419C8.47236 2.85028 8.47183 2.84651 8.47183 2.84261C8.47183 2.83881 8.47211 2.83508 8.47211 2.83128C8.46592 1.85129 7.65082 1.0559 6.64951 1.0559ZM15.4117 6.69942C15.2193 6.67655 15.0549 6.55009 14.9834 6.37004C14.912 6.18999 14.9449 5.98524 15.0692 5.83665C15.3275 5.52788 15.4641 5.15318 15.4641 4.75307C15.4641 4.00278 14.9303 3.34676 14.215 3.12196C14.0724 4.55859 12.8422 5.68495 11.3505 5.68495C11.059 5.68495 10.8227 5.44864 10.8227 5.15715C10.8227 4.86567 11.059 4.62935 11.3505 4.62935C12.3556 4.62935 13.1734 3.82784 13.1734 2.84264C13.1734 2.7571 13.1672 2.67121 13.155 2.58729C13.155 2.58722 13.155 2.58715 13.155 2.58708C13.0277 1.71417 12.252 1.0559 11.3505 1.0559C10.3491 1.0559 9.53408 1.85133 9.52782 2.83131C9.52782 2.83511 9.5281 2.83884 9.5281 2.84264V14.5331C9.51209 15.4913 10.2956 16.2852 11.2759 16.2843C12.2385 16.2843 13.0215 15.5169 13.0215 14.5736C13.0215 14.4986 13.0165 14.4231 13.0067 14.3494C12.8995 13.5449 12.1929 12.9382 11.3631 12.9382C11.0716 12.9382 10.8353 12.7019 10.8353 12.4104C10.8353 12.1189 11.0716 11.8826 11.3631 11.8826C12.5415 11.8826 13.5683 12.6375 13.9339 13.7052C14.7041 13.5263 15.279 12.8482 15.279 12.0407C15.279 11.646 15.1455 11.2752 14.8929 10.9682C14.7658 10.8137 14.737 10.6006 14.8184 10.4179C14.8999 10.2352 15.0777 10.1142 15.2776 10.1055C16.2122 10.065 16.9444 9.31443 16.9444 8.3969C16.9444 7.5331 16.2855 6.80336 15.4117 6.69942Z",
                                    fill: "#FF7700",
                                }
                            }

                            span { class: "text-[#151515] text-xs font-light", "Incomplete" }
                        }
                    }

                    select {
                        class: "ml-4 px-3 py-1 border border-[#EDEDED] rounded-[50px] bg-[#F7F7F7] font-normal text-xs text-[#555555] shadow-sm appearance-none pr-8",
                        style: r#"
                        background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%230387D9'/%3E%3C/svg%3E");
                        background-repeat: no-repeat;
                        background-position: right 0.75rem center;
                        background-size: 10px 6px;
                    "#,
                        option { value: "", disabled: true, selected: true, "Select Platform :" }
                        option { value: "web", "Web" }
                        option { value: "mobile", "Mobile" }
                        option { value: "desktop", "Desktop" }
                    }

                    div {
                        class: "flex items-center border border-[#EDEDED] rounded-[50px] px-3 py-1 bg-[#F7F7F7] shadow-sm",
                        svg {
                            width: "12",
                            height: "12",
                            view_box: "0 0 12 12",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            path {
                                d: "M11.8864 11.3471L8.78823 8.29845C9.59954 7.41698 10.098 6.25121 10.098 4.96841C10.0977 2.22426 7.83738 0 5.04897 0C2.26057 0 0.000289917 2.22426 0.000289917 4.96841C0.000289917 7.71256 2.26057 9.93682 5.04897 9.93682C6.25376 9.93682 7.35876 9.5201 8.22672 8.82731L11.3369 11.888C11.4885 12.0373 11.7345 12.0373 11.8861 11.888C12.038 11.7387 12.038 11.4964 11.8864 11.3471ZM5.04897 9.1724C2.68968 9.1724 0.777101 7.29021 0.777101 4.96841C0.777101 2.6466 2.68968 0.764419 5.04897 0.764419C7.40829 0.764419 9.32084 2.6466 9.32084 4.96841C9.32084 7.29021 7.40829 9.1724 5.04897 9.1724Z",
                                fill: "#0387D9",
                            }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Search",
                            class: "ml-2 outline-none border-none bg-transparent font-normal text-xs text-[#555555]"
                        }
                    }
                }
            }
            div {
                class: "h-full grid grid-cols-4 gap-4 p-4",
                NewProjectCard {},
               {
                visible_projects.into_iter().map(|project| {
                    rsx! {
                        ProjectCard {
                            id: project.id,
                            name: project.name.clone(),
                            platform: project.platform.clone(),
                            interface: project.interface.clone(),
                            description: project.description,
                            created_at: project.created_at.unwrap_or("Unknown".to_string()),
                            updated_at: project.updated_at.unwrap_or("Unknown".to_string()),
                            neurons: project.neurons.clone().map(|n| n)
                        }
                    }
                })
            }

            }
            div {
                class: "flex justify-end items-center gap-2 bg-white px-4 py-2 border-[#BEBEBE] rounded-lg",

                // First Page (⏪)
                button {
                    class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                    onclick: move |_| current_page.set(0),
                    disabled: "{current_page() == 0}",
                    svg {
                        width: "6",
                        height: "10",
                        view_box: "0 0 6 10",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M4.68257 9.07549C4.80814 9.07622 4.93107 9.03954 5.03571 8.97012C5.14035 8.90071 5.22194 8.80171 5.27009 8.68574C5.31824 8.56977 5.33077 8.44209 5.30607 8.31898C5.28137 8.19586 5.22057 8.08289 5.13141 7.99447L1.78089 4.65027L5.13141 1.30607C5.23498 1.18514 5.2891 1.02957 5.28295 0.870469C5.27681 0.711366 5.21085 0.560442 5.09826 0.447855C4.98568 0.335268 4.83475 0.269312 4.67565 0.263167C4.51655 0.257021 4.36098 0.311139 4.24005 0.414706L0.447006 4.20775C0.329263 4.32619 0.263174 4.48642 0.263174 4.65343C0.263174 4.82044 0.329263 4.98067 0.447006 5.09911L4.24005 8.89215C4.3578 9.00895 4.51673 9.07479 4.68257 9.07549Z",
                            fill: "#CACACA",
                        }
                    }

                    svg {
                        width: "6",
                        height: "10",
                        view_box: "0 0 6 10",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M4.68257 9.07549C4.80814 9.07622 4.93107 9.03954 5.03571 8.97012C5.14035 8.90071 5.22194 8.80171 5.27009 8.68574C5.31824 8.56977 5.33077 8.44209 5.30607 8.31898C5.28137 8.19586 5.22057 8.08289 5.13141 7.99447L1.78089 4.65027L5.13141 1.30607C5.23498 1.18514 5.2891 1.02957 5.28295 0.870469C5.27681 0.711366 5.21085 0.560442 5.09826 0.447855C4.98568 0.335268 4.83475 0.269312 4.67565 0.263167C4.51655 0.257021 4.36098 0.311139 4.24005 0.414706L0.447006 4.20775C0.329263 4.32619 0.263174 4.48642 0.263174 4.65343C0.263174 4.82044 0.329263 4.98067 0.447006 5.09911L4.24005 8.89215C4.3578 9.00895 4.51673 9.07479 4.68257 9.07549Z",
                            fill: "#CACACA",
                        }
                    }
                }

                // Previous Page (◀)
                button {
                    class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                    onclick: go_prev,
                    disabled: "{current_page() == 0}",
                    svg {
                        width: "6",
                        height: "10",
                        view_box: "0 0 6 10",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M4.68257 9.07549C4.80814 9.07622 4.93107 9.03954 5.03571 8.97012C5.14035 8.90071 5.22194 8.80171 5.27009 8.68574C5.31824 8.56977 5.33077 8.44209 5.30607 8.31898C5.28137 8.19586 5.22057 8.08289 5.13141 7.99447L1.78089 4.65027L5.13141 1.30607C5.23498 1.18514 5.2891 1.02957 5.28295 0.870469C5.27681 0.711366 5.21085 0.560442 5.09826 0.447855C4.98568 0.335268 4.83475 0.269312 4.67565 0.263167C4.51655 0.257021 4.36098 0.311139 4.24005 0.414706L0.447006 4.20775C0.329263 4.32619 0.263174 4.48642 0.263174 4.65343C0.263174 4.82044 0.329263 4.98067 0.447006 5.09911L4.24005 8.89215C4.3578 9.00895 4.51673 9.07479 4.68257 9.07549Z",
                            fill: "#CACACA",
                        }
                    }
                }

                // Next Page (▶)
                button {
                    class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                    onclick: go_next,
                    disabled: "{current_page() + 1 >= total_pages}",
                    svg {
                        width: "6",
                        height: "10",
                        view_box: "0 0 6 10",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M0.69808 9.21005C0.570598 9.2108 0.445782 9.17356 0.339548 9.10308C0.233313 9.03261 0.150471 8.9321 0.101585 8.81436C0.0526993 8.69662 0.0399837 8.56699 0.0650597 8.44199C0.0901356 8.317 0.151867 8.2023 0.242383 8.11253L3.64406 4.71727L0.242383 1.322C0.137235 1.19922 0.0822911 1.04128 0.0885304 0.87975C0.0947697 0.718217 0.161733 0.564988 0.276039 0.450682C0.390344 0.336376 0.543573 0.269413 0.705106 0.263174C0.866638 0.256935 1.02458 0.311879 1.14736 0.417027L4.99832 4.26799C5.11786 4.38824 5.18496 4.55091 5.18496 4.72048C5.18496 4.89004 5.11786 5.05271 4.99832 5.17296L1.14736 9.02392C1.02781 9.1425 0.866457 9.20934 0.69808 9.21005Z",
                            fill: "#CACACA",
                        }
                    }

                }

                // Last Page (⏩)
                button {
                    class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                    onclick: move |_| current_page.set(total_pages.saturating_sub(1)),
                    disabled: "{current_page() + 1 >= total_pages}",
                    svg {
                        width: "6",
                        height: "10",
                        view_box: "0 0 6 10",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M0.69808 9.21005C0.570598 9.2108 0.445782 9.17356 0.339548 9.10308C0.233313 9.03261 0.150471 8.9321 0.101585 8.81436C0.0526993 8.69662 0.0399837 8.56699 0.0650597 8.44199C0.0901356 8.317 0.151867 8.2023 0.242383 8.11253L3.64406 4.71727L0.242383 1.322C0.137235 1.19922 0.0822911 1.04128 0.0885304 0.87975C0.0947697 0.718217 0.161733 0.564988 0.276039 0.450682C0.390344 0.336376 0.543573 0.269413 0.705106 0.263174C0.866638 0.256935 1.02458 0.311879 1.14736 0.417027L4.99832 4.26799C5.11786 4.38824 5.18496 4.55091 5.18496 4.72048C5.18496 4.89004 5.11786 5.05271 4.99832 5.17296L1.14736 9.02392C1.02781 9.1425 0.866457 9.20934 0.69808 9.21005Z",
                            fill: "#CACACA",
                        }
                    }
                    svg {
                        width: "6",
                        height: "10",
                        view_box: "0 0 6 10",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M0.69808 9.21005C0.570598 9.2108 0.445782 9.17356 0.339548 9.10308C0.233313 9.03261 0.150471 8.9321 0.101585 8.81436C0.0526993 8.69662 0.0399837 8.56699 0.0650597 8.44199C0.0901356 8.317 0.151867 8.2023 0.242383 8.11253L3.64406 4.71727L0.242383 1.322C0.137235 1.19922 0.0822911 1.04128 0.0885304 0.87975C0.0947697 0.718217 0.161733 0.564988 0.276039 0.450682C0.390344 0.336376 0.543573 0.269413 0.705106 0.263174C0.866638 0.256935 1.02458 0.311879 1.14736 0.417027L4.99832 4.26799C5.11786 4.38824 5.18496 4.55091 5.18496 4.72048C5.18496 4.89004 5.11786 5.05271 4.99832 5.17296L1.14736 9.02392C1.02781 9.1425 0.866457 9.20934 0.69808 9.21005Z",
                            fill: "#CACACA",
                        }
                    }
                }

                // Page Indicator
                span {
                    class: "ml-2 bg-[#F7F7F7] text-[#555555] text-xs font-normal border border-[#EDEDED] rounded-[50px] px-[10px] py-[5px]",
                    "{page_info}"
                }
            }
        }
        div {
            class:"mt-1 border-t border-[#EDEDED] px-1 py-2 flex justify-start items-start gap-4",
            div {
                class:"flex gap-1",
                span {
                    class:"font-light text-[11px] text-[#313131]",
                    "Platform connected :"
                 }

                 span {
                    class:"font-medium text-[11px] text-[#313131]",
                    "02 (Simulation; Brilliant)"
                 }
            }

            div {
                class:"flex gap-1",
                span {
                    class:"font-light text-[11px] text-[#313131]",
                    "Platform disconnected :"
                 }

                 span {
                    class:"font-medium text-[11px] text-[#313131]",
                    "01 (Neuro Shield)"
                 }
            }
        }
    }
}

pub fn test_add_project() {
    let json = fs::read_to_string("assets/sample_project.json").expect("Failed to read file");
    let project: Project = serde_json::from_str(&json).expect("Invalid JSON");
    match add_project(project) {
        Ok(_) => println!("Project added successfully."),
        Err(e) => eprintln!("Error adding project: {}", e),
    }
}

pub fn test_get_projects_by_name() {
    let matches = get_projects_by_name(" ai");
    for p in matches {
        println!("Matched: {}", p.name);
    }
}

pub fn test_update_project() {
    let json = fs::read_to_string("assets/sample_project.json").expect("Failed to read file");
    let project: Project = serde_json::from_str(&json).expect("Invalid JSON");
    match update_project("SmartMartAI", project) {
        Ok(_) => println!("Project updated successfully."),
        Err(e) => eprintln!("Error updating project: {}", e),
    }
}

pub fn test_delete_project() {
    match delete_project("SmartMartAI_@@@@@@@@@@@") {
        Ok(_) => println!("Project deleted successfully."),
        Err(e) => eprintln!("Error deleting project: {}", e),
    }
}

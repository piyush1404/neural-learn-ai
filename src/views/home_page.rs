use dioxus::prelude::*;
use std::fs;

use crate::components::new_project_card::NewProjectCard;
use crate::components::project_card::ProjectCard;

use crate::project_store::{
    add_project, delete_project, get_projects_by_name, load_projects, update_project, Project,
};

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
    let mut tabs = use_signal(|| {
        vec![Tab {
            id: 1,
            title: "Home".to_string(),
            active: true,
        }]
    });
    let next_id = use_signal(|| 2);

    let add_tab = {
        to_owned![tabs, next_id];
        move |_| {
            tabs.write().iter_mut().for_each(|t| t.active = false);
            tabs.write().push(Tab {
                id: next_id(),
                title: format!("Tab {}", next_id()),
                active: true,
            });
            next_id.set(next_id() + 1);
        }
    };

    let mut activate_tab = {
        to_owned![tabs];
        move |id| {
            tabs.write().iter_mut().for_each(|t| t.active = t.id == id);
        }
    };

    let mut close_tab = {
        to_owned![tabs];
        move |id| {
            let mut new_list: Vec<Tab> = tabs().into_iter().filter(|t| t.id != id).collect();
            if !new_list.iter().any(|t| t.active) {
                if let Some(first) = new_list.first_mut() {
                    first.active = true;
                }
            }
            tabs.set(new_list);
        }
    };

    let rendered_tabs = tabs()
        .clone()
        .into_iter()
        .map(|tab| {
            let (bg_class, border_class, text_class, z_class) = if tab.active {
                (
                    "bg-[#e0e0e0] hover:bg-[#d0d0d0]",
                    "border-x border-t border-gray-300",
                    "text-black",
                    "z-10",
                )
            } else {
                ("bg-white", "border border-transparent", "text-gray-600", "z-0")
            };

            let class_name = format!(
                "relative px-4 py-1 flex items-center gap-2 text-sm rounded-t-md cursor-pointer min-w-[100px] flex-shrink-0 {bg_class} {border_class} {text_class} {z_class} -mb-px select-none"
            );

            rsx! {
                div {
                    key: "{tab.id}",
                    class: "{class_name}",
                    onclick: move |_| activate_tab(tab.id),
                    "{tab.title}",
                    button {
                        class: "ml-2 text-gray-800 hover:text-red-500",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            close_tab(tab.id);
                        },
                        "×"
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // test_add_project();
    // test_get_projects_by_name();
    // test_update_project();
    // test_delete_project();

    let page_info = format!("{:02}/{:02}", current_page() + 1, total_pages);

    rsx! {
        div {
            class: "relative px-2 border-b border-gray-300 flex items-end space-x-1",
            div {
                class: "flex overflow-x-auto whitespace-nowrap no-scrollbar",
                {rendered_tabs.into_iter()}
            }
            button {
                class: "w-6 h-6 mb-[1px] bg-gray-200 hover:bg-gray-300 rounded-full text-gray-700 text-lg flex items-center justify-center flex-shrink-0",
                onclick: add_tab,
                "+"
            }
        }
        div {
            class: "flex flex-col h-full w-full border-2 rounded-md",
            div {
                class: "flex items-center gap-4 p-4 border",
                span { class: "text-gray-700 font-medium", "Projects" }

                div {
                    class: "flex items-center gap-3",
                    label {
                        class: "flex items-center gap-1 cursor-pointer",
                        input { r#type: "radio", name: "filter", value: "all", checked: true }
                        span { class: "text-blue-600 font-medium", "All" }
                    }
                    label {
                        class: "flex items-center gap-1 cursor-pointer",
                        input { r#type: "radio", name: "filter", value: "complete" }
                        span { class: "text-green-600 font-medium", "Complete" }
                    }
                    label {
                        class: "flex items-center gap-1 cursor-pointer",
                        input { r#type: "radio", name: "filter", value: "incomplete" }
                        span { class: "text-orange-500 font-medium", "Incomplete" }
                    }
                }

                select {
                    class: "ml-4 px-3 py-1 rounded border border-gray-300 text-gray-700 bg-white shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                    option { value: "", disabled: true, selected: true, "Select Platform:" }
                    option { value: "web", "Web" }
                    option { value: "mobile", "Mobile" }
                    option { value: "desktop", "Desktop" }
                }

                div {
                    class: "ml-auto flex items-center border border-gray-300 rounded-full px-3 py-1 bg-white shadow-sm",
                    input {
                        r#type: "text",
                        placeholder: "Search",
                        class: "ml-2 outline-none border-none bg-transparent text-gray-700"
                    }
                }
            }
            div {
                class: "grid grid-cols-4 gap-4 p-4",
                NewProjectCard {},
               {
                visible_projects.into_iter().map(|project| {
                    rsx! {
                        ProjectCard {
                            name: project.name.clone(),
                            platform: project.platform.clone(),
                            interface: project.interface.clone(),
                            description: project.description.clone(),
                            created_at: project.created_at.clone(),
                            updated_at: project.updated_at.clone(),
                            neurons: project.neurons.clone().map(|n| n)
                        }
                    }
                })
            }
            div {
                class: "absolute bottom-4 right-4 flex items-center gap-2 bg-white px-4 py-2 rounded shadow-md",

                // First Page (⏪)
                button {
                    class: "w-9 h-9 bg-gray-200 hover:bg-gray-300 rounded-full flex items-center justify-center disabled:opacity-50",
                    onclick: move |_| current_page.set(0),
                    disabled: "{current_page() == 0}",
                    "⏪"
                }

                // Previous Page (◀)
                button {
                    class: "w-9 h-9 bg-gray-200 hover:bg-gray-300 rounded-full flex items-center justify-center disabled:opacity-50",
                    onclick: go_prev,
                    disabled: "{current_page() == 0}",
                    "◀"
                }

                // Next Page (▶)
                button {
                    class: "w-9 h-9 bg-gray-200 hover:bg-gray-300 rounded-full flex items-center justify-center disabled:opacity-50",
                    onclick: go_next,
                    disabled: "{current_page() + 1 >= total_pages}",
                    "▶"
                }

                // Last Page (⏩)
                button {
                    class: "w-9 h-9 bg-gray-200 hover:bg-gray-300 rounded-full flex items-center justify-center disabled:opacity-50",
                    onclick: move |_| current_page.set(total_pages.saturating_sub(1)),
                    disabled: "{current_page() + 1 >= total_pages}",
                    "⏩"
                }

                // Page Indicator
                span {
                    class: "ml-2 text-gray-700 font-medium",
                    "{page_info}"
                }
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

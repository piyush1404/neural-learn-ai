use dioxus::prelude::*;
use crate::components::new_project_card::NewProjectCard;
use crate::components::project_card::ProjectCard;
use crate::store::project::{ delete_project, load_projects};
use crate::store::project_schema::Project;
use dioxus_desktop::use_window;

const COMPLETE_BRAIN:Asset = asset!("/assets/icons/complete_brain.svg");
const INCOMPLETE_BRAIN:Asset = asset!("/assets/icons/incomplete_brain.svg");
const SEARCH:Asset = asset!("/assets/icons/search.svg");
const PREV:Asset = asset!("/assets/icons/prev.svg");
const NEXT:Asset = asset!("/assets/icons/next.svg");
const LEFT_ARROW:Asset = asset!("/assets/icons/left_arrow.svg");
const RIGHT_ARROW:Asset = asset!("/assets/icons/right_arrow.svg");


// const page_size: usize = 7;
#[component]
pub fn HomePage() -> Element {

    let window = use_window();
    let width = use_signal(|| window.inner_size().width);

    // Polling window width every 250ms
    use_coroutine(move |mut _rx: UnboundedReceiver<()>| {
        to_owned![width, window];
        async move { 
            loop {
                let current_width = window.inner_size().width;
                if *width.read() != current_width {
                    width.set(current_width);
                }
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            }
        }
    });

    // Determine layout values based on width
    let (page_size, padding_y, row_gap, col_gap) = match *width.read() {
        0..=639 => (2, 12, 10, 10),     // sm
        640..=767 => (3, 16, 15, 15),   // sm+
        768..=1023 => (5, 20, 20, 20),  // md
        1024..=1279 => (5, 30, 30, 30), // lg
        1280..=1535 => (7, 30, 30, 30), // xl
        _ => (14, 50, 50, 50),          // 2xl+
    };

    // Generate dynamic style
    let grid_style = format!(
        "
        width: 100%;
        height: 100%;
        padding: {}px 0px;
        border-radius: 12px;
        display: grid;
        row-gap: {}px;
        column-gap: {}px;
        justify-content: space-evenly;
        overflow-y: auto;
        overflow-x: hidden;
        grid-template-columns: repeat(auto-fill, minmax(270px, max-content));
        ",
        padding_y, row_gap, col_gap
    );

    let mut search_query = use_signal(|| "".to_string());
    let mut platform_filter = use_signal(|| "".to_string());
    let mut status_filter = use_signal(|| "all".to_string());

    let mut is_updating = use_signal(|| false);

    // Example: toggling this causes re-rendering
    let mut projects = use_signal(|| load_projects());

    use_effect(move || {
        if *is_updating.read() {
            // Do something when update is triggered
            projects.set(load_projects());
            is_updating.set(false);
        }
    
        // Optional cleanup or effect teardown
        // || {}
    });
    

    println!("Projects: {:#?}", projects().len());

    let mut current_page = use_signal(|| 0);

    let total_pages = (projects().len() + page_size - 1) / page_size;

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

    let filtered_projects: Vec<Project> = projects()
    .iter()
    .filter(|project| {
        // Match search query
        project.name.to_lowercase().contains(&search_query().to_lowercase())
    })
    .filter(|project| {
        // Match platform
        project.platform.to_lowercase().contains(&platform_filter().to_lowercase())
    })
    // .filter(|project| {
    //     match status_filter().as_str() {
    //         "complete" => project.is_completed == Some(true),
    //         "incomplete" => project.is_completed != Some(true),
    //         _ => true, // "all"
    //     }
    // })
    .cloned()
    .collect();

    let total_pages = (filtered_projects.len() + page_size - 1) / page_size;
    let start = current_page() * page_size;
    let end = (start + page_size).min(filtered_projects.len());

    let visible_projects = filtered_projects[start..end].to_vec();

    // test_delete_project();
    let page_info = format!("{:02}/{:02}", current_page() + 1, total_pages);

    rsx! {
        div {
            style: "margin-left: 28px; margin-right: 28px;",
            class: "flex flex-col bg-[#FFFFFF] border border-[#BEBEBE] rounded-lg mt-1",
            div {
                class: "w-full h-auto flex items-center gap-4 p-3 border-b",
                span { class: "text-[#151515] font-light text-xs", "Projects" }
                div {
                    class:"w-full h-auto flex flex-wrap items-center justify-end gap-4",
                    div {
                        class: "flex flex-wrap items-center gap-4",
                        label {
                            class: "flex items-center gap-2 cursor-pointer",
                            input { 
                                r#type: "radio", name: "filter", value: "all", checked: true,
                                onchange: move |evt| status_filter.set(evt.value()),
                                class: "w-5 h-5 border-2 rounded-full accent-[#0387D9]" 
                        }
                            span { class: "text-[#151515] text-xs font-light", "All" }
                        }
                        label {
                            class: "flex items-center gap-2 cursor-pointer",
                            input { r#type: "radio", name: "filter", value: "complete", 
                                onchange: move |evt| status_filter.set(evt.value()), 
                                class: "w-5 h-5 border-2 rounded-full accent-[#0387D9]
                            "}
                            img {  
                                src: COMPLETE_BRAIN, 
                                alt: "Complete", 
                            }

                            span { class: "text-[#151515] text-xs font-light", "Complete" }
                        }
                        label {
                            class: "flex items-center gap-2 cursor-pointer",
                            input { 
                                r#type: "radio", name: "filter", value: "incomplete", 
                                onchange: move |evt| status_filter.set(evt.value()),
                                class: "w-5 h-5 border-2 rounded-full accent-[#0387D9]" 
                            }
                            img {  
                                src: INCOMPLETE_BRAIN, 
                                alt: "Incomplete",
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
                    onchange: move |evt| platform_filter.set(evt.value()),
                    option { value: "", disabled: true, hidden: true, selected: platform_filter() == "", "Select Platform :" }
                    option { value: "Simulation", "Simulation" }
                    option { value: "Brilliant", "Brilliant" }
                    option { value: "Neuro Shield", "Neuro Shield" }
                    }

                    span {  
                        class: "h-5 border border-[#BEBEBE]", 
                    }

                    div {
                        class: "flex items-center border border-[#EDEDED] rounded-[50px] px-3 py-1 bg-[#F7F7F7] shadow-sm",
                        img {  src: "{SEARCH}", alt: "Search" }
                        input {
                            r#type: "text",
                            oninput: move |evt| search_query.set(evt.value()),
                            placeholder: "Search",
                            class: "ml-2 outline-none border-none bg-transparent font-normal text-xs text-[#555555]"
                        }
                    }
                }
            }
            div {
                style: "{grid_style}",
                // style: "
                //     // background: green;
                //     width: 100%;
                //     height: 100%;
                //     padding: 30px 0px;
                //     border-radius: 12px;
                //     display: grid;
                //     row-gap: 30px;
                //     column-gap: 30px;
                //     justify-content: space-evenly;
                //     overflow-y: auto;
                //     overflow-x: hidden;
                //     grid-template-columns: repeat(auto-fill, minmax(270px, max-content));
                // ",
                // class: "h-full grid grid-cols-4 gap-4 p-4",
                // class: "w-full h-full flex flex-wrap justify-start gap-8 p-5",
        //         class : "w-full rounded-[12px] grid md:gap-x-4 md:gap-y-3 gap-3 
        //         grid-cols-[repeat(auto-fill,minmax(8rem,max-content))] 
        //         md:grid-cols-[repeat(auto-fill,minmax(10rem,max-content))] 
        //         lg:grid-cols-[repeat(auto-fill,minmax(155px,max-content))]
        //         xl:grid-cols-[repeat(auto-fill,minmax(145px,max-content))] 
        //         2xl:grid-cols-[repeat(auto-fill,minmax(11.5rem,max-content))] 
        //         justify-between xl:gap-4 overflow-y-auto overflow-x-hidden [&::-webkit-scrollbar]:w-1 [&::-webkit-scrollbar-track]:bg-gray-100 [&::-webkit-scrollbar-thumb]:bg-gray-300
        //   dark:[&::-webkit-scrollbar-track]:bg-neutral-700 dark:[&::-webkit-scrollbar-thumb]:bg-neutral-500 [&::-webkit-scrollbar-track]:rounded-full [&::-webkit-scrollbar-thumb]:rounded-full ",
                NewProjectCard { is_updating: is_updating },
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
                            neurons: project.neurons.clone().map(|n| n),
                            is_updating: is_updating
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
                    img {  src: "{PREV}", alt: ""}
                }

                // Previous Page (◀)
                button {
                    class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                    onclick: go_prev,
                    disabled: "{current_page() == 0}",
                    img {  src: "{LEFT_ARROW}", alt: "left"}
                }

                // Next Page (▶)
                button {
                    class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                    onclick: go_next,
                    disabled: "{current_page() + 1 >= total_pages}",
                    img {  src: "{RIGHT_ARROW}", alt: "right"}
                }

                // Last Page (⏩)
                button {
                    class: "w-6 h-6 bg-[#F7F7F7] text-[#CACACA] border border-[#EDEDED] rounded-full flex items-center justify-center",
                    onclick: move |_| current_page.set(total_pages.saturating_sub(1)),
                    disabled: "{current_page() + 1 >= total_pages}",
                    img {  src: "{NEXT}", alt: "Next"}
                }

                // Page Indicator
                span {
                    class: "ml-2 bg-[#F7F7F7] text-[#555555] text-xs font-normal border border-[#EDEDED] rounded-[50px] px-[10px] py-[5px]",
                    "{page_info}"
                }
            }
        }
        div {
            class:"mt-1 border-t bg-[#FFFFFF] border-[#EDEDED] px-1 py-2 flex flex-wrap justify-start items-start gap-4",
            style: "margin-left: 28px;",
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

pub fn test_delete_project() {
    match delete_project("SmartMartAI_@@@@@@@@@@@") {
        Ok(_) => println!("Project deleted successfully."),
        Err(e) => eprintln!("Error deleting project: {}", e),
    }
}

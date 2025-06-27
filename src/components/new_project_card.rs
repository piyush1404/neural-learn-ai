use dioxus::prelude::*;

#[component]
pub fn NewProjectCard() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut show_advanced = use_signal(|| false);

    let mut categories = use_signal(|| vec![
        ("Background".to_string(), "bg-[#4C4C4C]".to_string()),
        ("Object".to_string(), "bg-[#F85858]".to_string())
    ]);


    let mut project_name = use_signal(|| "".to_string());
    let mut platform = use_signal(|| "Simulation".to_string());
    let mut project_type = use_signal(|| "Image".to_string());

    let mut description = use_signal(|| "".to_string());
    let mut normalized = use_signal(|| false);
    let mut algorithm = use_signal(|| "HOG".to_string());

    let mut roi_width = use_signal(|| 64);
    let mut roi_height = use_signal(|| 64);
    let mut block_width = use_signal(|| 2);
    let mut block_height = use_signal(|| 2);
    let mut range_min = use_signal(|| 5);
    let mut range_max = use_signal(|| 45000);

    let selected_label = use_signal(|| "Image".to_string());
    let selected_icon = use_signal(|| "🖼️".to_string());
    let mut show_options = use_signal(|| false);

    let options = vec![
        ("Image", "🖼️"),
        ("Video", "🎥"),
        ("Audio", "🎧"),
        ("Text", "📝"),
        ("3D", "🧊"),
    ];

    // Build the options list separately
    let option_list: Vec<_> = options
        .iter()
        .map(|(label, icon)| {
            let label = label.to_string();
            let icon = icon.to_string();
            let mut selected_label = selected_label.clone();
            let mut selected_icon = selected_icon.clone();
            let mut show_options = show_options.clone();

            rsx!(
                li {
                    class: "flex items-center gap-2 px-4 py-2 text-[#FFFFFF] rounded hover:bg-[#555555]  cursor-pointer",
                    onclick: move |_| {
                        selected_label.set(label.clone());
                        selected_icon.set(icon.clone());
                        show_options.set(false);
                    },
                    span { "{icon}" }
                    span { "{label}" }
                }
            )
        })
        .collect();


    rsx! {
        div {
            class: "border border-[#BEBEBE] rounded-xl shadow-sm flex flex-col items-center justify-center gap-2 cursor-pointer",
            onclick: move |_| show_modal.set(true),
            div {
                class: "w-20 h-20 border-2 border-dashed border-[#999999] flex items-center justify-center text-[#0387D9] text-xl rounded-sm",
                "+"
            }
            span {
                class: "text-[#0387D9] text-sm font-medium",
                "New Project"
            }
        }
        if *show_modal.read() {
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
                                value: "{project_name}",
                                oninput: move |e| project_name.set(e.value().to_string())
                            }
                        }
                        div {
                            label { class: "block mb-1 text-xs font-normal text-[#4D4D4D]", "Select Platform" }
                            select {
                                class: "w-full border-[0.5px] border-[#8F8F8F] rounded px-4 py-1 font-normal text-xs text-[#313131 appearance-none pr-7",
                                style: r#"
                                color: #555555;
                                background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                                background-repeat: no-repeat;
                                background-position: right 0.75rem center;
                                background-size: 10px 6px;
                            "#,
                                option { "Simulation" }
                                option { "Brilliant" }
                                option { "Neuro Shield" }
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
                                        span { "{selected_icon()}" }
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
                            oninput: move |e| description.set(e.value().to_string())
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
                                    class: "relative mb-2 space-y-2 h-[266px]",
                    
                                    span { class: "block mb-2 text-xs text-[#404040] font-normal", "Categories" }
                    
                                    div {
                                        class: "flex items-center gap-2 text-xs text-[#404040] mb-1",
                                        span { class: "w-[91px] text-[10px] h-[15px] rounded", "Name" }
                                        span { class: "w-[28px] text-[10px] h-[15px]", "Color" }
                                        span {}
                                    }
                    
                                    for (index, (name, color)) in categories.read().iter().cloned().enumerate() {
                                        div {
                                            class: "flex items-center gap-2",
                    
                                            input {
                                                class: "border p-1 w-[91px] rounded text-sm h-[20px]",
                                                value: "{name}",
                                                oninput: move |e| {
                                                    let mut updated = categories.write().clone();
                                                    updated[index].0 = e.value().clone();
                                                    categories.set(updated);
                                                }
                                            }
                    
                                            div {
                                                div {
                                                    class: format_args!("w-[28px] h-[20px] rounded border border-gray-300 {}", color),
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
                                                            updated.push(("".to_string(), "bg-black".to_string()));
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
                                                    ("Background".to_string(), "bg-zinc-800".to_string()),
                                                    ("Object".to_string(), "bg-red-500".to_string()),
                                                ]);
                                            },
                                            "Reset"
                                        }
                                    }
                                }
                            }
                    
                            // Context Panel
                            div {
                                class: "pt-3 border-r w-[22%]",
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
                                class: "pt-3 w-[56%]",
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
                                                style: r#"
                                                color: #555555;
                                                background-image: url("data:image/svg+xml,%3Csvg width='10' height='6' viewBox='0 0 10 6' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M4.99997 5.70028C4.82075 5.70028 4.64155 5.63185 4.50492 5.49528L0.205141 1.19546C-0.0683804 0.921938 -0.0683804 0.478469 0.205141 0.205058C0.478552 -0.0683528 0.921933 -0.0683528 1.19548 0.205058L4.99997 4.00978L8.80449 0.205191C9.07801 -0.0682199 9.52135 -0.0682199 9.79474 0.205191C10.0684 0.478602 10.0684 0.922071 9.79474 1.19559L5.49503 5.49541C5.35832 5.63201 5.17913 5.70028 4.99997 5.70028Z' fill='%23555555'/%3E%3C/svg%3E");
                                                background-repeat: no-repeat;
                                                background-position: right 0.75rem center;
                                                background-size: 10px 6px;
                                            "#,
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
                                            value: "16",
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
                                            value: "16",
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
                                            value: "1",
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
                                            value: "1",
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
                                            value: "16",
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
                                            value: "16",
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
                                        let data = serde_json::json!({
                                            "name": project_name.read().to_string(),
                                            "platform": platform.read().to_string(),
                                            // "interface": interface.read().to_string(),
                                            "type": project_type.read().to_string(),
                                            "description": description.read().to_string(),
                                            "categories": categories.read().iter().enumerate().map(|(i, (name, color))| {
                                                serde_json::json!({
                                                    "id": i + 1,
                                                    "name": name,
                                                    "color": color,
                                                    "context_id": i + 1 // just as an example
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
                                
                                        // Print or send data somewhere
                                        println!("{:#?}", data);
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
}
        
    


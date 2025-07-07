use dioxus::prelude::*;
use crate::components::project_form::ProjectForm;

#[derive(PartialEq, Props, Clone)]
pub struct NewProjectCardProps {
    is_updating: Signal<bool>,
}


#[component]
pub fn NewProjectCard(props: NewProjectCardProps) -> Element {
    // Access the passed signal
    let is_updating = props.is_updating;
    let mut show_modal = use_signal(|| false);
    let mut show_advanced = use_signal(|| false);


    let mut categories = use_signal(|| vec![
        ("Background".to_string(), "bg-neutral-800".to_string()),
        ("Object".to_string(), "bg-red-600".to_string())
    ]);

    let default_categories = vec![
        ("Background".to_string(), "bg-neutral-800".to_string()),
        ("Object".to_string(), "bg-red-600".to_string()),
    ];

    let mut description = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "w-[270px] h-[203px] border border-[#BEBEBE] rounded-xl shadow-sm flex flex-col items-center justify-center gap-2 cursor-pointer hover:shadow-md hover:scale-[1.01] cursor-pointer transition-all duration-200",
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
            ProjectForm {
                show_modal: show_modal,
                is_updating: is_updating
            }
        }
    }
}
        
    


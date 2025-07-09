use dioxus::prelude::*;

use crate::components::learn::Learn;
use crate::components::menu_bar::MenuBar;
use crate::components::recognize::Recognize;
use crate::components::setting::Setting;
use crate::state::tabs::TabContext;

// Shared struct
#[derive(Clone)]
pub struct SharedData {
    pub algorithm: String,
    pub clear_clicked: bool
}
#[derive(PartialEq, Props, Clone)]
pub struct AppState {
    pub shared: Signal<SharedData>
}

#[component]
pub fn ProjectDetails() -> Element {
    let mut tab_context = use_context::<Signal<TabContext>>();
    // Print tabs
    let ctx = tab_context.read();
    let tabs = ctx.tabs.read();
    let active_id = ctx.active_tab.read();

    // println!("🧠 Total Tabs (in effect): {}", tabs.len());
    // println!("⭐ Active Tab ID: {}", active_id);
    // for (i, tab) in tabs.iter().enumerate() {
    //     println!("#{}: [{}] {} {:?}", i, tab.id, tab.title, tab.icon);
    // }
    let shared = use_signal(|| SharedData {
        algorithm: "grayscale".to_string(),
        clear_clicked: false
    });

    let app_state = AppState { shared };
    rsx! {
        div {
            class: "flex mt-1",

            // Left panel (Menu + Learn + Recognize)
            div {
                class: "h-full w-3/4 flex flex-col border-t border-b border-l border-[#BEBEBE] bg-[#FFFFFF] rounded-tl-[15px] rounded-bl-[15px]",

                // Menu bar component
                MenuBar {app_state: app_state.clone()}

                div {
                    class: "h-full border-t border-[#BEBEBE]",

                    div {
                        class: "h-full flex flex-col",

                        // Learn component
                        Learn {app_state: app_state.clone()}

                        // Recognizegnize component
                        Recognize {}
                    }
                }
            }

            // Right panel (Settings)
            div {
                class: "w-1/4 border-[0.5px] border-[#BEBEBE] bg-[#FAFAFA] rounded-tr-[15px] rounded-br-[15px] flex flex-col gap-2",

                // Setting component
                Setting {app_state: app_state.clone()}
            }
        }
    }
}

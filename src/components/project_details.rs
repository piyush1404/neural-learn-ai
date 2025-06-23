use dioxus::prelude::*;

use crate::components::learn::Learn;
use crate::components::menu_bar::MenuBar;
use crate::components::reco::Reco;
use crate::components::setting::Setting;

#[component]
pub fn ProjectDetails() -> Element {
    rsx! {
        div {
            class: "flex mt-1",

            // Left panel (Menu + Learn + Reco)
            div {
                class: "h-full w-3/4 flex flex-col border-t border-b border-l border-[#BEBEBE] bg-[#FFFFFF] rounded-tl-[15px] rounded-bl-[15px]",

                // Menu bar component
                MenuBar {}

                div {
                    class: "h-full border-t border-[#BEBEBE]",

                    div {
                        class: "h-full flex flex-col",

                        // Learn component
                        Learn {}

                        // Recognize component
                        Reco {}
                    }
                }
            }

            // Right panel (Settings)
            div {
                class: "p-3 min-h-full w-1/4 px-[13px] py-[14px] border-[0.5px] border-[#BEBEBE] bg-[#FAFAFA] rounded-tr-[15px] rounded-br-[15px] flex flex-col gap-2",

                // Setting component
                Setting {}
            }
        }
    }
}

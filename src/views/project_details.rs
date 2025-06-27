use dioxus::prelude::*;

use crate::components::learn::Learn;
use crate::components::menu_bar::MenuBar;
use crate::components::recognize::Recognize;
use crate::components::setting::Setting;

#[component]
pub fn ProjectDetails() -> Element {
    rsx! {
        div {
            class: "flex mt-1",

            // Left panel (Menu + Learn + Recognize)
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

                        // Recognizegnize component
                        Recognize {}
                    }
                }
            }

            // Right panel (Settings)
            div {
                class: "w-1/4 border-[0.5px] border-[#BEBEBE] bg-[#FAFAFA] rounded-tr-[15px] rounded-br-[15px] flex flex-col gap-2",

                // Setting component
                Setting {}
            }
        }
    }
}

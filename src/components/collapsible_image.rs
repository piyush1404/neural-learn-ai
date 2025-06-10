use crate::components::image_upload::ImageUploader;
use dioxus::prelude::*;
#[component]
pub fn CollapsibleImage() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div {
            class: "border border-gray-300 rounded-lg shadow-md p-4",
            div {
                class: "flex items-center justify-between",
                h2 { class: "text-lg font-medium", "Image Preview" }
                button {
                    class: "text-sm text-blue-500 hover:underline",
                    onclick: move |_| open.set(!open()),
                    {if open() { "Hide" } else { "Show" }}
                }
                ImageUploader {  }
            }

            if open() {

                    div {
                        class: "mt-4 transition-all duration-300 ease-in-out",
                        // img {
                        //     class: "rounded-lg shadow w-full h-auto",
                        //     src: "https://via.placeholder.com/300",
                        //     alt: "Preview Image",
                        // }

                        ImageUploader {  }
                    }


            }
        }
    }
}

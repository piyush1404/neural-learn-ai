use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus::LaunchBuilder;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod components;
mod date_format;
mod store;
mod views;

use views::home_page::HomePage;
use views::project_details::ProjectDetails;


fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_window(WindowBuilder::new().with_title("Neuron Learn AI"))
                .with_menu(None),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
         document::Stylesheet { rel: "stylesheet", href: MAIN_CSS }
         style { "{include_str!(\"../assets/tailwind.css\")}" }

        div {
            class: "w-full h-full bg-white font-poppins px-7",

            HomePage {}

            // ProjectDetails {}

        }


    }
}

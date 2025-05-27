use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus::LaunchBuilder;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod components;
mod views;

use components::top_bar::TopBar;
use views::project_grid::ProjectGrid;

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
        //  document::Stylesheet { rel: "stylesheet", href: TAILWIND_CSS }
         style { "{include_str!(\"../assets/tailwind.css\")}" }

        div {
            class: "w-full h-full bg-white font-sans",
            TopBar {}
            ProjectGrid {}
        }


    }
}

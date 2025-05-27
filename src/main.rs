use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus::LaunchBuilder;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod components;
mod views;

use components::project_details::ProjectDetails;
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

            // ProjectDetails {
            //     title: "Neuron Classifier",
            //     engine: "Python",
            //     file_roi: "Region 24",
            //     min_if: "10",
            //     max_if: "50",
            //     algorithm: "YOLOv5",
            //     block_width: "128",
            //     block_height: "128",
            //     step_xy: "10",
            //     learning_mode: "Supervised",
            //     output_mode: "Bounding Boxes",
            //     categories: vec!["Neuron".into(), "Glia".into()],
            //     nn_capacity: "2048",
            //     neurons: "500",
            //     search_area: "256x256",
            //     created: "2024-11-01",
            //     edited: "2025-05-25",
            //  }

        }


    }
}

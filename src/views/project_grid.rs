use dioxus::prelude::*;

use crate::components::new_project_card::NewProjectCard;
use crate::components::project_card::ProjectCard;

#[component]
pub fn ProjectGrid() -> Element {
    rsx!(
        div {
            class: "p-6 grid grid-cols-4 gap-4",
            NewProjectCard {},
            // Add more ProjectCard {} here...
        }
    )
}

use dioxus::prelude::*;

use crate::components::new_project_card::NewProjectCard;
use crate::components::project_card::ProjectCard;

#[component]
pub fn ProjectGrid() -> Element {
    rsx!(
        div {
            class: "p-6 grid grid-cols-4 gap-4",
            NewProjectCard {},
            ProjectCard {
                title: "Cars",
                engine: "Simulation",
                file_roi: "16X16, Sub sample, with blocks 1X1",
                min_if: "04",
                max_if: "3656",
                search_area: "[1,2,3,2]",
                nn_capacity: "6878",
                neurons: "2",
                created: "10/05/2025",
                edited: "12/05/2025"
            },
            ProjectCard {
                title: "Traffic Flow",
                engine: "Prediction",
                file_roi: "32X32, Full sample, with blocks 2X2",
                min_if: "10",
                max_if: "4789",
                search_area: "[3,4,2,5]",
                nn_capacity: "8902",
                neurons: "4",
                created: "02/04/2025",
                edited: "05/05/2025"
            },
            ProjectCard {
                title: "Weather Patterns",
                engine: "Analysis",
                file_roi: "64X64, Sub sample, with blocks 4X4",
                min_if: "5",
                max_if: "2345",
                search_area: "[6,1,3,8]",
                nn_capacity: "7643",
                neurons: "3",
                created: "03/03/2025",
                edited: "06/04/2025"
            },
            ProjectCard {
                title: "Face Recognition",
                engine: "Neural Net",
                file_roi: "128X128, Full sample, with blocks 8X8",
                min_if: "1",
                max_if: "9210",
                search_area: "[8,8,8,8]",
                nn_capacity: "15000",
                neurons: "12",
                created: "01/02/2025",
                edited: "14/03/2025"
            },
            ProjectCard {
                title: "Lunar Mapping",
                engine: "Simulation",
                file_roi: "48X48, Sub sample, with blocks 3X3",
                min_if: "7",
                max_if: "3700",
                search_area: "[3,7,5,2]",
                nn_capacity: "6750",
                neurons: "5",
                created: "25/01/2025",
                edited: "01/03/2025"
            },
            ProjectCard {
                title: "Retail Forecasting",
                engine: "Statistical",
                file_roi: "16X16, Full sample, with blocks 1X1",
                min_if: "12",
                max_if: "4200",
                search_area: "[2,2,2,2]",
                nn_capacity: "8021",
                neurons: "6",
                created: "14/02/2025",
                edited: "25/03/2025"
            },
            ProjectCard {
                title: "Drone Navigation",
                engine: "Control System",
                file_roi: "24X24, Sub sample, with blocks 2X2",
                min_if: "3",
                max_if: "5600",
                search_area: "[4,5,3,6]",
                nn_capacity: "9923",
                neurons: "9",
                created: "05/01/2025",
                edited: "17/02/2025"
            },
            ProjectCard {
                title: "Satellite Imagery",
                engine: "Vision Processing",
                file_roi: "128X128, Full sample, with blocks 4X4",
                min_if: "9",
                max_if: "8500",
                search_area: "[7,6,5,4]",
                nn_capacity: "12000",
                neurons: "11",
                created: "09/03/2025",
                edited: "22/04/2025"
            },
            ProjectCard {
                title: "Audio Compression",
                engine: "Signal Processing",
                file_roi: "8X8, Sub sample, with blocks 1X1",
                min_if: "2",
                max_if: "3080",
                search_area: "[2,3,1,4]",
                nn_capacity: "5090",
                neurons: "3",
                created: "11/05/2025",
                edited: "15/05/2025"
            },
            ProjectCard {
                title: "Agriculture Yield",
                engine: "Predictive Analytics",
                file_roi: "40X40, Full sample, with blocks 2X2",
                min_if: "6",
                max_if: "6700",
                search_area: "[3,6,3,6]",
                nn_capacity: "8855",
                neurons: "7",
                created: "20/02/2025",
                edited: "28/03/2025"
            },
            ProjectCard {
                title: "Text Classification",
                engine: "Natural Language",
                file_roi: "12X12, Sub sample, with blocks 1X1",
                min_if: "4",
                max_if: "2900",
                search_area: "[1,1,1,1]",
                nn_capacity: "4300",
                neurons: "2",
                created: "03/01/2025",
                edited: "10/02/2025"
            }

            // Add more ProjectCard {} here...
        }
    )
}

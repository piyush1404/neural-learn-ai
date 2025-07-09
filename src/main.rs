use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus::LaunchBuilder;
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const HOME: Asset = asset!("/assets/icons/home.svg");
mod state;
mod components;
mod date_format;
mod store;
mod views;

use components::tabs::Tabs;
use state::tabs::TabContext;
use components::top_bar::TopBar;
use views::home_page::HomePage;
use views::project_details::ProjectDetails;
use store::project::ProjectState;

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_window(WindowBuilder::new().with_title("Neural Vision"))
                .with_menu(None),
        )
        .launch(app);
}

fn app() -> Element {
    provide_context(ProjectState::new());
    let tab_context = use_signal(TabContext::new); // ✅ stays persistent

    use_context_provider(|| tab_context.clone());

    let initialized = use_signal(|| false);

    {
        let mut initialized = initialized.clone();
        let mut tab_context = tab_context.clone();
        use_effect(move || {
            if !*initialized.read() {
                println!("Adding Home tab once");

                tab_context.write().add_tab(
                    "Home",
                    rsx! { crate::views::home_page::HomePage {} },
                    Some(rsx! {
                        img {  
                            src: HOME,
                            alt: "Home",
                        }
                    }),
                );

                initialized.set(true);

                // // Print tabs
                // let ctx = tab_context.read();
                // let tabs = ctx.tabs.read();
                // let active_id = ctx.active_tab.read();

                // println!("🧠 Total Tabs (in effect): {}", tabs.len());
                // println!("⭐ Active Tab ID: {}", active_id);
                // for (i, tab) in tabs.iter().enumerate() {
                //     println!("#{}: [{}] {} {:?}", i, tab.id, tab.title, tab.icon);
                // }
            }
            (|| ())()
        });
    }

    // Live print
    let ctx = tab_context.read();
    let tabs = ctx.tabs.read();
    let active_id = ctx.active_tab.read();

    // println!("🧠 Total Tabs: {}", tabs.len());
    // println!("⭐ Active Tab ID: {}", active_id);
    // for (i, tab) in tabs.iter().enumerate() {
    //     println!("#{}: [{}] {}", i, tab.id, tab.title);
    // }

    rsx! {
        link { rel: "stylesheet", href: MAIN_CSS }
        style { "{include_str!(\"../assets/tailwind.css\")}" }

        div {
            class: "w-full h-full bg-white font-poppins",
            components::tabs::Tabs {}

            match tabs.iter().find(|t| t.id == *ctx.active_tab.read()) {
                Some(tab) => rsx!(div { class: "mt-4", {tab.content.clone()} }),
                None => rsx!(div { class: "mt-4 text-gray-500", "No tab selected" })
            }
        }
    }
}




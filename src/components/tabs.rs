use dioxus::prelude::*;
use crate::state::tabs::TabContext;

#[component]
pub fn Tabs() -> Element {
    let tab_context = use_context::<Signal<TabContext>>();
    let tabs = tab_context.read().tabs.read().clone();
    let active_tab_id = *tab_context.read().active_tab.read();

    let tab_bar = tabs.iter().map(|tab| {
        let mut tab_context = tab_context.clone();
        let tab_id = tab.id;
        let is_active = active_tab_id == tab_id;
        let (bg_class, border_class, text_class, z_class) = if is_active {
            (
                // "bg-[#e0e0e0] hover:bg-[#d0d0d0]",
                "bg-white",
                "border-x border-t border-gray-300",
                "text-black",
                "z-10",
            )
        } else {
            ("bg-white", "border-b", "text-gray-600", "z-0")
        };
        let class_name = format!(
            "ml-px-0_5 relative px-4 py-1 flex items-center gap-2 text-sm rounded-t-md cursor-pointer min-w-[100px] flex-shrink-0 {bg_class} {border_class} {text_class} {z_class} -mb-px select-none"
        );
// Print tabs
                let ctx = tab_context.read();
                let tabs = ctx.tabs.read();
                let active_id = ctx.active_tab.read();

                // println!("🧠 Total Tabs (in effect): {}", tabs.len());
                // println!("⭐ Active Tab ID: {}", active_id);
                // for (i, tab) in tabs.iter().enumerate() {
                //     println!("#{}: [{}] {} {:?}", i, tab.id, tab.title, tab.icon);
                // }
        rsx!(
            div {
                key: "{tab_id}",
                class: "{class_name} cursor-pointer select-none",
                onclick: move |_| {
                    tab_context.write().active_tab.set(tab_id);
                },
                
                // ✅ Correct way: wrap in `{}` to return node
                {if let Some(icon) = &tab.icon {
                    rsx!( div { class: "mr-1", {icon.clone()} } )
                }else{
                    rsx!(span { class: "mr-1 text-base", "🏠" })
                }}
                if *active_id == tab_id {
                    span {
                        class: "truncate max-w-[90px] text-[#0387D9] font-medium",
                        "{tab.title}"
                    }
                } else {
                    span {
                        class: "truncate max-w-[90px] text-gray-600",
                        "{tab.title}"
                    }
                },

                // span {
                //     class: "truncate max-w-[90px]",
                //     "{tab.title}"
                // }
                button {
                    class: "ml-2 text-gray-400 hover:text-red-500 text-sm",
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        tab_context.write().close_tab(tab_id);
                    },
                    "×"
                }
            }
        )
    });

    rsx! {
        div {
            class: "flex items-end px-2 border-b h-12",
            {tab_bar}
        }
    }
}

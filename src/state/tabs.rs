use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub content: Element,
    pub icon: Option<Element>, // ✅ Add icon support
}

#[derive(Clone)]
pub struct TabContext {
    pub tabs: Signal<Vec<Tab>>,
    pub active_tab: Signal<usize>,
    pub next_id: Signal<usize>,
}

impl TabContext {
    pub fn new() -> Self {
        Self {
            tabs: Signal::new(vec![]),
            active_tab: Signal::new(0),
            next_id: Signal::new(1),
        }
    }

    // ✅ Properly adds a tab, sets active tab automatically
    pub fn add_tab(&mut self, title: &str, content: Element, icon: Option<Element>) {
        let id = *self.next_id.read();
        self.tabs.write().push(Tab {
            id,
            title: title.to_string(),
            content,
            icon, // if you're using icons
        });
        *self.next_id.write() += 1;
        self.active_tab.set(id); // ✅ THIS must be present
    }

    // ✅ Remove tab and update active tab to fallback
    pub fn close_tab(&mut self, id: usize) {
        let mut tabs = self.tabs.write();
        if let Some(index) = tabs.iter().position(|tab| tab.id == id) {
            tabs.remove(index);
            if !tabs.is_empty() {
                let new_index = if index == 0 { 0 } else { index - 1 };
                self.active_tab.set(tabs[new_index].id);
            } else {
                self.active_tab.set(0);
            }
        }
    }

    // ✅ Gets currently active tab
    pub fn active_tab(&self) -> Option<Tab> {
        let active_id = self.active_tab.read(); // ✅ NOT active_id
        let tabs = self.tabs.read();
        tabs.iter().find(|tab| tab.id == *active_id).cloned()
    }

}

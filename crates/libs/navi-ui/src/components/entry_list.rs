use std::path::PathBuf;
use std::sync::Arc;
use gpui::*;
use crate::components::entry_item::EntryItem;
use crate::screens::browser::Browser;

pub struct EntryList {
    list_state: ListState,
    browser: WeakEntity<Browser>,
    pub entries: Arc<[PathBuf]>,
}

impl EntryList {
    pub fn new(entries: Arc<[PathBuf]>, browser: WeakEntity<Browser>) -> Self {
        Self {
            list_state: ListState::new(entries.len(), ListAlignment::Top, px(500.)).measure_all(),
            browser,
            entries,
        }
    }

    pub fn set_entries(&mut self, entries: Arc<[PathBuf]>, cx: &mut Context<Self>) {
        self.entries = entries;
        self.list_state =
            ListState::new(self.entries.len(), ListAlignment::Top, px(500.)).measure_all();

        cx.notify();
    }
}

impl Render for EntryList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let browser = self.browser.clone();
        let entries = self.entries.clone();

        list(self.list_state.clone(), move |index, _window, _cx| {
            let entry = entries.get(index).expect("entry index out of bounds").clone();

            div()
                .w_full()
                .child(EntryItem::new(entry, index, browser.clone()))
                .into_any()
        })
            .size_full()
    }
}
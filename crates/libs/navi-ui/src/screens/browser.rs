use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use gpui::*;
use navi_core::list_entries;
use crate::chooser::ChooserState;
use crate::ChooserResult;
use crate::components::entry_list::{EntryList};
use crate::components::navbar::{NavBar, NavBarEvent};
use crate::config::NaviMode;

pub struct Browser {
    pub directory: PathBuf,
    mode: NaviMode,
    navbar: Entity<NavBar>,
    chooser: Option<ChooserState>,

    entry_list: Entity<EntryList>,
    pub selected_entries: HashSet<usize>,
    pub first_selected: Option<usize>,
}

impl Browser {
    pub fn new(directory: &std::path::Path, mode: &NaviMode, chooser: Option<ChooserState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let browser = cx.entity().downgrade();
        let entries: Arc<[PathBuf]> = list_entries(directory).into();

        let entry_list = cx.new(|_cx| {
            EntryList::new(entries, browser.clone())
        });

        let navbar = cx.new(|cx| {
            NavBar::new(&directory, browser, window, cx)
        });

        cx.subscribe_in(&navbar, window, |this, _navbar, event, window, cx| {
            match event {
                NavBarEvent::Navigate(path) => {
                    this.open_path(path.clone(), cx, window);
                }
            }
        }).detach();

        Self {
            directory: directory.to_owned(),
            mode: mode.to_owned(),
            navbar,
            chooser,

            entry_list,
            selected_entries: HashSet::new(),
            first_selected: None,
        }
    }

    pub fn open_path(&mut self, path: PathBuf, cx: &mut Context<Self>, window: &mut Window) {
        if path.is_dir() {
            self.set_directory(path, cx, window);
            return;
        }

        if path.is_file() {
            match self.mode {
                NaviMode::Browser => self.open_file_in_system(path),
                NaviMode::FileChooser => self.submit_selection(path, window, cx),
            }
        }
    }

    fn set_directory(&mut self, path: PathBuf, cx: &mut Context<Self>, window: &mut Window) {
        if path == self.directory {
            return;
        }

        self.directory = path;

        self.entry_list.update(cx, |view, cx| {
            let entries: Arc<[PathBuf]> = list_entries(&self.directory).into();
            view.set_entries(entries, cx);
        });

        self.selected_entries = HashSet::new();
        self.first_selected = None;

        self.navbar.update(cx, |view, cx| {
            view.nav_input.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(self.directory.to_string_lossy()),
                    window,
                    cx
                )
            })
        });

        cx.notify();
    }

    fn open_file_in_system(&self, path: PathBuf) {
        let terminal = env::var("TERMINAL").unwrap_or_else(|_| "xterm".to_string());

        if let Err(err) = Command::new(terminal)
            .arg("-e")
            .arg("xdg-open")
            .arg(&path)
            .spawn()
        {
            eprintln!("failed to spawn terminal opener: {err}");
        }
    }

    pub fn submit_selection(&mut self, path: PathBuf, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chooser) = self.chooser.as_mut() {
            chooser.submit(ChooserResult::Selected(path));
        }

        window.remove_window();
        cx.quit();
        cx.shutdown();
    }

    pub fn _cancel_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(chooser) = self.chooser.as_mut() {
            chooser._cancel();
        }

        window.remove_window();
        cx.quit();
    }
}

impl Render for Browser {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(
                self.navbar.clone()
            )
            .child(
                self.entry_list.clone()
            )
    }
}
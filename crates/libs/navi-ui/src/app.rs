use std::env;
use std::path::PathBuf;
use std::process::Command;

use gpui::*;
use gpui::private::anyhow;
use gpui_component::input::{InputEvent, InputState};
use gpui_component::list::{ListDelegate, ListItem, ListState};
use gpui_component::IndexPath;

use navi_core::list_files;

use crate::chooser::{ChooserLaunch, ChooserRequest, ChooserResult, ChooserState};
use crate::components::file_item::file_item;
use crate::config::{NaviConfig, NaviMode};
use crate::screens::browser::browser;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Screen {
    #[default]
    Browser,
}

#[derive(Debug, Clone)]
pub struct FileBrowserDelegate {
    pub files: Vec<PathBuf>,
    pub selected_index: Vec<Option<IndexPath>>,
    pub dragging_index: Option<IndexPath>,
    pub drag_over_index: Option<IndexPath>,
    pub is_dragging: bool,
}

impl Default for FileBrowserDelegate {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            selected_index: vec![None],
            dragging_index: None,
            drag_over_index: None,
            is_dragging: false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileListEvent {
    Open(PathBuf),
}

impl EventEmitter<FileListEvent> for ListState<FileBrowserDelegate> {}

impl FileBrowserDelegate {
    pub fn with_items(files: Vec<PathBuf>) -> Self {
        Self { files, ..Default::default() }
    }

    pub fn set_selected_index(&mut self, selected_index: Option<IndexPath>) {
        self.selected_index.clear();
        self.selected_index.push(selected_index);
    }

    pub fn add_selected_index(&mut self, selected_index: Option<IndexPath>) {
        self.selected_index.push(selected_index);
    }
}

impl ListDelegate for FileBrowserDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.files.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<ListItem> {
        self.files.get(ix.row).cloned().map(|item| file_item(self, ix, item, cx))
    }

    fn set_selected_index(&mut self, _ix: Option<IndexPath>, _window: &mut Window, _cx: &mut Context<ListState<Self>>, ) {}
}

#[derive(Debug)]
pub struct NaviView {
    screen: Screen,
    mode: NaviMode,
    chooser: Option<ChooserState>,
    pub current_dir: PathBuf,
    pub file_list: Entity<ListState<FileBrowserDelegate>>,
    pub nav_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl NaviView {
    pub fn new(config: NaviConfig, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_dir = config.start_dir;
        let items = list_files(&current_dir).into();

        let nav_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(SharedString::new(current_dir.to_string_lossy()))
        });

        let file_list = cx.new(|cx| {
            ListState::new(FileBrowserDelegate::with_items(items), window, cx)
                .selectable(false)
        });

        let chooser = match (config.mode, config.chooser) {
            (NaviMode::FileChooser, Some(launch)) => {
                Some(ChooserState::new(launch.request, launch.tx))
            }
            (NaviMode::FileChooser, None) => {
                None
            }
            _ => None,
        };

        let subscriptions = vec![
            cx.subscribe_in(&nav_input, window, |view, state, event, window, cx| {
                if let InputEvent::PressEnter { .. } = event {
                    let path: PathBuf = state.read(cx).value().to_string().into();
                    view.open_path(path, cx, window);
                }
            }),

            cx.subscribe_in(&file_list, window, |view, _state, event: &FileListEvent, window, cx| {
                match event {
                    FileListEvent::Open(path) => view.open_path(path.clone(), cx, window),
                }
            }),
        ];

        Self {
            screen: Screen::Browser,
            mode: config.mode,
            chooser,
            current_dir,
            file_list,
            nav_input,
            _subscriptions: subscriptions,
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
                NaviMode::FileChooser => self.submit_selection(path, window),
            }
        }
    }

    fn set_directory(&mut self, path: PathBuf, cx: &mut Context<Self>, window: &mut Window) {
        if path == self.current_dir {
            return;
        }

        let items = list_files(&path).into();
        self.current_dir = path;

        self.file_list.update(cx, |state, _cx| {
            state.delegate_mut().files = items;
        });

        self.nav_input.update(cx, |state, cx| {
            state.set_value(
                SharedString::new(self.current_dir.to_string_lossy()),
                window,
                cx,
            );
        });

        cx.notify();
    }

    fn submit_selection(&mut self, path: PathBuf, window: &mut Window) {
        if let Some(chooser) = self.chooser.as_mut() {
            chooser.submit(ChooserResult::Selected(path));
            window.remove_window();
        }
    }

    pub fn _cancel_selection(&mut self, window: &mut Window) {
        if let Some(chooser) = self.chooser.as_mut() {
            chooser.cancel();
            window.remove_window();
        }
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
}

impl Render for NaviView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(match self.screen {
                Screen::Browser => browser(self, cx),
            })
    }
}

pub fn run() {
    crate::bootstrap::run_with_config(NaviConfig::default());
}

pub async fn run_file_chooser(request: ChooserRequest) -> Result<ChooserResult> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    crate::bootstrap::run_with_config(NaviConfig {
        mode: NaviMode::FileChooser,
        title: "Open File".into(),
        app_id: "navi-file-chooser".to_string(),
        chooser: Some(ChooserLaunch { request, tx }),
        ..Default::default()
    });

    let result = rx
        .await
        .map_err(|e| anyhow::anyhow!("chooser channel closed before result: {e}"))?;

    Ok(result)
}
use std::path::PathBuf;
use std::process::Command;
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::{IndexPath, Root, Theme, ThemeRegistry};
use gpui_component::input::{InputEvent};
use gpui_component::list::{ListDelegate, ListItem, ListState};
use gpui_platform::application;
use navi_core::list_files;
use crate::components::file_item::file_item;
use crate::screens::browser::browser;

#[derive(Debug, Default)]
enum Screens {
    #[default]
    Browser,
}

#[derive(Debug, Clone)]
pub struct FileList {
    pub items: Vec<PathBuf>,
    pub selected_index: Option<IndexPath>,

    pub dragging_index: Option<IndexPath>,
    pub drag_over_index: Option<IndexPath>,

    pub is_dragging: bool,
}

#[derive(Clone, Debug)]
pub enum FileListEvent {
    Open(PathBuf),
    Move { from: IndexPath, to: IndexPath },
    ClearDrag,
}

impl EventEmitter<FileListEvent> for ListState<FileList> {}

impl FileList {
    pub fn move_item(&mut self, from: usize, to: usize) {
        if from == to || from >= self.items.len() || to >= self.items.len() {
            return;
        }

        let item = self.items.remove(from);
        self.items.insert(to, item);
    }
}

impl ListDelegate for FileList {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.items.len()
    }

    fn render_item(&mut self, ix: IndexPath, _window: &mut Window, cx: &mut Context<ListState<Self>>) -> Option<ListItem> {
        self.items.get(ix.row).cloned().map(|item| {
            file_item(self, ix, item, cx)
        })
    }

    fn set_selected_index(&mut self, ix: Option<IndexPath>, _window: &mut Window, cx: &mut Context<ListState<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }
}

#[derive(Debug)]
pub struct NaviView {
    screen: Screens,
    pub current_dir: PathBuf,

    pub file_list: Entity<ListState<FileList>>,
    pub nav_input: Entity<InputState>,

    _subscriptions: Vec<Subscription>,
}

impl NaviView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let paths = list_files(&current_dir).into();

        let nav_input = cx.new(| cx|
            InputState::new(window, cx)
                .default_value(SharedString::new(current_dir.to_string_lossy()))
        );

        let file_list = FileList {
            items: paths,
            selected_index: None,
            dragging_index: None,
            drag_over_index: None,
            is_dragging: false,
        };

        let file_list = cx.new(|cx| ListState::new(file_list, window, cx));

        let _subscriptions = vec![
            cx.subscribe_in(&nav_input, window, |view, state, event, window, cx| {
                match event {
                    InputEvent::PressEnter { .. } => {
                        let path: PathBuf = state.read(cx).value().to_string().into();

                        view.open_path(path, cx, window);
                    }
                    _ => {}
                }
            }),

            cx.subscribe_in(&file_list, window, |view, state, event: &FileListEvent, window, cx| {
                match event {
                    FileListEvent::Open(path) => {
                        view.open_path(path.to_path_buf(), cx, window);
                    },

                    FileListEvent::Move { from, to } => {
                        let from_ix = from.row;
                        let to_ix = to.row;

                        state.update(cx, |state, _cx| {
                            state.delegate_mut().move_item(from_ix, to_ix);
                        })
                    },

                    FileListEvent::ClearDrag => {
                        state.update(cx, |state, _cx| {
                            state.delegate_mut().dragging_index = None;
                            state.delegate_mut().drag_over_index = None;
                        })
                    }
                }
            })
        ];

        Self {
            screen: Screens::Browser,
            file_list,
            current_dir,
            nav_input,
            _subscriptions,
        }
    }

    pub fn open_path(&mut self, path: PathBuf, cx: &mut Context<Self>, window: &mut Window) {
        if path.is_dir() && path != self.current_dir {
            let paths = list_files(&path).into();
            self.current_dir = path;

            self.file_list.update(cx, |state, _cx| {
               state.delegate_mut().items = paths;
            });

            self.nav_input.update(cx, |state, cx| {
                state.set_value(SharedString::new(self.current_dir.to_string_lossy()), window, cx);
            });

            cx.notify();
        } else if path.is_file() {
            if let Err(err) = Command::new("xdg-open").arg(&path).spawn() {
                eprintln!("failed to open path {}: {err}", path.display());
            }
        }
    }
}

impl Render for NaviView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(
                match self.screen {
                    Screens::Browser => browser(self, cx),
                }
            )
    }
}

pub fn run() {
    application().run(|cx| {
        gpui_component::init(cx);

        let theme_name = SharedString::from("Ayu Dark");

        if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./crates/libs/navi-ui/src/themes"), cx, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get(&theme_name)
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
        }) {
            eprintln!("{}", err)
        };

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Navi".to_string())),
                    ..Default::default()
                }),
                app_id: Some("navi".to_string()),
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                cx.new(|cx| Root::new(cx.new(|_cx| NaviView::new(window, _cx)), window, cx))
            }).expect("failed to open window");
        }).detach()
    });
}
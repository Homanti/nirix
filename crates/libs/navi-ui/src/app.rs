use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::sync::Arc;
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::{Root, Theme, ThemeConfig, ThemeMode};
use gpui_component::input::{InputEvent};
use gpui_platform::application;
use navi_core::list_files;
use crate::screens::browser::browser;

#[derive(Debug, Default)]
enum Screens {
    #[default]
    Browser,
}

#[derive(Debug, Clone)]
pub struct FileList {
    pub list_state: ListState,
    pub items: Arc<[PathBuf]>,
    pub selected_index: Option<usize>,
}

#[derive(Debug)]
pub struct NaviView {
    screen: Screens,
    pub navi_view: WeakEntity<NaviView>,
    pub current_dir: PathBuf,

    pub file_list: FileList,
    pub nav_input: Entity<InputState>,

    _subscription: Subscription,
}

impl NaviView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let paths: Arc<[PathBuf]> = list_files(&current_dir).into();

        let nav_input = cx.new(| cx|
            InputState::new(window, cx)
                .default_value(SharedString::new(current_dir.to_string_lossy()))
        );

        let file_list = FileList {
            list_state: ListState::new(paths.len(), ListAlignment::Top, px(500.)).measure_all(),
            items: paths,
            selected_index: None,
        };

        let _subscription = cx.subscribe_in(&nav_input, window, |view, state, event, window, cx| {
            match event {
                InputEvent::PressEnter { .. } => {
                    let path: PathBuf = state.read(cx).value().to_string().into();

                    view.open_path(path, cx, window);
                }
                _ => {}
            }
        });

        Self {
            screen: Screens::Browser,
            navi_view: WeakEntity::new_invalid(),
            file_list,
            current_dir,
            nav_input,
            _subscription,
        }
    }

    pub fn open_path(&mut self, path: PathBuf, cx: &mut Context<Self>, window: &mut Window) {
        if path.is_dir() && path != self.current_dir {
            let paths: Arc<[PathBuf]> = list_files(&path).into();
            self.current_dir = path;

            self.file_list = FileList {
                list_state: ListState::new(paths.len(), ListAlignment::Top, px(500.)).measure_all(),
                items: paths,
                selected_index: None,
            };

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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let view = self.navi_view.clone();

        div()
            .bg(rgb(0x000000))
            .size_full()
            .child(
                match self.screen {
                    Screens::Browser => browser(self, view, _cx),
                }
            )
    }
}

pub fn run() {
    application().run(|cx| {
        gpui_component::init(cx);

        let theme = Theme::global_mut(cx);

        theme.apply_config(
            &Rc::new(ThemeConfig {
                mode: ThemeMode::Dark,
                ..Default::default()
            })
        );

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
                let view = cx.new(|_cx| NaviView::new(window, _cx));

                view.update(cx, |this, _cx| {
                    this.navi_view = view.downgrade();
                });

                cx.new(|cx| Root::new(view, window, cx))
            }).expect("failed to open window");
        }).detach()
    });
}
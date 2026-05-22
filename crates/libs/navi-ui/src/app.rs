use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use gpui::*;
use gpui_component::input::InputState;
use gpui_component::Root;
use gpui_component::input::{InputEvent};
use gpui_platform::application;
use navi_core::list_files;
use crate::screens::browser::browser;

#[derive(Debug, Default)]
enum Screens {
    #[default]
    Browser,
}

#[derive(Debug)]
pub struct NaviView {
    screen: Screens,
    pub navi_view: WeakEntity<NaviView>,
    pub current_dir: PathBuf,
    pub entries: Arc<[PathBuf]>,
    pub file_list_state: ListState,
    pub nav_input: Entity<InputState>,
}

impl NaviView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        let entries: Arc<[PathBuf]> = list_files(&current_dir).into();
        let nav_input = cx.new(| cx|
            InputState::new(window, cx)
                .default_value(SharedString::new(current_dir.to_str().unwrap()))
        );


        cx.subscribe_in(&nav_input, window, |view, state, event, window, cx| {
            match event {
                InputEvent::PressEnter { .. } => {
                    let path: PathBuf = state.read(cx).value().to_string().into();

                    view.open_path(path, cx, window);
                }
                _ => {}
            }
        }).detach();

        Self {
            screen: Screens::Browser,
            navi_view: WeakEntity::new_invalid(),
            file_list_state: ListState::new(entries.len(), ListAlignment::Top, px(500.)).measure_all(),
            current_dir,
            entries,
            nav_input
        }
    }
    fn refresh(&mut self, cx: &mut Context<Self>, window: &mut Window) {
        self.entries = list_files(&self.current_dir).into();
        self.file_list_state = ListState::new(self.entries.len(), ListAlignment::Top, px(500.)).measure_all();

        self.nav_input.update(cx, |state, cx| {
            state.set_value(SharedString::new(self.current_dir.to_str().unwrap()), window, cx);
        });

        cx.notify();
    }

    pub fn open_path(&mut self, path: PathBuf, cx: &mut Context<Self>, window: &mut Window) {
        if path.is_dir() && path != self.current_dir {
            self.current_dir = path;
            self.refresh(cx, window);
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
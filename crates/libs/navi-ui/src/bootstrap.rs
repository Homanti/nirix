use std::path::PathBuf;

use gpui::*;
use gpui_component::{Root, Theme, ThemeRegistry};
use gpui_platform::application;

use crate::app::NaviView;
use crate::config::NaviConfig;

pub fn run_with_config(config: NaviConfig) {
    application().run(move |cx| {
        gpui_component::init(cx);
        init_theme(cx);
        
        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(config.title.clone()),
                    ..Default::default()
                }),
                app_id: Some(config.app_id.clone()),
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                cx.new(|cx| {
                    Root::new(cx.new(|cx| NaviView::new(config, window, cx)), window, cx)
                })
            })
                .expect("failed to open window");
        })
            .detach();
    });
}

fn init_theme(cx: &mut App) {
    let theme_name = SharedString::from("Ayu Dark");

    if let Err(err) = ThemeRegistry::watch_dir(
        PathBuf::from("./crates/libs/navi-ui/src/themes"),
        cx,
        move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get(&theme_name)
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
        },
    ) {
        eprintln!("{err}");
    }
}
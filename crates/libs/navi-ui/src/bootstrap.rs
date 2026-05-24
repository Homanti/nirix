use std::path::PathBuf;

use gpui::*;
use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui_component::{Root, Theme, ThemeRegistry};
use gpui_platform::application;
use gpui::layer_shell::Anchor;

use crate::app::NaviView;
use crate::config::{NaviConfig, NaviMode};

pub fn run_with_config(config: NaviConfig) {
    application().run(move |cx| {
        gpui_component::init(cx);
        init_theme(cx);
        
        let displays = cx.displays();
        let display = displays.get(0);

        let display_size = display
            .and_then(|id| cx.find_display(id.id()))
            .or_else(|| cx.primary_display())
            .map(|display| display.bounds().size)
            .unwrap_or_else(|| Size::new(px(1920.), px(1080.)));

        cx.spawn(async move |cx| {
            let is_chooser = config.mode == NaviMode::FileChooser;

            let window_options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(config.title.clone().into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: Size::new(px(500.), px(200.)),
                })),
                app_id: Some(config.app_id.clone()),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: if is_chooser {
                    WindowKind::LayerShell(LayerShellOptions {
                        namespace: Some(config.title.clone().to_string()).unwrap_or("navi".to_string()),
                        anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP | Anchor::BOTTOM,
                        margin: Some((px(display_size.height.as_f32() / 3f32), px(display_size.width.as_f32() / 3f32), px(display_size.height.as_f32() / 3f32), px(display_size.width.as_f32() / 3f32))),
                        keyboard_interactivity: KeyboardInteractivity::OnDemand,
                        ..Default::default()
                    })
                } else {
                    WindowKind::Normal
                },
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                cx.new(|cx| {
                    Root::new(cx.new(|cx| NaviView::new(config, window, cx)), window, cx)
                        .size_full()
                        .window_shadow_size(px(0.))
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
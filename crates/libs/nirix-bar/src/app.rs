use std::env::home_dir;
use std::path::PathBuf;
use gpui::*;
use gpui::layer_shell::{Anchor, KeyboardInteractivity, LayerShellOptions};

const BAR_HEIGHT: f32 = 30.;

struct Bar;

impl Bar {
    pub fn new(_cx: &mut App) -> Self {
        Self
    }
}

impl Render for Bar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgba(0x00000055))
    }
}


pub fn init(cx: &mut App) {
    let window_options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: point(px(0.), px(0.)),
            size: Size::new(px(0.), px(BAR_HEIGHT)),
        })),
        kind: WindowKind::LayerShell(
            LayerShellOptions {
                anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
                exclusive_zone: Some(px(BAR_HEIGHT)),
                keyboard_interactivity: KeyboardInteractivity::None,
                ..Default::default()
            }
        ),
        ..Default::default()
    };

    let _ = cx.open_window(window_options,|window, cx| {
        cx.new(|cx| Bar::new(cx))
    });
}
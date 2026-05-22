use gpui::*;

struct NaviView;

impl Render for NaviView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child("future best file manager")
    }
}

pub fn run() {
    Application::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| NaviView)
        })
            .expect("failed to open window");
    });
}
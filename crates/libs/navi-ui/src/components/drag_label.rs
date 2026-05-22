use gpui::*;
use gpui_component::IndexPath;

pub struct DragLabel {
    pub ix: IndexPath,
}

impl DragLabel {
    pub fn new(ix: IndexPath) -> Self {
        Self { ix }
    }
}

impl Render for DragLabel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_3()
            .py_1()
            .rounded_md()
            .bg(rgb(0x2f3340))
            .text_color(rgb(0xffffff))
            .child(self.ix.clone().to_string())
    }
}
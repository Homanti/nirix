use gpui::*;

pub fn file_item(label: String) -> Stateful<Div> {
    div()
        .w_full()
        .hover(|style| style.bg(rgb(0x505050)))
        .p_2()
        .id(label.to_string())
        .cursor_pointer()
        .text_color(rgb(0xffffff))
        .child(label)
}
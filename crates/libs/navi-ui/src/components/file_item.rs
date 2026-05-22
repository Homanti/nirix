use std::path::PathBuf;
use gpui::*;
use crate::app::NaviView;

pub fn file_item(index: usize, path: PathBuf, is_selected: bool, view: WeakEntity<NaviView>) -> Stateful<Div> {
    let label = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("/")
        .to_string();

    div()
        .id(format!("{}-{}", index, label))
        .w_full()
        .px_2()
        .child(
            div()
                .id(format!("{}-{}", index, label))
                .cursor_pointer()
                .text_color(rgb(0xffffff))
                .px_2()
                .py_1()
                .on_click(move |event, window, app| {
                    if event.click_count() == 2 {
                        let _ = view.update(app, |this, cx| {
                            this.open_path(path.clone(), cx, window);
                        });
                    } else {
                        let _ = view.update(app, |this, _cx| {
                            this.file_list.selected_index = Some(index);
                        });
                    }
                })
                .w_full()
                .hover(|style| if !is_selected {style.bg(rgb(0x505050))} else {style.bg(rgba(0x00000000))})
                .border_2()
                .rounded(px(8.))
                .border_color(if is_selected {rgb(0x505050)} else {rgba(0x00000000)})
                .child(label)
        )
}
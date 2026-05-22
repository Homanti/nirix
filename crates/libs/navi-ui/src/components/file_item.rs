use std::path::PathBuf;
use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{IndexPath};
use gpui_component::label::Label;
use gpui_component::list::{ListItem, ListState};
use crate::app::{FileList, FileListEvent};
use crate::components::drag_label::DragLabel;

pub fn file_item(
    state: &mut FileList,
    ix: IndexPath,
    item: PathBuf,
    cx: &mut Context<ListState<FileList>>,
) -> ListItem {
    let label = item
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("/")
        .to_string();
    
    let is_dragged = state.dragging_index.as_ref() == Some(&ix);
    let is_drag_over = state.drag_over_index.as_ref() == Some(&ix);

    ListItem::new(ix.clone())
        .selected(Some(ix.clone()) == state.selected_index)
        .py_2()
        .opacity(if is_dragged { 0.45 } else { 1.0 })
        .child(
            div()
                .id(format!("{label}-{ix}"))
                .size_full()
                .when(is_drag_over, |this| this.bg(rgb(0x2f3340)))
                .on_click(cx.listener({
                    let item = item.clone();
                    move |_view, event: &ClickEvent, _window, cx| {
                        if event.click_count() == 2 {
                            cx.emit(FileListEvent::Open(item.clone()));
                        }
                    }
                }))
                .on_drag(state.clone(), move |_dragged_data, _offset, _window, cx| {
                    cx.new(|_| {
                        DragLabel {
                            ix: ix,
                        }
                    })
                })
                .child(Label::new(label)),
        )
}
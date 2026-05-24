use std::path::PathBuf;
use gpui::*;
use gpui_component::{ActiveTheme, IndexPath};
use gpui_component::label::Label;
use gpui_component::list::{ListItem, ListState};
use crate::app::{FileBrowserDelegate, FileListEvent};

pub fn file_item(
    state: &mut FileBrowserDelegate,
    ix: IndexPath,
    item: PathBuf,
    cx: &mut Context<ListState<FileBrowserDelegate>>,
) -> ListItem {
    let label = item
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("/")
        .to_string();

    let is_selected = state.selected_index.contains(&Some(ix));

    ListItem::new(ix.clone())
        .py_2()
        .border_1()
        .bg(if is_selected {cx.theme().list_active} else { Default::default() })
        .border_color(if is_selected {cx.theme().list_active_border} else { Default::default() })
        .selected(is_selected)
        .confirmed(is_selected)
        .on_click(cx.listener({
            let item = item.clone();
            let ix = ix.clone();
            move |view, event: &ClickEvent, _window, cx| {
                let modifiers = event.modifiers();
                let ctrl = modifiers.control;

                if event.click_count() == 2 {
                    cx.emit(FileListEvent::Open(item.clone()));
                } else if event.click_count() == 1 {
                    if ctrl {
                        FileBrowserDelegate::add_selected_index(view.delegate_mut(), Some(ix));
                    } else {
                        FileBrowserDelegate::set_selected_index(view.delegate_mut(), Some(ix));
                    }

                    cx.notify();
                }
            }
        }))
        .child(Label::new(label))
}
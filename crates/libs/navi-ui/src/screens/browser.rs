use gpui::*;
use crate::app::NaviView;
use crate::components::file_item::file_item;
use crate::components::navbar::navbar;

pub fn browser(
    state: &mut NaviView,
    view: WeakEntity<NaviView>,
    _cx: &mut Context<NaviView>,
) -> impl IntoElement {
    let items = state.file_list.items.clone();
    let selected_index = state.file_list.selected_index;

    div().size_full()
        .child(
            navbar(state)
        )
        .child(
            list(state.file_list.list_state.clone(), move |index, _window, _app| {
                let path = items[index].clone();
                let view = view.clone();
                let is_selected = Some(index) == selected_index;

                div()
                    .w_full()
                    .child(
                        file_item(index, path, is_selected, view)
                    )
                    .into_any()

            }).size_full()
        )
}
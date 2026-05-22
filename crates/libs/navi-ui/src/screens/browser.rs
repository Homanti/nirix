use gpui::*;
use crate::app::NaviView;
use crate::components::file_item::file_item;
use crate::components::navbar::navbar;

pub fn browser(
    state: &mut NaviView,
    view: WeakEntity<NaviView>,
    _cx: &mut Context<NaviView>,
) -> impl IntoElement {
    let items = state.entries.clone();

    div().size_full()
        .child(
            navbar(state)
        )
        .child(
            list(state.file_list_state.clone(), move |index, _window, _app| {
                let path = items[index].clone();
                let label = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("/")
                    .to_string();
                let view = view.clone();

                div()
                    .w_full()
                    .child(
                        file_item(label)
                            .on_mouse_up(MouseButton::Left, move |_event, window, app| {
                                let _ = view.update(app, |this, cx| {
                                    this.open_path(path.clone(), cx, window);
                                });
                            })
                    )
                    .into_any()

            }).size_full()
        )
}
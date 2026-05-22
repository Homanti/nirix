use gpui::*;
use gpui_component::button::{Button};
use gpui_component::input::{Input};
use crate::app::NaviView;

pub fn navbar(state: &mut NaviView, cx: &mut Context<NaviView>) -> Div {
    let current_dir = state.current_dir.clone();

    div()
        .flex()
        .flex_row()
        .child(
            Button::new("back")
                .label("back")
                .rounded(px(0.0))
                .on_click(cx.listener(move |view, _event, window, cx| {
                    if let Some(parent) = current_dir.parent() {
                        view.open_path(parent.to_path_buf(), cx, window);
                    }
                })),
        )
        .child(
            Input::new(&state.nav_input)
                .rounded(px(0.0))
        )
}
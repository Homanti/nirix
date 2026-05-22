use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input};
use crate::app::NaviView;

pub fn navbar(state: &mut NaviView) -> Div {
    let view = state.navi_view.clone();
    let current_dir = state.current_dir.clone();

    div()
        .flex()
        .flex_row()
        .child(
            Button::new("back")
                .primary()
                .label("back")
                .on_click(move |_event, window, cx| {
                    if let Some(parent) = current_dir.parent() {
                        let parent = parent.to_path_buf();

                        let _ = view.update(cx, |this, cx| {
                            this.open_path(parent, cx, window);
                        });
                    }
                }),
        )
        .child(
            Input::new(&state.nav_input)
                .bg(rgb(0x000000))
                .text_color(rgb(0xffffff))
                .border_0()
                .rounded(px(0.0))
        )
}
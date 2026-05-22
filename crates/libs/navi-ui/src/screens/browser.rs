use gpui::*;
use gpui_component::list::List;
use crate::app::NaviView;
use crate::components::navbar::navbar;

pub fn browser(
    state: &mut NaviView,
    cx: &mut Context<NaviView>,
) -> impl IntoElement {

    div().size_full()
        .child(
            navbar(state, cx)
        )
        .child(
            List::new(&state.file_list)
        ).size_full()
}
use std::path::PathBuf;
use gpui_component::input::{Input, InputState};
use gpui::*;
use gpui_component::button::Button;
use gpui_component::input::InputEvent;
use crate::screens::browser::Browser;

pub struct NavBar {
    pub nav_input: Entity<InputState>,
    browser: WeakEntity<Browser>,
}

pub enum NavBarEvent {
    Navigate(PathBuf),
}

impl EventEmitter<NavBarEvent> for NavBar {}

impl NavBar {
    pub fn new(directory: &std::path::Path, browser: WeakEntity<Browser>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let nav_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(SharedString::new(directory.to_string_lossy()))
        });

        cx.subscribe_in(&nav_input, window,move |_view, state, event, _window, cx| {
            if let InputEvent::PressEnter { .. } = event {
                let path: PathBuf = state.read(cx).value().to_string().into();
                cx.emit(NavBarEvent::Navigate(path));
            }
        }).detach();

        Self { nav_input, browser }
    }
}

impl Render for NavBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .child(
                Button::new("back")
                    .label("back")
                    .rounded(px(0.0))
                    .on_click(cx.listener(|view, _event, _window, cx| {
                        let path = view.browser.update(cx, |view, _cx| {
                            view.directory.clone()
                        });

                        if let Ok(path) = path {
                            if let Some(parent) = path.parent() {
                                cx.emit(NavBarEvent::Navigate(parent.into()))
                            }
                        }
                    }))
            )
            .child(
                Input::new(&self.nav_input)
                    .rounded(px(0.0))
            )
    }
}
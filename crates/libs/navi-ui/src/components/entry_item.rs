use std::collections::HashSet;
use std::path::PathBuf;
use gpui::*;
use gpui_component::ActiveTheme;
use gpui_component::label::Label;
use crate::screens::browser::Browser;

#[derive(IntoElement)]
pub struct EntryItem {
    path: PathBuf,
    index: usize,
    browser: WeakEntity<Browser>,
}

impl EntryItem {
    pub fn new(path: PathBuf, index: usize, browser: WeakEntity<Browser>) -> Self {
        Self {
            path, index, browser
        }
    }
}

fn get_range(from: usize, to: usize) -> HashSet<usize> {
    let start = from.min(to);
    let end   = from.max(to);
    (start..=end).collect()
}

impl RenderOnce for EntryItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let path = self.path;
        let index = self.index;
        let browser = self.browser;

        let label = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .into_owned();

        let (is_active, prev_active, next_active) = browser.update(cx, |view, _cx| {
            let is_active = view.selected_entries.contains(&index);
            let prev_active = index > 0 && view.selected_entries.contains(&(index - 1));
            let next_active = view.selected_entries.contains(&(index + 1));

            (is_active, prev_active, next_active)
        }).unwrap_or((false, false, false));

        div()
            .id(format!("{}-{}", path.display(), index))
            .flex()
            .flex_row()
            .w_full()
            .gap_x_1()
            .py_2()
            .px_3()
            .text_color(cx.theme().foreground)
            .items_center()
            .justify_start()
            .on_click(move |event, window, cx| {
                if event.click_count() == 1 {
                    let ctrl = event.modifiers().control;
                    let shift = event.modifiers().shift;

                    let _ = browser.update(cx, |view, _cx| {
                        match (ctrl, shift, view.first_selected) {
                            (true, _, _) => {
                                if view.selected_entries.contains(&index) {
                                    view.selected_entries.remove(&index);
                                } else {
                                    view.selected_entries.insert(index);
                                }
                            }
                            (false, true, Some(first)) => {
                                view.selected_entries = get_range(first, index);
                            }
                            (false, true, None) | (false, false, _) => {
                                view.selected_entries.clear();
                                view.selected_entries.insert(index);
                                view.first_selected = Some(index);
                            }
                        }
                    });
                } else if event.click_count() == 2 {
                    let path = path.clone();
                    let _ = browser
                        .update(cx, |view, cx| {
                            view.open_path(path, cx, window);
                        });
                }
            })

            .child(Label::new(label))
            .bg(if is_active {cx.theme().list_active} else { Default::default() })
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .bottom_0()
                    .border(AbsoluteLength::Pixels(px(1.)))
                    .border_t(if is_active && prev_active {
                        AbsoluteLength::Pixels(px(0.))
                    } else {
                        AbsoluteLength::Pixels(px(1.))
                    })
                    .border_b(if is_active && next_active {
                        AbsoluteLength::Pixels(px(0.))
                    } else {
                        AbsoluteLength::Pixels(px(1.))
                    })
                    .border_color(if is_active {
                        cx.theme().list_active_border
                    } else {
                        Default::default()
                    })
            )
    }
}
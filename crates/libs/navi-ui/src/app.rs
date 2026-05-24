use gpui::*;
use gpui::private::anyhow;
use crate::chooser::{ChooserLaunch, ChooserRequest, ChooserResult, ChooserState};
use crate::config::{NaviConfig, NaviMode};
use crate::screens::browser::Browser;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Screen {
    #[default]
    Browser,
}

#[derive(Debug)]
pub struct NaviView {
    screen: Screen,
    _mode: NaviMode,
    pub browser: Entity<Browser>,
}

impl NaviView {
    pub fn new(config: NaviConfig, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let directory = config.start_dir;

        let chooser = match (config.mode, config.chooser) {
            (NaviMode::FileChooser, Some(launch)) => {
                Some(ChooserState::new(launch.request, launch.tx))
            }
            (NaviMode::FileChooser, None) => {
                None
            }
            _ => None,
        };

        let browser = cx.new(|cx| {
            Browser::new(&directory, &config.mode, chooser, window, cx)
        });

        Self {
            screen: Screen::Browser,
            _mode: config.mode,
            browser,
        }
    }
}

impl Render for NaviView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()
            .relative()
            .child(
                match self.screen {
                    Screen::Browser => self.browser.clone(),
                }
            )
    }
}

pub fn run() {
    crate::bootstrap::run_with_config(NaviConfig::default());
}

pub async fn run_file_chooser(request: ChooserRequest) -> Result<ChooserResult> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    let thread = std::thread::spawn(move || {
        crate::bootstrap::run_with_config(NaviConfig {
            mode: NaviMode::FileChooser,
            title: "Open File".into(),
            app_id: "navi-file-chooser".to_string(),
            chooser: Some(ChooserLaunch { request, tx }),
            ..Default::default()
        });
    });

    let result = rx
        .await
        .map_err(|e| anyhow::anyhow!("chooser channel closed before result: {e}"))?;

    thread
        .join()
        .map_err(|_| anyhow::anyhow!("gpui thread panicked"))?;

    Ok(result)
}
use std::path::PathBuf;
use gpui::SharedString;
use crate::chooser::ChooserLaunch;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NaviMode {
    #[default]
    Browser,
    FileChooser,
}

#[derive(Debug)]
pub struct NaviConfig {
    pub mode: NaviMode,
    pub title: SharedString,
    pub app_id: String,
    pub start_dir: PathBuf,
    pub chooser: Option<ChooserLaunch>,
}

impl Default for NaviConfig {
    fn default() -> Self {
        Self {
            mode: NaviMode::Browser,
            title: SharedString::from("Navi"),
            app_id: "navi".to_string(),
            start_dir: dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
            chooser: None,
        }
    }
}
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use async_process::Command;
use zbus::interface;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use crate::file_chooser::request::parse_chooser_request;
use crate::file_chooser::response::response_from_chooser;
use navi_ui::ChooserResult;

pub struct FileChooserBackend;

impl FileChooserBackend {
    pub fn new() -> Self {
        Self
    }
}

struct RequestContext {
    _handle: OwnedObjectPath,
    _app_id: String,
    _parent_window: String,
}

#[interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooserBackend {
    async fn open_file(
        &self,
        handle: OwnedObjectPath,
        app_id: String,
        parent_window: String,
        title: String,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)> {
        let _ctx = RequestContext {
            _handle: handle,
            _app_id: app_id,
            _parent_window: parent_window,
        };

        let request = parse_chooser_request(title, &options).map_err(|e| {
            zbus::fdo::Error::InvalidArgs(format!("invalid file chooser request: {e}"))
        })?;

        let mut ui_binary = env::current_exe().unwrap_or_else(|_| PathBuf::from("navi-chooser"));
        ui_binary.set_file_name("navi-chooser");

        let mut cmd = Command::new(&ui_binary);
        cmd.arg("--title").arg(&request.title);

        if request.multiple {
            cmd.arg("--multiple");
        }
        if request.directory {
            cmd.arg("--directory");
        }

        let output = cmd.output().await.map_err(|e| {
            zbus::fdo::Error::Failed(format!("failed to spawn navi-chooser: {e}"))
        })?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

            if lines.is_empty() {
                return Ok((1, HashMap::new()));
            }

            let result = if request.multiple {
                let paths = lines.into_iter().map(PathBuf::from).collect();
                ChooserResult::SelectedMany(paths)
            } else {
                ChooserResult::Selected(PathBuf::from(lines[0]))
            };

            response_from_chooser(result)
        } else {
            Ok((1, HashMap::new()))
        }
    }
}
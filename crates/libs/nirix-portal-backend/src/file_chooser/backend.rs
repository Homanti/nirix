use std::collections::HashMap;

use zbus::interface;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use crate::file_chooser::request::parse_chooser_request;
use crate::file_chooser::response::response_from_chooser;

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

        let result = navi_ui::run_file_chooser(request)
            .await
            .map_err(|e| zbus::fdo::Error::Failed(format!("file chooser failed: {e}")))?;

        response_from_chooser(result)
    }
}
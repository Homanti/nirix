use std::collections::HashMap;
use zbus::interface;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use nirix_portal_core::{parse_chooser_request, response_from_chooser};

pub struct FileChooserBackend;

impl FileChooserBackend {
    pub fn new() -> Self {
        Self
    }
}

#[interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooserBackend {
    async fn open_file(
        &self,
        _handle: OwnedObjectPath,
        _app_id: String,
        _parent_window: String,
        title: String,
        options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)> {
        let request = parse_chooser_request(title, &options)?;

        let result = navi_ui::run_file_chooser(request)
            .await
            .map_err(|e| zbus::fdo::Error::Failed(format!("file chooser failed: {e}")))?;

        response_from_chooser(result)
    }
}
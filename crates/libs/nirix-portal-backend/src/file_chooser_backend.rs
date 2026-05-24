use std::collections::HashMap;
use std::path::{Path, PathBuf};
use zbus::interface;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

use navi_ui::{ChooserRequest, ChooserResult};

fn get_bool_option(
    options: &HashMap<String, OwnedValue>,
    key: &str,
) -> zbus::fdo::Result<Option<bool>> {
    let Some(value) = options.get(key) else {
        return Ok(None);
    };

    match <bool>::try_from(value.clone()) {
        Ok(v) => Ok(Some(v)),
        Err(err) => Err(zbus::fdo::Error::Failed(format!(
            "option `{key}` is not a bool: {err}"
        ))),
    }
}

fn path_to_file_uri(path: &Path) -> zbus::fdo::Result<String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::fs::canonicalize(path).map_err(|e| {
            zbus::fdo::Error::Failed(format!("failed to canonicalize {}: {e}", path.display()))
        })?
    };

    let s = absolute.to_string_lossy();

    if !s.starts_with('/') {
        return Err(zbus::fdo::Error::Failed(format!(
            "path is not a unix absolute path: {}",
            absolute.display()
        )));
    }

    Ok(format!("file://{}", s))
}

fn encode_uris<I>(paths: I) -> zbus::fdo::Result<OwnedValue>
where
    I: IntoIterator<Item = PathBuf>,
{
    let uris: Vec<String> = paths
        .into_iter()
        .map(|path| path_to_file_uri(&path))
        .collect::<zbus::fdo::Result<_>>()?;

    Value::from(uris)
        .try_into()
        .map_err(|e| zbus::fdo::Error::Failed(format!("failed to encode uris: {e}")))
}

fn response_from_chooser(
    result: ChooserResult,
) -> zbus::fdo::Result<(u32, HashMap<String, OwnedValue>)> {

    match result {
        ChooserResult::Cancelled => Ok((1, HashMap::new())),
        ChooserResult::Selected(path) => {
            let mut results = HashMap::new();
            results.insert("uris".to_string(), encode_uris([path])?);
            Ok((0, results))
        }
        ChooserResult::SelectedMany(paths) => {
            let mut results = HashMap::new();
            results.insert("uris".to_string(), encode_uris(paths)?);
            Ok((0, results))
        }
    }
}

fn parse_chooser_request(
    title: String,
    options: &HashMap<String, OwnedValue>,
) -> zbus::fdo::Result<ChooserRequest> {

    Ok(ChooserRequest {
        title,
        multiple: get_bool_option(options, "multiple")?.unwrap_or(false),
        directory: get_bool_option(options, "directory")?.unwrap_or(false),
    })
}

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
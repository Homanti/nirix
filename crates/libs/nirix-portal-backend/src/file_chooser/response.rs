use std::collections::HashMap;
use std::path::{Path, PathBuf};

use zbus::zvariant::{OwnedValue, Value};

use navi_ui::ChooserResult;

pub fn path_to_file_uri(path: &Path) -> zbus::fdo::Result<String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::fs::canonicalize(path).map_err(|e| {
            zbus::fdo::Error::Failed(format!(
                "failed to canonicalize {}: {e}",
                path.display()
            ))
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

pub fn encode_uris<I>(paths: I) -> zbus::fdo::Result<OwnedValue>
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

pub fn response_from_chooser(
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
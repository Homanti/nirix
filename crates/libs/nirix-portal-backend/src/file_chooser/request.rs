use std::collections::HashMap;

use zbus::zvariant::OwnedValue;

use navi_ui::ChooserRequest;

pub fn get_bool_option(
    options: &HashMap<String, OwnedValue>,
    key: &str,
) -> zbus::fdo::Result<Option<bool>> {
    let Some(value) = options.get(key) else {
        return Ok(None);
    };

    match bool::try_from(value.clone()) {
        Ok(v) => Ok(Some(v)),
        Err(err) => Err(zbus::fdo::Error::Failed(format!(
            "option `{key}` is not a bool: {err}"
        ))),
    }
}

pub fn parse_chooser_request(
    title: String,
    options: &HashMap<String, OwnedValue>,
) -> zbus::fdo::Result<ChooserRequest> {
    Ok(ChooserRequest {
        title,
        multiple: get_bool_option(options, "multiple")?.unwrap_or(false),
        directory: get_bool_option(options, "directory")?.unwrap_or(false),
    })
}
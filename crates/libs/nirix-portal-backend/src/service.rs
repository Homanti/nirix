use crate::file_chooser_backend::FileChooserBackend;

pub async fn run() -> anyhow::Result<()> {
    let file_chooser = FileChooserBackend::new();

    let _conn = zbus::connection::Builder::session()?
        .name("org.freedesktop.impl.portal.desktop.nirix")?
        .serve_at("/org/freedesktop/portal/desktop", file_chooser)?
        .build()
        .await?;

    std::future::pending::<()>().await;
    Ok(())
}
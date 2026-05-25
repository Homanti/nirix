use gpui_platform::application;
use tracing::error;
use tracing_subscriber::EnvFilter;
use nirix_wallpaper::WallpaperService;

const NAMESPACE: &str = "nirix-wallpaper";

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut wallpaper = WallpaperService::new(NAMESPACE);
    if let Err(err) = wallpaper.ensure_backend() {
        error!("failed to initialize wallpaper backend: {err}");
    }

    let app = application();
    app.run(|cx| {
        nirix_bar::init(cx);
    });
}
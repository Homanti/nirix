use std::env;
use navi_ui::{run_file_chooser, ChooserRequest, ChooserResult};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut request = ChooserRequest::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--title" => {
                if i + 1 < args.len() {
                    request.title = args[i + 1].clone();
                    i += 1;
                }
            }
            "--multiple" => request.multiple = true,
            "--directory" => request.directory = true,
            _ => {}
        }
        i += 1;
    }

    match run_file_chooser(request).await {
        Ok(ChooserResult::Selected(path)) => {
            println!("{}", path.display());
            std::process::exit(0);
        }
        Ok(ChooserResult::SelectedMany(paths)) => {
            for path in paths {
                println!("{}", path.display());
            }
            std::process::exit(0);
        }
        Ok(ChooserResult::Cancelled) | Err(_) => {
            std::process::exit(1);
        }
    }
}
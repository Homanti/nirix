use std::path::PathBuf;
use tokio::sync::oneshot;
use oneshot::Sender;

#[derive(Debug, Clone)]
pub struct ChooserRequest {
    pub title: String,
    pub multiple: bool,
    pub directory: bool,
}

impl Default for ChooserRequest {
    fn default() -> Self {
        Self {
            title: String::new(),
            directory: false,
            multiple: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChooserResult {
    Cancelled,
    Selected(PathBuf),
    SelectedMany(Vec<PathBuf>),
}

#[derive(Debug)]
pub struct ChooserState {
    pub _request: ChooserRequest,
    tx: Option<Sender<ChooserResult>>,
}

#[derive(Debug)]
pub struct ChooserLaunch {
    pub request: ChooserRequest,
    pub tx: Sender<ChooserResult>,
}

impl ChooserState {
    pub fn new(
        _request: ChooserRequest,
        tx: Sender<ChooserResult>,
    ) -> Self {
        Self {
            _request,
            tx: Some(tx),
        }
    }

    pub fn submit(&mut self, result: ChooserResult) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(result);
        }
    }

    pub fn cancel(&mut self) {
        self.submit(ChooserResult::Cancelled);
    }
}
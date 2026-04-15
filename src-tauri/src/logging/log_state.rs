use std::path::Path;
use std::path::PathBuf;

pub struct LogState {
    log_path: PathBuf,
}

impl LogState {
    pub fn new(log_path: PathBuf) -> Self {
        Self { log_path }
    }

    pub fn log_path(&self) -> &Path {
        self.log_path.as_path()
    }
}

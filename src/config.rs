use std::path::PathBuf;

pub struct Config {
    pub excluded: Vec<PathBuf>,

    /// Converts `/` to `/index.html`
    pub root_to_index: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            excluded: Vec::new(),
            root_to_index: true,
        }
    }
}

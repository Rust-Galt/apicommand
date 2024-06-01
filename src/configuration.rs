use std::path::{Path, PathBuf};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
pub struct Config {
    api_root: String,
    api_key: Option<String>,
    db_path: PathBuf,
}
impl Config {
    pub fn get_api_root(&self) -> &str {
        &self.api_root
    }
    pub fn get_api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }
    pub fn get_db_path(&self) -> &Path {
        &self.db_path
    }
}

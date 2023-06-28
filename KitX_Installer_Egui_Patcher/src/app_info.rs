pub struct AppInfo {
    pub version: String,
    pub patch_file_path: Option<String>,
}

impl AppInfo {
    pub fn default() -> AppInfo {
        AppInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            patch_file_path: None,
        }
    }
}

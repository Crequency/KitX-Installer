use std::env;

pub struct AppInfo {
    pub version: String,
    pub current_directory: String,
    pub current_exe: String,
}

impl AppInfo {
    pub fn default() -> AppInfo {
        AppInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            current_directory: env::current_dir().unwrap().to_str().unwrap().to_string(),
            current_exe: env::current_exe().unwrap().to_str().unwrap().to_string(),
        }
    }
}

pub enum RunMode {
    Cli = 1,
    Gui = 0,
}

impl Clone for RunMode {
    fn clone(&self) -> Self {
        match self {
            RunMode::Cli => RunMode::Cli,
            RunMode::Gui => RunMode::Gui,
        }
    }
}

pub struct AppInfo {
    pub run_mode: Option<RunMode>,
    pub version: String,
}

impl AppInfo {
    pub fn _default() -> AppInfo {
        AppInfo {
            run_mode: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl Clone for AppInfo {
    fn clone(&self) -> Self {
        AppInfo {
            run_mode: if self.run_mode.is_none() {
                None
            } else {
                Some(self.run_mode.clone().unwrap().clone())
            },
            version: self.version.clone(),
        }
    }
}

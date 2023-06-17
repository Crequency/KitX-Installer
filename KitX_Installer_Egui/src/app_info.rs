pub enum RunMode {
    Cli = 1,
    Gui = 0,
}

pub struct AppInfo {
    pub run_mode: Option<RunMode>,
    pub version: String,
}

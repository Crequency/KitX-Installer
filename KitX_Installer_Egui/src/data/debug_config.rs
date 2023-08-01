use crate::platforms::windows::debug_config::WindowsDebugConfig;

pub struct DebugConfig {
    pub windows_debug_config: WindowsDebugConfig,
    pub install_skip_download: bool,
    pub install_skip_extract: bool,
    pub install_skip_clean: bool,
}

impl DebugConfig {
    pub fn _default() -> DebugConfig {
        DebugConfig {
            windows_debug_config: WindowsDebugConfig::_default(),
            install_skip_download: false,
            install_skip_extract: false,
            install_skip_clean: false,
        }
    }
}

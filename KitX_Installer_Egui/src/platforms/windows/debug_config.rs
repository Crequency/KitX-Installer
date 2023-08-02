pub struct WindowsDebugConfig {
    pub install_skip_folder_permission: bool,
    pub install_skip_shortcuts: bool,
    pub install_skip_registry: bool,
    pub install_skip_uninstaller: bool,
}

impl WindowsDebugConfig {
    pub fn _default() -> WindowsDebugConfig {
        WindowsDebugConfig {
            install_skip_folder_permission: false,
            install_skip_shortcuts: false,
            install_skip_registry: false,
            install_skip_uninstaller: false,
        }
    }
}

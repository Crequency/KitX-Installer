pub struct InstallConfig {
    pub installation_path: String,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub install_as_portable: bool,
    pub launch_after_install: bool,
    pub desktop_path: Option<String>,
    pub start_menu_path: Option<String>,
    pub install_progress: f32,
}

impl InstallConfig {
    pub fn default() -> InstallConfig {
        InstallConfig {
            installation_path: "C:\\Program Files\\Crequency\\KitX\\".to_string(),
            create_desktop_shortcut: false,
            create_start_menu_shortcut: true,
            install_as_portable: false,
            launch_after_install: true,
            desktop_path: None,
            start_menu_path: None,
            install_progress: 0.0,
        }
    }
}

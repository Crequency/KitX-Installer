use crate::platforms::windows::install_config::WindowsInstallConfig;

pub struct InstallConfig {
    pub installation_path: String,
    pub install_as_portable: bool,
    pub launch_after_install: bool,
    pub install_progress: f32,
    pub windows_config: WindowsInstallConfig,
}

impl InstallConfig {
    pub fn default() -> InstallConfig {
        let is_window = cfg!(target_os = "windows");

        InstallConfig {
            installation_path: if is_window {
                "C:\\Program Files\\Crequency\\KitX\\".to_string()
            } else {
                "./KitX/".to_string()
            },
            install_as_portable: false,
            launch_after_install: true,
            install_progress: 0.0,
            windows_config: WindowsInstallConfig::default(),
        }
    }
}

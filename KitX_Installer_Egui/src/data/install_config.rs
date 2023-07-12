use std::sync::mpsc;

use crate::platforms::windows::install_config::WindowsInstallConfig;

pub struct InstallConfig {
    pub installation_path: String,
    pub install_as_portable: bool,
    pub launch_after_install: bool,
    pub install_progress: f32,
    pub windows_config: WindowsInstallConfig,
    pub progress_channel_receiver: Option<mpsc::Receiver<f32>>,
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
            progress_channel_receiver: None,
        }
    }

    pub fn update_progress(&mut self) {
        if self.progress_channel_receiver.is_none() {
            return;
        }

        let progress = self.progress_channel_receiver.as_ref().unwrap().try_recv();
        self.install_progress = match progress {
            Ok(progress) => progress,
            Err(_) => self.install_progress,
        };
    }
}

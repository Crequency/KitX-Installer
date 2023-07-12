use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::data::install_config::InstallConfig;

use super::install_config::WindowsInstallConfig;

pub fn report_progress(progress: f32, progress_channel: &mpsc::Sender<f32>) {
    if progress_channel.send(progress).is_err() {
        println!("Failed to send progress.");
    }
}

pub fn install(config: &InstallConfig, progress: mpsc::Sender<f32>) {
    let config_clone = InstallConfig {
        installation_path: config.installation_path.clone(),
        install_as_portable: config.install_as_portable,
        launch_after_install: config.launch_after_install,
        install_progress: config.install_progress,
        windows_config: WindowsInstallConfig {
            create_desktop_shortcut: config.windows_config.create_desktop_shortcut,
            create_start_menu_shortcut: config.windows_config.create_start_menu_shortcut,
            desktop_path: config.windows_config.desktop_path.clone(),
            start_menu_path: config.windows_config.start_menu_path.clone(),
        },
        progress_channel_receiver: None,
    };

    let _handle = thread::spawn(move || {
        println!("Installing...");

        report_progress(0.1, &progress);

        // Increase install progress every 0.1 second.
        for i in 0..100 {
            // if config.is_cancelled() {
            //     println!("Installation cancelled.");
            //     return;
            // }

            report_progress(0.01 * (i + 1) as f32, &progress);
            thread::sleep(Duration::from_millis(100));
        }

        println!("Installation complete.");
    });
}

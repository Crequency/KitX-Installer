use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::data::install_config::InstallConfig;

use super::install_config::WindowsInstallConfig;

pub fn install(
    config: &InstallConfig,
    progress: mpsc::Sender<f32>,
    details: mpsc::Sender<String>,
    cancel: mpsc::Receiver<i32>,
) {
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
        install_details_channel_receiver: None,
        cancle_channel_sender: None,
        installation_canceled: false,
        installation_cancel_requested: false,
    };

    let _handle = thread::spawn(move || {
        let mut is_canceled = false;
        let mut current_progress = 0.0;

        // Check if installation is canceled and return true if it is.
        let mut check_cancel = move || {
            if !is_canceled {
                if cancel.try_recv().is_ok() {
                    is_canceled = true;
                }
            }
            is_canceled
        };

        // Report installation progress to main thread though progress channel.
        // If value is -1.0, it will return current progress without updating.
        // Return current progress.
        let mut report_progress = move |value: f32| {
            if value == -1.0 {
                return current_progress;
            }
            current_progress = value;
            if progress.send(value).is_err() {
                println!("Failed to send progress.");
            } else {
                if cfg!(debug_assertions) {
                    println!("Updated progress: {}%", current_progress * 100.0);
                }
            }
            current_progress
        };

        println!("Installing...");

        // Emulate installation progress.
        for i in 0..100 {
            if check_cancel() {
                break;
            }

            report_progress(0.01 * (i + 1) as f32);
            thread::sleep(Duration::from_millis(100));
        }

        // Emulate installation cancellation progress.
        if check_cancel() {
            println!("Cancelling installation...");

            let cp = (report_progress(-1.0) * 100.0) as i32;

            for i in 0..cp {
                report_progress(0.01 * ((cp - i) as f32));
                thread::sleep(Duration::from_millis(100));
            }

            // Remember to set progress to 0.0 when installation cancellation progress is done.
            report_progress(0.0);

            println!("Installation cancelled.");

            return;
        }

        println!("Installation complete.");
    });
}

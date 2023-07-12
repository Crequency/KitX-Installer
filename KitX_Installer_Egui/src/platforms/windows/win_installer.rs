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
        // Return the value of `is_canceled`.
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
                println!("! Failed to send progress.");
            } else {
                if cfg!(debug_assertions) {
                    println!("> Updated progress: {}%", current_progress * 100.0);
                }
            }
            current_progress
        };

        // Report installation details to main thread though details channel.
        let report_detail = move |content: &str| {
            println!("{}", content);
            if details.send(content.to_string()).is_err() {
                println!("! Failed to send detail.");
            }
        };

        println!();
        report_detail("> Installing...");

        if !check_cancel() {
            // Download installation files.

            report_detail("┌ Downloading installation files ...");

            thread::sleep(Duration::from_millis(2000));

            report_detail("└ Installation files downloaded.");

            report_progress(0.50);
        }

        if !check_cancel() {
            // Extract installation files.

            report_detail("┌ Extracting installation files ...");

            thread::sleep(Duration::from_millis(500));

            report_detail("└ Installation files extracted.");

            report_progress(0.65);
        }

        if !check_cancel() {
            // Move installation files to installation path.

            report_detail("┌ Moving installation files to installation path ...");

            thread::sleep(Duration::from_millis(300));

            report_detail("└ Installation files moved to installation path.");

            report_progress(0.80);
        }

        if !check_cancel() {
            // Update access permissions of installation path.

            report_detail("┌ Updating installation path permissions ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ Installation path permissions updated.");

            report_progress(0.85);
        }

        if !check_cancel() {
            // Create desktop shortcut and start menu shortcut.

            report_detail("┌ Creating shortcuts ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ Shortcuts created.");

            report_progress(0.90);
        }

        if !check_cancel() {
            // Insert application info to registry.

            report_detail("┌ Inserting application info to registry ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ Application info inserted to registry.");

            report_progress(0.95);
        }

        if !check_cancel() {
            // Insert file association info to registry.

            report_detail("┌ Inserting file association info to registry ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ File association info inserted to registry.");

            report_progress(0.975);
        }

        if !check_cancel() {
            // Insert uninstall info to registry and create uninstaller.

            report_detail("┌ Inserting uninstall info to registry ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ Uninstall info inserted to registry.");

            report_detail("┌ Creating uninstaller program ...");

            thread::sleep(Duration::from_millis(3000));

            report_detail("└ Uninstaller program created.");

            report_progress(1.00);
        }

        // If installation is canceled, uninstall and return.
        if check_cancel() {
            uninstall(
                config_clone,
                &mut report_progress.clone(),
                report_detail.clone(),
            );
            return;
        }

        report_detail("> Installation complete.");
    });
}

fn uninstall<RP: FnMut(f32) -> f32, RD: Fn(&str)>(
    config: InstallConfig,
    report_progress: &mut RP,
    report_detail: RD,
) {
    report_detail("> Cancelling installation...");

    let installation_progress = (report_progress(-1.0) * 100.0) as i32;

    if installation_progress >= 100 {
        report_detail("! Installation had been completed. You can't cancel it now.");
        return;
    }

    // Delete related registry keys (including app info, file association info, uninstall info).
    if installation_progress >= 90 && installation_progress < 100 {
        report_detail("┌ Deleting registry keys...");

        thread::sleep(Duration::from_millis(100));

        report_detail("└ Registry keys deleted.");

        report_progress(0.85);
    }

    // Delete shortcuts.
    if installation_progress >= 85 {
        report_detail("┌ Deleting shortcuts...");

        thread::sleep(Duration::from_millis(100));

        report_detail("└ Shortcuts deleted.");

        report_progress(0.80);
    }

    // Delete installation files.
    if installation_progress >= 80 {
        report_detail("┌ Deleting installation files...");

        thread::sleep(Duration::from_millis(1000));

        report_detail("└ Installation files deleted.");

        report_progress(0.0);
    }

    // Remember to set progress to 0.0 when installation cancellation progress is done.
    report_progress(0.0);

    report_detail("> Installation cancelled.");
}

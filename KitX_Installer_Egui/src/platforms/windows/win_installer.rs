use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::data::data_fetcher;
use crate::data::download_config::DownloadConfig;
use crate::data::install_config::InstallConfig;
use crate::utils::assets_manager;
use crate::utils::zip_file_manager;

pub fn install(
    i_config: &InstallConfig,
    d_config: &DownloadConfig,
    progress_report_channel_sender: mpsc::Sender<f32>,
    details_report_channel_sender: mpsc::Sender<String>,
    cancel_command_channel_receiver: mpsc::Receiver<i32>,
) -> JoinHandle<()> {
    let mut ic_config = i_config.clone();
    let dc_config = d_config.clone();

    let handle = thread::spawn(move || {
        let mut is_canceled = false;
        let mut current_progress = 0.0;

        // Check if installation is canceled and return true if it is.
        // Return the value of `is_canceled`.
        let mut check_cancel = move || {
            if !is_canceled {
                if cancel_command_channel_receiver.try_recv().is_ok() {
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
            if progress_report_channel_sender.send(value).is_err() {
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
            if details_report_channel_sender
                .send(content.to_string())
                .is_err()
            {
                println!("! Failed to send detail.");
            }
        };

        println!();
        report_detail("> Installing...");

        // Download installation files.
        if !check_cancel() {
            report_progress(0.05);
            report_detail("┌ Downloading installation files ...");

            // Create installation path.
            report_detail(
                format!(
                    "├ Creating installation path `{}` ...",
                    ic_config.installation_path.clone()
                )
                .as_str(),
            );
            if create_dir_all(ic_config.installation_path.clone()).is_err() {
                report_detail("! Failed to create installation path, quiting ...");
                // TODO: Cancel installation.
            }
            report_progress(0.10);

            // Get download url from download config.
            let download_url = dc_config.get_full_url("github.com");
            report_detail(
                format!(
                    "├ Downloading installation files from `{}` ...",
                    download_url
                )
                .as_str(),
            );

            // Fetch installation files in binary.
            let bytes = data_fetcher::fetch_binary(download_url);
            report_detail(format!("├ Downloaded {} bytes.", bytes.len()).as_str());
            report_progress(0.40);

            // Save installation files to target file path.
            let target_file_path = format!(
                "{}/kitx-{}.7z",
                ic_config.installation_path.clone(),
                dc_config.profile.clone()
            );
            ic_config.installation_file_path = Some(target_file_path.clone());
            report_detail(
                format!(
                    "├ Saving installation files to `{}` ...",
                    target_file_path.clone()
                )
                .as_str(),
            );
            let file = File::create(target_file_path.clone());
            if file.is_err() {
                report_detail("! Failed to create installation file, quiting ...");
            }
            file.unwrap().write_all(bytes.as_slice()).unwrap();
            report_detail(format!("├ Saved to `{}`.", target_file_path.clone()).as_str());

            report_detail("└ [DONE] Installation files downloaded.");

            report_progress(0.50);
        }

        // Extract installation files.
        if !check_cancel() {
            report_detail("┌ Extracting installation files ...");

            report_detail("├ Extracting 7z file ...");

            ic_config.extraction_program_path = Some(assets_manager::release_7z(
                ic_config.installation_path.clone(),
            ));
            let v7z_file = ic_config.extraction_program_path.clone().unwrap();
            let installation_file = format!("kitx-{}.7z", dc_config.profile.clone());
            zip_file_manager::decompress_7z_with_7z(
                v7z_file,
                ic_config.installation_path.clone(),
                format!("./{}", installation_file),
                &mut report_progress.clone(),
                report_detail.clone(),
            );

            report_detail("└ [DONE] Installation files extracted.");

            report_progress(0.65);
        }

        // Clean installation files in installation path.
        if !check_cancel() {
            report_detail("┌ Clean installation files in installation path ...");

            // Sleep to await for 7z process to exit.
            thread::sleep(Duration::from_millis(3 * 1000));

            let mut all_cleaned = true;

            if Path::new(ic_config.installation_file_path.clone().unwrap().as_str()).exists() {
                if fs::remove_file(ic_config.installation_file_path.clone().unwrap()).is_err() {
                    report_detail("! Failed to remove installation file.");
                    all_cleaned = false;
                }
            }

            if Path::new(ic_config.extraction_program_path.clone().unwrap().as_str()).exists() {
                if fs::remove_file(ic_config.extraction_program_path.clone().unwrap()).is_err() {
                    report_detail("! Failed to remove extraction program.");
                    all_cleaned = false;
                }
            }

            if all_cleaned {
                report_detail("└ [DONE] Installation files cleaned in installation path.");
            } else {
                report_detail(
                    "└ [FAIL] Failed to clean all installation files in installation path.",
                );
            }

            report_progress(0.80);
        }

        // Update access permissions of installation path.
        if !check_cancel() {
            report_detail("┌ Updating installation path permissions ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ [DONE] Installation path permissions updated.");

            report_progress(0.85);
        }

        // Create desktop shortcut and start menu shortcut.
        if !check_cancel() {
            report_detail("┌ Creating shortcuts ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ [DONE] Shortcuts created.");

            report_progress(0.90);
        }

        // Insert application info to registry.
        if !check_cancel() {
            report_detail("┌ Inserting application info to registry ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ [DONE] Application info inserted to registry.");

            report_progress(0.95);
        }

        // Insert file association info to registry.
        if !check_cancel() {
            report_detail("┌ Inserting file association info to registry ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ [DONE] File association info inserted to registry.");

            report_progress(0.975);
        }

        // Insert uninstall info to registry and create uninstaller.
        if !check_cancel() {
            report_detail("┌ Inserting uninstall info to registry ...");

            thread::sleep(Duration::from_millis(100));

            report_detail("└ [DONE] Uninstall info inserted to registry.");

            report_detail("┌ Creating uninstaller program ...");

            thread::sleep(Duration::from_millis(3000));

            report_detail("└ [DONE] Uninstaller program created.");

            report_progress(1.00);
        }

        // If installation is canceled, uninstall and return.
        if check_cancel() {
            cancel_installation(
                ic_config,
                &mut report_progress.clone(),
                report_detail.clone(),
            );
            return;
        }

        report_detail("> Installation complete.");
    });

    handle
}

fn cancel_installation<RP: FnMut(f32) -> f32, RD: Fn(&str)>(
    _config: InstallConfig,
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

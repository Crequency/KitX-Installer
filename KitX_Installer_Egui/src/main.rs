#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app_info;
mod data;
mod platforms;
mod utils;
mod views;

extern crate msgbox;

use {
    crate::{
        app_info::{AppInfo, RunMode},
        utils::arguments_processor,
        views::{application, font_helper},
    },
    arguments::Arguments,
    platforms::windows::win_uninstaller,
    std::env,
};

// Entry point of this program.
fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let src_args = env::args(); // Get arguments from command line.
    let args: Arguments = arguments::parse(src_args).unwrap(); // Parse arguments.

    let app_info = arguments_processor::args_to_app_info(args); // Process arguments and return `AppInfo` if success.

    // If failed to process arguments, exit.
    if app_info.is_none() {
        println!("Failed to process arguments.");
        eprintln!("Failed to process arguments.");
        return;
    }

    let app_info = app_info.unwrap();

    // Run in GUI or CLI mode according to `app_info.run_mode`.
    match &app_info.run_mode.as_ref().unwrap() {
        RunMode::Gui => {
            if run_gui(app_info).is_err() {
                println!("Failed to run GUI.");
                eprintln!("Failed to run GUI.");
            }
        }
        RunMode::Cli => {
            run_cli(app_info);
        }
        RunMode::SlientUninstall => {
            if cfg!(target_os = "windows") {
                win_uninstaller::uninstall()
                    .join()
                    .expect("Failed to uninstall.");
            }
            // TODO: Implement uninstaller for other platforms.
        }
    }
}

// Run in GUI mode.
fn run_gui(app_info: AppInfo) -> Result<(), eframe::Error> {
    println!("KitX Installer GUI");
    println!("Version: v{}", app_info.version);
    println!();

    let options = application::get_native_options(None);

    let result = eframe::run_native(
        "KitX Installer",
        options,
        Box::new(|cc| {
            font_helper::set_default_font(cc);

            Box::<application::AppData>::default()
        }),
    );

    println!();
    println!("GUI exited.");
    println!("KitX Installer GUI exited.");

    result
}

// Run in CLI mode.
fn run_cli(app_info: AppInfo) {
    println!("KitX Installer CLI");
    println!("Version: v{}", app_info.version);
    println!();

    println!();
    println!("CLI exited.");
    println!("KitX Installer CLI exited.");
}

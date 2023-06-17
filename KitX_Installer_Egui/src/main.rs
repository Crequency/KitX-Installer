#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod data;
mod platforms;

mod app_info;

use arguments::Arguments;
use std::env;

use crate::app_info::{RunMode, AppInfo};
use crate::platforms::windows::application;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let src_args = env::args();
    let args: Arguments = arguments::parse(src_args).unwrap();

    let app_info = process_args(args);

    if app_info.is_none() {
        return;
    }

    let app_info = app_info.unwrap();

    match &app_info.run_mode.as_ref().unwrap() {
        RunMode::Gui => {
            if run_gui(app_info).is_err() {
                eprintln!("Failed to run GUI.");
                println!("Failed to run GUI.");
            }
        }
        RunMode::Cli => {
            run_cli(app_info);
        }
    }
}

fn process_args(args: Arguments) -> Option<AppInfo> {
    // `clia` means `command line interface argument`
    let mut clia_run_gui = args.get::<bool>("run-gui");
    let mut clia_run_cli = args.get::<bool>("run-cli");

    // when both `run-gui` and `run-cli` are not specified, auto detect.
    if clia_run_gui.is_none() && clia_run_cli.is_none() {
        clia_run_gui = Some(cfg!(target_os = "windows"));
        clia_run_cli = Some(!clia_run_gui.unwrap());
    }

    // when both `run-gui` and `run-cli` are specified to true, error and exit.
    if clia_run_gui.unwrap_or(false) && clia_run_cli.unwrap_or(false) {
        eprintln!("Cannot run GUI and CLI at the same time.");
        println!("Cannot run GUI and CLI at the same time.");
        return None;
    }

    if clia_run_gui.unwrap_or(false) || clia_run_cli.is_none() {
        clia_run_cli = Some(!clia_run_gui.unwrap_or(false));
    }

    if clia_run_cli.unwrap_or(false) || clia_run_gui.is_none() {
        clia_run_gui = Some(!clia_run_cli.unwrap_or(false));
    }

    let clia_run_gui = clia_run_gui.unwrap_or(false);
    let clia_run_cli = clia_run_cli.unwrap_or(false);

    let run_in_gui = (clia_run_gui || cfg!(target_os = "windows")) && !clia_run_cli;

    return Some(AppInfo {
        run_mode: if run_in_gui {
            Some(RunMode::Gui)
        } else {
            Some(RunMode::Cli)
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
    });
}

fn run_gui(app_info: AppInfo) -> Result<(), eframe::Error> {
    let options = application::get_native_options(None);

    eframe::run_native(
        "KitX Installer",
        options,
        Box::new(|_cc| Box::<application::AppData>::default()),
    )
}

fn run_cli(app_info: AppInfo) {
    println!("KitX Installer CLI");
    println!("v{}", app_info.version);
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod data;
mod platforms;
mod utils;
mod views;

use std::env;

use arguments::Arguments;
use eframe::egui;

mod app_info;

use crate::app_info::{AppInfo, RunMode};
use crate::views::application;

// Entry point of this program.
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

// Process arguments and return `AppInfo` if success.
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
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "SrcHei".to_string(),
                egui::FontData::from_owned(include_bytes!("../assets/fonts/SrcHei.ttf").to_vec()),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "SrcHei".to_string());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("SrcHei".to_string());
            cc.egui_ctx.set_fonts(fonts);

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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod platforms;
use crate::platforms::windows::application;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    if cfg!(target_os = "windows") {
        if run_gui().is_err() {
            eprintln!("Failed to run GUI");
        }
    }
}

fn run_gui() -> Result<(), eframe::Error> {
    let options = application::get_native_options(None);

    eframe::run_native(
        "KitX Installer",
        options,
        Box::new(|_cc| Box::<application::AppData>::default()),
    )
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod application;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = application::get_native_options(None);

    eframe::run_native(
        "KitX Installer",
        options,
        Box::new(|_cc| Box::<application::AppData>::default()),
    )
}

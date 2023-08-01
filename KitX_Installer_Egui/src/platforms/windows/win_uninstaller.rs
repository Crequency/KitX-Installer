use std::fs;

use super::reg_helper::{self, delete_program_registry};

pub fn uninstall() {
    let path = reg_helper::fetch_program_installation_path();

    if path.is_some() {
        if delete_program_registry().is_err() {
            panic!("Failed to delete program registry.");
        }
        if delete_installation_files(path.unwrap()).is_err() {
            panic!("Failed to delete installation files.");
        }
        delete_shortcuts();
    } else {
        panic!("Failed to fetch installation path.");
    }
}

pub fn delete_shortcuts() {
    let desktop_shortcut = format!(
        "{}\\KitX Dashboard.lnk",
        reg_helper::get_desktop_path().unwrap()
    );

    let start_menu_shortcut = format!(
        "{}\\KitX Dashboard.lnk",
        reg_helper::get_start_menu_path().unwrap()
    );

    if fs::remove_file(desktop_shortcut).is_err() {
        eprintln!("Failed to delete desktop shortcut.")
    }

    if fs::remove_file(start_menu_shortcut).is_err() {
        eprintln!("Failed to delete start menu shortcut.")
    }
}

pub fn delete_installation_files(path: String) -> Result<(), std::io::Error> {
    let status = fs::remove_dir_all(path);

    if status.is_ok() {
        Ok(())
    } else {
        eprintln!("Failed to delete all installation files.");
        Err(status.err().unwrap())
    }
}

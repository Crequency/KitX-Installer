use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub fn get_desktop_path() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let shell_folders = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders")
        .ok()?;
    let desktop_value: String = shell_folders.get_value("Desktop").ok()?;
    Some(desktop_value)
    // Some(PathBuf::from(desktop_value))
}

pub fn get_start_menu_path() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let shell_folders = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders")
        .ok()?;
    let start_menu_value: String = shell_folders.get_value("Start Menu").ok()?;
    Some(start_menu_value)
    // Some(PathBuf::from(start_menu_value))
}

use std::error::Error;
#[cfg(windows)]
use winreg::enums::*;

#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
pub fn get_desktop_path() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let shell_folders = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders")
        .ok()?;
    let desktop_value: String = shell_folders.get_value("Desktop").ok()?;
    Some(desktop_value)
    // Some(PathBuf::from(desktop_value))
}

#[cfg(not(windows))]
pub fn get_desktop_path() -> Option<String> {
    None
}

#[cfg(windows)]
pub fn get_start_menu_path() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let shell_folders = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders")
        .ok()?;
    let start_menu_value: String = shell_folders.get_value("Start Menu").ok()?;
    Some(start_menu_value)
    // Some(PathBuf::from(start_menu_value))
}

#[cfg(not(windows))]
pub fn get_start_menu_path() -> Option<String> {
    None
}

#[cfg(windows)]
pub fn update_program_registry(
    exe_path: String,
    dll_path: String,
    dir_path: String,
) -> Result<(), Box<dyn Error>> {
    // Registry Tree We Need to Add
    // + HKEY_CLASSES_ROOT
    //   + .kxp
    //     - (Default) = KitX.ExtensionsPackage
    //     - Content Type = application/kitx-extensions-package
    //   + KitX.ExtensionsPackage
    //     - (Default) = KitX Extensions Package
    //     + DefaultIcon
    //       - (Default) = {dir_path}\Assets\kxp.ico
    //     + Shell
    //       + Open
    //         - FriendlyAppName = KitX
    //         + Command
    //           - (Default) = "{exe_path}" --import-plugin "%1"
    // + HKEY_LOCAL_MACHINE
    //   + SOFTWARE
    //     + Microsoft
    //       + Windows
    //         + CurrentVersion
    //           + App Paths
    //             + KitX.Dashboard.exe
    //               - (Default) = {exe_path}
    //               - Path = {dir_path}
    //           + Uninstall
    //             + KitX
    //               - DisplayName = KitX Dashboard
    //               - DisplayVersion = {version}
    //               - DisplayIcon = {exe_path}
    //               - Publisher = Crequency Studio
    //               - InstallLocation = {dir_path}
    //               - UninstallString = {uninstall_string}
    //               - QuietUninstallString = {uninstall_string} --silent
    //               - HelpLink = https://kitx.apps.catrol.cn/help
    //               - URLInfoAbout = https://kitx.apps.catrol.cn/
    //               - NoModify = 1
    //               - NoRepair = 1
    //               - EstimatedSize = sizeof({dir_path})

    // Define Basic Registry Keys
    let software_key =
        RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags("SOFTWARE", KEY_SET_VALUE)?;
    let microsoft_key = software_key.open_subkey_with_flags("Microsoft", KEY_SET_VALUE)?;
    let windows_key = microsoft_key.open_subkey_with_flags("Windows", KEY_SET_VALUE)?;
    let current_version_key =
        windows_key.open_subkey_with_flags("CurrentVersion", KEY_SET_VALUE)?;
    let filecon_key = RegKey::predef(HKEY_CLASSES_ROOT).create_subkey(".kxp")?;
    let filepro_key = RegKey::predef(HKEY_CLASSES_ROOT).create_subkey("KitX.ExtensionsPackage")?;

    // Update App Paths
    {
        let app_paths_key =
            current_version_key.open_subkey_with_flags("App Paths", KEY_SET_VALUE)?;
        let kitx_key = app_paths_key.create_subkey("KitX.Dashboard.exe")?;
        kitx_key.0.set_value("", &exe_path)?;
        kitx_key.0.set_value("Path", &dir_path)?;
    }

    // Update Uninstall
    {
        let cmd_fetch_program_version =
            format!("(Get-Item -path \"{}\").VersionInfo.FileVersion", dll_path);
        let version_output = std::process::Command::new("powershell")
            .arg("-ExecutionPolicy")
            .arg("Bypass") // 忽略脚本执行策略（仅在需要时使用）
            .arg("-Command")
            .arg(cmd_fetch_program_version)
            .output()
            .expect("Failed to run powershell script in order to fetch program version.");
        let version = String::from_utf8(version_output.stdout)?;
        let uninstall_string = format!("{}\\Installer.exe --uninstall", dir_path.clone());

        let uninstall_key =
            current_version_key.open_subkey_with_flags("Uninstall", KEY_SET_VALUE)?;
        let kitx_uninstall_key = uninstall_key.create_subkey("KitX")?;
        kitx_uninstall_key
            .0
            .set_value("DisplayName", &"KitX Dashboard")?;
        kitx_uninstall_key
            .0
            .set_value("DisplayVersion", &version.to_string())?;
        kitx_uninstall_key.0.set_value("DisplayIcon", &exe_path)?;
        kitx_uninstall_key
            .0
            .set_value("Publisher", &"Crequency Studio")?;
        kitx_uninstall_key
            .0
            .set_value("InstallLocation", &dir_path)?;
        kitx_uninstall_key
            .0
            .set_value("UninstallString", &uninstall_string)?;
        kitx_uninstall_key.0.set_value(
            "QuietUninstallString",
            &format!("{} --silent", uninstall_string),
        )?;
        kitx_uninstall_key
            .0
            .set_value("HelpLink", &"https://kitx.apps.catrol.cn/help")?;
        kitx_uninstall_key
            .0
            .set_value("URLInfoAbout", &"https://kitx.apps.catrol.cn/")?;
        kitx_uninstall_key.0.set_value("NoModify", &1u32)?;
        kitx_uninstall_key.0.set_value("NoRepair", &1u32)?;
        // kitx_uninstall_key.0.set_value("EstimatedSize", &version)?;
    }

    // Update file association
    {
        filecon_key.0.set_value("", &"KitX.ExtensionsPackage")?;
        filecon_key
            .0
            .set_value("Content Type", &"application/kitx-extensions-package")?;

        filepro_key.0.set_value("", &"KitX Extensions Package")?;

        let icon_key = filepro_key.0.create_subkey("DefaultIcon")?;

        icon_key
            .0
            .set_value("", &format!("{}\\Assets\\kxp.ico", dir_path))?;

        let shell_key = filepro_key.0.create_subkey("Shell")?;
        let open_key = shell_key.0.create_subkey("Open")?;
        let com_key = open_key.0.create_subkey("Command")?;

        open_key.0.set_value("FriendlyAppName", &"KitX")?;

        com_key
            .0
            .set_value("", &format!("\"{}\" --import-plugin \"%1\"", exe_path))?;
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn update_program_registry() -> Result<(), Box<dyn Error>> {
    Err(())
}

#[cfg(windows)]
pub fn delete_program_registry() -> Result<(), Box<dyn Error>> {
    // Registry Tree Nodes We Need to Delete
    // + HKEY_CLASSES_ROOT
    //   - .kxp
    //   - KitX.ExtensionsPackage
    // + HKEY_LOCAL_MACHINE
    //   + SOFTWARE
    //     + Microsoft
    //       + Windows
    //         + CurrentVersion
    //           + App Paths
    //             - KitX.Dashboard.exe
    //           + Uninstall
    //             - KitX

    // Define Basic Registry Keys
    let software_key =
        RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags("SOFTWARE", KEY_SET_VALUE)?;
    let microsoft_key = software_key.open_subkey_with_flags("Microsoft", KEY_SET_VALUE)?;
    let windows_key = microsoft_key.open_subkey_with_flags("Windows", KEY_SET_VALUE)?;
    let current_version_key =
        windows_key.open_subkey_with_flags("CurrentVersion", KEY_SET_VALUE)?;
    let classes_root = RegKey::predef(HKEY_CLASSES_ROOT);

    // Delete App Paths
    {
        let app_paths_key =
            current_version_key.open_subkey_with_flags("App Paths", KEY_SET_VALUE)?;
        app_paths_key.delete_subkey("KitX.Dashboard.exe")?;
    }

    // Delete Uninstall
    {
        let uninstall_key =
            current_version_key.open_subkey_with_flags("Uninstall", KEY_SET_VALUE)?;
        uninstall_key.delete_subkey("KitX")?;
    }

    // Delete file association
    {
        classes_root.delete_subkey(".kxp")?;
        classes_root.delete_subkey("KitX.ExtensionsPackage")?;
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn delete_program_registry() -> Result<(), Box<dyn Error>> {
    Err(())
}

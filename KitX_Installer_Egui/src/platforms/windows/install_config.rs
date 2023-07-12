use super::reg_helper;

pub struct WindowsInstallConfig {
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub desktop_path: Option<String>,
    pub start_menu_path: Option<String>,
}

impl WindowsInstallConfig {
    pub fn default() -> WindowsInstallConfig {
        WindowsInstallConfig {
            create_desktop_shortcut: false,
            create_start_menu_shortcut: true,
            desktop_path: None,
            start_menu_path: None,
        }
    }

    pub fn init(&mut self) {
        if self.desktop_path == None {
            self.desktop_path = Some(reg_helper::get_desktop_path().unwrap());
            println!("# Desktop directory: {:?}", self.desktop_path);
        }

        if self.start_menu_path == None {
            self.start_menu_path = Some(reg_helper::get_start_menu_path().unwrap());
            println!("# Start menu directory: {:?}", self.start_menu_path);
        }
    }
}

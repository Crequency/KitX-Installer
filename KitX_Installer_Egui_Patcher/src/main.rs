mod app_info;

use std::env;

use crate::app_info::AppInfo;

fn main() {
    let args = arguments::parse(env::args()).unwrap();

    let mut app_info = AppInfo::default();

    app_info.patch_file_path = args.get::<String>("patch");

    println!("KitX Installer Patcher");
    println!("Version: v{}", app_info.version);

    if app_info.patch_file_path.is_none() {
        println!("Please provide file to patch path !");
        return;
    }

    println!("File to patch: {:?}", app_info.patch_file_path);


}

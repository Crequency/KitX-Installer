use std::{fs::File, io::Write};

pub fn release_7z(release_path: String) -> String {
    let mut v7z_file = String::new();
    let mut bytes: &[u8] = &[];

    if cfg!(target_os = "windows") {
        v7z_file += "7zr.exe";
        bytes = include_bytes!("../../assets/7z/7zr.exe");
    } else if cfg!(target_os = "linux") {
        v7z_file += "7zzs-linux";
        if cfg!(target_arch = "x86") {
            v7z_file += "-x86";
            bytes = include_bytes!("../../assets/7z/7zzs-linux-x86");
        } else if cfg!(target_arch = "x86_64") {
            v7z_file += "-x64";
            bytes = include_bytes!("../../assets/7z/7zzs-linux-x64");
        } else if cfg!(target_arch = "arm") {
            v7z_file += "-arm";
            bytes = include_bytes!("../../assets/7z/7zzs-linux-arm");
        } else if cfg!(target_arch = "aarch64") {
            v7z_file += "-arm64";
            bytes = include_bytes!("../../assets/7z/7zzs-linux-arm64");
        }
    } else if cfg!(target_os = "macos") {
        v7z_file += "7zz-mac";
        bytes = include_bytes!("../../assets/7z/7zz-mac");
    }

    let target_path = format!("{}{}", release_path, v7z_file);
    let mut file = File::create(target_path.clone()).unwrap();
    file.write_all(&u8_to_vec(bytes).as_slice()).unwrap();

    target_path
}

fn u8_to_vec(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for byte in bytes {
        vec.push(*byte);
    }
    vec
}

pub enum IconScale {
    X16,
    X32,
    X64,
    X128,
    X256,
}

fn get_icons(scale: IconScale) -> &'static [u8] {
    let bytes: &[u8];

    match scale {
        IconScale::X16 => {
            bytes = include_bytes!("../../assets/icons/icon-16x.ico");
        }
        IconScale::X32 => {
            bytes = include_bytes!("../../assets/icons/icon-32x.ico");
        }
        IconScale::X64 => {
            bytes = include_bytes!("../../assets/icons/icon-64x.ico");
        }
        IconScale::X128 => {
            bytes = include_bytes!("../../assets/icons/icon-128x.ico");
        }
        IconScale::X256 => {
            bytes = include_bytes!("../../assets/icons/icon-256x.ico");
        }
    }

    bytes
}

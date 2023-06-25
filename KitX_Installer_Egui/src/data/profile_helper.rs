pub fn get_profile() -> String {
    let mut profile = String::new();

    if cfg!(target_os = "windows") {
        profile += "win";
    } else if cfg!(target_os = "linux") {
        profile += "linux";
    } else if cfg!(target_os = "macos") {
        profile += "osx";
    }

    profile += "-";

    if cfg!(target_arch = "x86") {
        profile += "x86";
    } else if cfg!(target_arch = "x86_64") {
        profile += "x64";
    } else if cfg!(target_arch = "arm") {
        profile += "arm";
    } else if cfg!(target_arch = "aarch64") {
        profile += "arm64";
    }

    profile += "-";

    profile += "single";

    profile
}

﻿use super::{models::host::Host, profile_helper::get_profile};

pub fn get_default_version_str() -> String {
    "$$_!_%Version%_@_$$                                        #".to_string()
}

pub fn get_default_profile_str() -> String {
    "$$_!_%Profile%_@_$$                                        #".to_string()
}

pub struct DownloadConfig {
    pub version: String,
    pub profile: String,
    pub hosts: Vec<Host>,
}

impl DownloadConfig {
    pub fn default() -> DownloadConfig {
        DownloadConfig {
            version: get_default_version_str(),
            profile: get_default_profile_str(),
            hosts: vec![
                Host{
                    host_key: "github".to_string(),
                    host_url: "https://github.com/Crequency/KitX/releases/download/<version>/kitx-<profile>.7z".to_string(),
                    host_latest_url: "https://github.com/Crequency/KitX/releases/latest/download/kitx-<profile>.7z".to_string(),
                    host_descr: "GitHub Releases - Crequency/KitX".to_string(),
                },
            ],
        }
    }

    pub fn version_patched(&self) -> bool {
        self.version != get_default_version_str()
    }

    pub fn profile_patched(&self) -> bool {
        self.profile != get_default_profile_str()
    }

    pub fn get_full_url(&self, host_key: &str) -> String {
        let mut host_url = String::new();
        let mut host_latest_url = String::new();

        // Find the host by `host_key`
        for host in &self.hosts {
            if host.host_key == host_key {
                host_url = host.host_url.clone();
                host_latest_url = host.host_latest_url.clone();
                break;
            }
        }

        // If version patched, use patched version, else, use latest version
        if self.version_patched() {
            host_url = host_url.replace("<version>", &self.version);
        } else {
            host_url = host_latest_url;
        }

        // If profile patched, use patched profile, else, use auto detected profile
        if self.profile_patched() {
            host_url = host_url.replace("<profile>", &self.profile);
        } else {
            host_url = host_url.replace("<profile>", &get_profile());
        }

        host_url
    }
}

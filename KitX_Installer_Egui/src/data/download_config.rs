pub struct DownloadConfig {
    pub version: String,
    pub profile: String,
}

impl DownloadConfig {
    pub fn default() -> DownloadConfig {
        DownloadConfig {
            version: "$$_!_%Version%_@_$$                                        #".to_string(),
            profile: "$$_!_%Profile%_@_$$                                        #".to_string(),
        }
    }
}
